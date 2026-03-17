use crate::bin_database::BinDatabase;
use crate::card::{CardBrand, ValidationResult};

/// Validate a card number using the Luhn algorithm (ISO/IEC 7812-1)
pub fn validate_luhn(number: &str) -> bool {
    let digits: Vec<u8> = number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c as u8 - b'0')
        .collect();

    if digits.len() < 2 {
        return false;
    }

    luhn_check(&digits)
}

/// Core Luhn check on a slice of digits
fn luhn_check(digits: &[u8]) -> bool {
    let mut sum: u32 = 0;

    for (i, &digit) in digits.iter().rev().enumerate() {
        let d = digit as u32;
        if i % 2 == 1 {
            let doubled = d * 2;
            sum += if doubled > 9 { doubled - 9 } else { doubled };
        } else {
            sum += d;
        }
    }

    sum % 10 == 0
}

/// Calculate the Luhn check digit for a partial card number
pub fn calculate_check_digit(partial_digits: &[u8]) -> u8 {
    let mut sum: u32 = 0;
    // When calculating check digit, the check digit position (rightmost) is even index from right (0)
    // So we double starting from the rightmost digit of the partial number
    for (i, &digit) in partial_digits.iter().rev().enumerate() {
        let d = digit as u32;
        if i % 2 == 0 {
            let doubled = d * 2;
            sum += if doubled > 9 { doubled - 9 } else { doubled };
        } else {
            sum += d;
        }
    }

    ((10 - (sum % 10)) % 10) as u8
}

/// Detect card brand from number prefix using BIN ranges
pub fn detect_brand_from_number(number: &str) -> Option<CardBrand> {
    let digits: String = number.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return None;
    }

    // Quick prefix detection without full BIN database
    let first = digits.as_bytes()[0] - b'0';
    let prefix2: u32 = if digits.len() >= 2 {
        digits[..2].parse().unwrap_or(0)
    } else {
        (first as u32) * 10
    };
    let prefix4: u32 = if digits.len() >= 4 {
        digits[..4].parse().unwrap_or(0)
    } else {
        0
    };
    let prefix6: u64 = if digits.len() >= 6 {
        digits[..6].parse().unwrap_or(0)
    } else {
        0
    };

    // Order matters: check most specific ranges first
    // Amex: 34, 37
    if prefix2 == 34 || prefix2 == 37 {
        return Some(CardBrand::Amex);
    }

    // Dankort: 5019
    if prefix4 == 5019 {
        return Some(CardBrand::Dankort);
    }

    // Verve: 506099-506198, 650002-650027
    if digits.len() >= 6 {
        if (506099..=506198).contains(&prefix6) || (650002..=650027).contains(&prefix6) {
            return Some(CardBrand::Verve);
        }
    }

    // Mir: 2200-2204
    if prefix4 >= 2200 && prefix4 <= 2204 {
        return Some(CardBrand::Mir);
    }

    // MasterCard: 2221-2720 or 51-55
    if (prefix4 >= 2221 && prefix4 <= 2720) || (prefix2 >= 51 && prefix2 <= 55) {
        return Some(CardBrand::MasterCard);
    }

    // JCB: 3528-3589
    if prefix4 >= 3528 && prefix4 <= 3589 {
        return Some(CardBrand::JCB);
    }

    // Diners Club: 300-305, 3095, 36, 38-39
    if (prefix4 >= 3000 && prefix4 <= 3059) || prefix4 == 3095 || prefix2 == 36 || (prefix2 >= 38 && prefix2 <= 39)
    {
        return Some(CardBrand::DinersClub);
    }

    // Visa: 4
    if first == 4 {
        return Some(CardBrand::Visa);
    }

    // InterPayment: 636
    if digits.len() >= 3 && &digits[..3] == "636" {
        return Some(CardBrand::InterPayment);
    }

    // Maestro
    if digits.len() >= 4 {
        let maestro_prefixes = ["5018", "5020", "5038", "5893", "6304", "6759", "6761", "6762", "6763"];
        for mp in &maestro_prefixes {
            if digits.starts_with(mp) {
                return Some(CardBrand::Maestro);
            }
        }
    }

    // Discover: 6011, 622126-622925, 644-649, 65
    if prefix4 == 6011 || (prefix6 >= 622126 && prefix6 <= 622925) || (prefix4 >= 6440 && prefix4 <= 6499) || prefix2 == 65 {
        return Some(CardBrand::Discover);
    }

    // UnionPay: 62
    if prefix2 == 62 {
        return Some(CardBrand::UnionPay);
    }

    // RuPay: 60, 65, 81, 82, 508
    if prefix2 == 60 || prefix2 == 81 || prefix2 == 82 {
        return Some(CardBrand::RuPay);
    }
    if digits.len() >= 3 && &digits[..3] == "508" {
        return Some(CardBrand::RuPay);
    }

    // UATP: 1
    if first == 1 {
        return Some(CardBrand::UATP);
    }

    None
}

/// Full triple verification pipeline
pub fn triple_verify(card_number: &str, db: &BinDatabase) -> ValidationResult {
    let digits: Vec<u8> = card_number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c as u8 - b'0')
        .collect();

    let clean: String = digits.iter().map(|d| (d + b'0') as char).collect();

    // 1. Forward Luhn
    let luhn_valid = luhn_check(&digits);

    // 2. Reverse Luhn consistency (same result with same digits)
    let mut reversed = digits.clone();
    reversed.reverse();
    // Reverse check: recalculate and verify the check digit is consistent
    let checksum_consistent = if digits.len() >= 2 {
        // The reverse Luhn means: if we reverse the full number and run Luhn,
        // the original check digit should still make the number pass Luhn forward.
        // This is inherently true for any Luhn-valid number, but we verify the
        // partial recomputation matches too.
        let partial = &digits[..digits.len() - 1];
        let computed_check = calculate_check_digit(partial);
        computed_check == *digits.last().unwrap_or(&255)
    } else {
        false
    };

    // 3. Detect brand
    let detected_brand = detect_brand_from_number(&clean);

    // 4. BIN lookup
    let card_info = db.lookup(&clean);

    // 5. Length validation
    let length_valid = if let Some(brand) = detected_brand {
        brand.valid_lengths().contains(&(digits.len() as u8))
    } else {
        // Unknown brand, accept common lengths
        [12, 13, 14, 15, 16, 17, 18, 19].contains(&(digits.len() as u8))
    };

    // 6. Structure validation (MII check + BIN range)
    let structure_valid = digits.len() >= 12
        && digits.len() <= 19
        && detected_brand.is_some()
        && length_valid;

    let bin_range_valid = card_info.is_some();

    // Calculate confidence score
    let mut score = 0.0f64;
    if luhn_valid { score += 0.35; }
    if structure_valid { score += 0.25; }
    if bin_range_valid { score += 0.20; }
    if length_valid { score += 0.10; }
    if checksum_consistent { score += 0.10; }

    let overall_valid = luhn_valid && length_valid && detected_brand.is_some();

    ValidationResult {
        luhn_valid,
        structure_valid,
        bin_range_valid,
        length_valid,
        checksum_consistent,
        overall_valid,
        card_brand: detected_brand,
        card_info,
        confidence_score: score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_luhn_valid_visa() {
        assert!(validate_luhn("4111111111111111"));
    }

    #[test]
    fn test_luhn_valid_amex() {
        assert!(validate_luhn("378282246310005"));
    }

    #[test]
    fn test_luhn_valid_mc() {
        assert!(validate_luhn("5500000000000004"));
    }

    #[test]
    fn test_luhn_invalid() {
        assert!(!validate_luhn("4111111111111112"));
    }

    #[test]
    fn test_check_digit_calculation() {
        // 4111111111111111 → check digit is 1
        let partial: Vec<u8> = vec![4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        let check = calculate_check_digit(&partial);
        assert_eq!(check, 1);
    }

    #[test]
    fn test_detect_visa() {
        assert_eq!(detect_brand_from_number("4111111111111111"), Some(CardBrand::Visa));
    }

    #[test]
    fn test_detect_mastercard() {
        assert_eq!(detect_brand_from_number("5500000000000004"), Some(CardBrand::MasterCard));
    }

    #[test]
    fn test_detect_amex() {
        assert_eq!(detect_brand_from_number("378282246310005"), Some(CardBrand::Amex));
    }

    #[test]
    fn test_detect_mastercard_2_series() {
        assert_eq!(detect_brand_from_number("2221000000000000"), Some(CardBrand::MasterCard));
    }

    #[test]
    fn test_triple_verify() {
        let db = BinDatabase::new();
        let result = triple_verify("4111111111111111", &db);
        assert!(result.luhn_valid);
        assert!(result.overall_valid);
        assert_eq!(result.card_brand, Some(CardBrand::Visa));
        assert!(result.confidence_score > 0.9);
    }
}
