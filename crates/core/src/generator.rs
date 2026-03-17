use std::collections::HashSet;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

use crate::bin_database::BinDatabase;
use crate::card::*;
use crate::crypto::SecureRng;
use crate::error::{CcGenError, Result};
use crate::formatter;
use crate::pattern::{BinPattern, PatternSegment};
use crate::validator;

const MAX_QUANTITY: u32 = 100_000;

/// Main card generator engine
pub struct CardGenerator {
    rng: SecureRng,
    db: BinDatabase,
}

impl CardGenerator {
    pub fn new() -> Self {
        Self {
            rng: SecureRng::new(),
            db: BinDatabase::new(),
        }
    }

    /// Generate cards based on a request
    pub fn generate(&mut self, request: &GenerateRequest) -> Result<GenerateResult> {
        if request.count == 0 || request.count > MAX_QUANTITY {
            return Err(CcGenError::InvalidQuantity(MAX_QUANTITY));
        }

        let pattern = BinPattern::parse(&request.bin_pattern)?;
        let brand = self.detect_brand_for_pattern(&pattern);
        let target_length = request.card_length.unwrap_or_else(|| {
            brand.map_or(16, |b| b.default_length())
        });

        #[cfg(not(target_arch = "wasm32"))]
        let start = Instant::now();
        let mut cards = Vec::with_capacity(request.count as usize);
        let mut seen = if request.unique {
            HashSet::with_capacity(request.count as usize)
        } else {
            HashSet::new()
        };

        let mut attempts: u32 = 0;
        let max_attempts = request.count * 10 + 1000;

        while cards.len() < request.count as usize && attempts < max_attempts {
            attempts += 1;

            let number = self.generate_number(&pattern, target_length);

            if request.unique && !seen.insert(number.clone()) {
                continue;
            }

            let detected_brand = brand.unwrap_or_else(|| {
                validator::detect_brand_from_number(&number).unwrap_or(CardBrand::Visa)
            });

            let card_type = self.db.lookup(&number)
                .map(|info| info.card_type)
                .unwrap_or(CardType::Credit);

            let (month, year) = if request.include_expiry {
                self.generate_expiry(request.min_expiry_years, request.max_expiry_years)
            } else {
                ("00".to_string(), "0000".to_string())
            };

            let cvv = if request.include_cvv {
                self.generate_cvv(detected_brand.cvv_length())
            } else {
                String::new()
            };

            let number_formatted = GeneratedCard::format_number(&number, &detected_brand);

            let issuer = self.db.lookup(&number).map(|i| i.issuer_name);
            let country = self.db.lookup(&number).and_then(|i| i.country_code);

            cards.push(GeneratedCard {
                number,
                number_formatted,
                brand: detected_brand,
                card_type,
                expiration_month: month,
                expiration_year: year,
                cvv,
                issuer,
                country,
                luhn_valid: true, // We always generate Luhn-valid numbers
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        let elapsed_us = {
            let elapsed = start.elapsed();
            elapsed.as_micros() as u64
        };
        #[cfg(target_arch = "wasm32")]
        let elapsed_us = 0u64;

        let formatted_output = formatter::format_cards(&cards, request.format);

        Ok(GenerateResult {
            stats: GenerationStats {
                total_generated: cards.len() as u32,
                valid_count: cards.len() as u32,
                generation_time_us: elapsed_us,
                unique_count: cards.len() as u32,
            },
            cards,
            formatted_output,
        })
    }

    /// Generate a Luhn-valid card number from a pattern
    fn generate_number(&mut self, pattern: &BinPattern, target_length: u8) -> String {
        let tlen = target_length as usize;
        let mut digits: Vec<u8> = Vec::with_capacity(tlen);

        // Fill from pattern segments (all except the last position, reserved for check digit)
        for seg in pattern.segments.iter() {
            if digits.len() >= tlen - 1 {
                break;
            }
            let digit = match seg {
                PatternSegment::Fixed(d) => *d,
                PatternSegment::Random => self.rng.digit(),
                PatternSegment::Range(lo, hi) => self.rng.digit_range(*lo, *hi),
                PatternSegment::OneOf(opts) => self.rng.pick_from(opts),
                PatternSegment::NonZero => self.rng.nonzero_digit(),
            };
            digits.push(digit);
        }

        // Fill remaining positions (except check digit) with random digits
        while digits.len() < tlen - 1 {
            digits.push(self.rng.digit());
        }

        // Calculate and append Luhn check digit
        let check = validator::calculate_check_digit(&digits);
        digits.push(check);

        digits.iter().map(|d| (d + b'0') as char).collect()
    }

    /// Detect card brand from pattern's fixed prefix
    fn detect_brand_for_pattern(&self, pattern: &BinPattern) -> Option<CardBrand> {
        let prefix = pattern.fixed_prefix();
        if prefix.is_empty() {
            return None;
        }
        validator::detect_brand_from_number(&prefix)
    }

    /// Generate a realistic future expiration date
    fn generate_expiry(&mut self, min_years: u32, max_years: u32) -> (String, String) {
        // Get current date (approximate without full chrono in WASM)
        // We use a simple approach: current year 2026, month 3
        let current_year = 2026u32;
        let current_month = 3u32;

        let years_ahead = self.rng.range_u32(min_years, max_years + 1);
        let month = self.rng.range_u32(1, 13);
        let mut year = current_year + years_ahead;

        // If the generated month/year is in the past, bump the year
        if year == current_year && month <= current_month {
            year += 1;
        }

        (format!("{:02}", month), format!("{}", year))
    }

    /// Generate a CVV/CVC/CID
    fn generate_cvv(&mut self, length: u8) -> String {
        (0..length)
            .map(|_| (self.rng.digit() + b'0') as char)
            .collect()
    }

    /// Get the BIN database reference
    pub fn database(&self) -> &BinDatabase {
        &self.db
    }

    /// Validate a card number
    pub fn validate(&self, card_number: &str) -> ValidationResult {
        validator::triple_verify(card_number, &self.db)
    }
}

impl Default for CardGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_visa() {
        let mut cgen = CardGenerator::new();
        let req = GenerateRequest {
            bin_pattern: "4xxxxxxxxxxxxx".to_string(),
            count: 100,
            ..Default::default()
        };
        let result = cgen.generate(&req).unwrap();
        assert_eq!(result.cards.len(), 100);
        for card in &result.cards {
            assert!(validator::validate_luhn(&card.number), "Card {} failed Luhn", card.number);
            assert!(card.number.starts_with('4'));
        }
    }

    #[test]
    fn test_generate_mastercard() {
        let mut cgen = CardGenerator::new();
        let req = GenerateRequest {
            bin_pattern: "51xxxxxxxxxxxx".to_string(),
            count: 50,
            ..Default::default()
        };
        let result = cgen.generate(&req).unwrap();
        assert_eq!(result.cards.len(), 50);
        for card in &result.cards {
            assert!(validator::validate_luhn(&card.number));
            assert!(card.number.starts_with("51"));
            assert_eq!(card.number.len(), 16);
        }
    }

    #[test]
    fn test_generate_amex() {
        let mut cgen = CardGenerator::new();
        let req = GenerateRequest {
            bin_pattern: "37xxxxxxxxxxx".to_string(),
            count: 50,
            card_length: Some(15),
            ..Default::default()
        };
        let result = cgen.generate(&req).unwrap();
        assert_eq!(result.cards.len(), 50);
        for card in &result.cards {
            assert!(validator::validate_luhn(&card.number));
            assert!(card.number.starts_with("37"));
            assert_eq!(card.number.len(), 15);
            assert_eq!(card.cvv.len(), 4);
        }
    }

    #[test]
    fn test_all_unique() {
        let mut cgen = CardGenerator::new();
        let req = GenerateRequest {
            bin_pattern: "4111xxxxxxxxxxxx".to_string(),
            count: 1000,
            unique: true,
            ..Default::default()
        };
        let result = cgen.generate(&req).unwrap();
        let set: HashSet<String> = result.cards.iter().map(|c| c.number.clone()).collect();
        assert_eq!(set.len(), 1000);
    }

    #[test]
    fn test_100_percent_luhn_valid() {
        let mut cgen = CardGenerator::new();
        let req = GenerateRequest {
            bin_pattern: "4xxxxxxxxxxxxx".to_string(),
            count: 10000,
            include_expiry: false,
            include_cvv: false,
            unique: false,
            ..Default::default()
        };
        let result = cgen.generate(&req).unwrap();
        for card in &result.cards {
            assert!(
                validator::validate_luhn(&card.number),
                "Card {} failed Luhn validation",
                card.number
            );
        }
    }

    #[test]
    fn test_zero_quantity_fails() {
        let mut cgen = CardGenerator::new();
        let req = GenerateRequest {
            count: 0,
            ..Default::default()
        };
        assert!(cgen.generate(&req).is_err());
    }
}
