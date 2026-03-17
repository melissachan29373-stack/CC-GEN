use crate::card::{CardBrand, CardInfo, CardType};
use std::collections::HashMap;

/// A BIN range entry in the database
#[derive(Debug, Clone)]
pub struct BinEntry {
    pub range_start: u64,
    pub range_end: u64,
    pub brand: CardBrand,
    pub card_type: CardType,
    pub card_lengths: Vec<u8>,
    pub cvv_length: u8,
    pub issuer_name: String,
    pub country_code: Option<String>,
    pub country_name: Option<String>,
}

/// In-memory BIN database for fast lookups
pub struct BinDatabase {
    entries: Vec<BinEntry>,
    prefix_index: HashMap<u8, Vec<usize>>,
}

impl BinDatabase {
    pub fn new() -> Self {
        let entries = Self::build_entries();
        let mut prefix_index: HashMap<u8, Vec<usize>> = HashMap::new();

        for (i, entry) in entries.iter().enumerate() {
            // Index by first digit
            let first = (entry.range_start / 100000) as u8;
            prefix_index.entry(first).or_default().push(i);
        }

        Self {
            entries,
            prefix_index,
        }
    }

    /// Look up a BIN (first 6-8 digits) and return card info
    /// Prefers the most specific (narrowest range) match
    pub fn lookup(&self, bin_str: &str) -> Option<CardInfo> {
        let digits: String = bin_str.chars().filter(|c| c.is_ascii_digit()).collect();
        if digits.len() < 6 {
            return None;
        }
        let bin6: u64 = digits[..6].parse().ok()?;
        let first = (bin6 / 100000) as u8;

        let mut best_match: Option<&BinEntry> = None;
        let mut best_range_size: u64 = u64::MAX;

        if let Some(indices) = self.prefix_index.get(&first) {
            for &idx in indices {
                let entry = &self.entries[idx];
                if bin6 >= entry.range_start && bin6 <= entry.range_end {
                    let range_size = entry.range_end - entry.range_start;
                    if range_size < best_range_size {
                        best_range_size = range_size;
                        best_match = Some(entry);
                    }
                }
            }
        }

        best_match.map(|entry| CardInfo {
            brand: entry.brand,
            card_type: entry.card_type,
            issuer_name: entry.issuer_name.clone(),
            country_code: entry.country_code.clone(),
            country_name: entry.country_name.clone(),
        })
    }

    /// Detect card brand from a card number
    pub fn detect_brand(&self, card_number: &str) -> Option<CardBrand> {
        self.lookup(card_number).map(|info| info.brand)
    }

    /// Get valid lengths for a BIN
    pub fn get_lengths(&self, bin_str: &str) -> Option<Vec<u8>> {
        let digits: String = bin_str.chars().filter(|c| c.is_ascii_digit()).collect();
        if digits.len() < 4 {
            return None;
        }
        // Pad to 6 digits for lookup
        let lookup_str = if digits.len() < 6 {
            format!("{:0<6}", &digits)
        } else {
            digits[..6].to_string()
        };
        let bin6: u64 = lookup_str.parse().ok()?;
        let first = (bin6 / 100000) as u8;

        if let Some(indices) = self.prefix_index.get(&first) {
            for &idx in indices {
                let entry = &self.entries[idx];
                if bin6 >= entry.range_start && bin6 <= entry.range_end {
                    return Some(entry.card_lengths.clone());
                }
            }
        }
        None
    }

    /// Get a default BIN prefix for a brand
    pub fn get_default_bin(&self, brand: CardBrand) -> &str {
        match brand {
            CardBrand::Visa => "4",
            CardBrand::MasterCard => "51",
            CardBrand::Amex => "34",
            CardBrand::Discover => "6011",
            CardBrand::DinersClub => "36",
            CardBrand::JCB => "3528",
            CardBrand::UnionPay => "62",
            CardBrand::Maestro => "5018",
            CardBrand::Mir => "2200",
            CardBrand::RuPay => "60",
            CardBrand::Verve => "506099",
            CardBrand::UATP => "1",
            CardBrand::Dankort => "5019",
            CardBrand::InterPayment => "636",
        }
    }

    fn build_entries() -> Vec<BinEntry> {
        let mut entries = Vec::with_capacity(200);

        // === VISA ===
        let visa_issuers = [
            (400000, 400099, "Visa Inc.", "US", "United States"),
            (410000, 410099, "Chase Bank", "US", "United States"),
            (411111, 411111, "Chase Bank", "US", "United States"),
            (420000, 420099, "Wells Fargo", "US", "United States"),
            (426000, 426099, "Citibank", "US", "United States"),
            (430000, 430099, "HSBC", "GB", "United Kingdom"),
            (440000, 440099, "Bank of America", "US", "United States"),
            (450000, 450099, "Royal Bank of Canada", "CA", "Canada"),
            (453200, 453299, "Visa Test", "US", "United States"),
            (460000, 460099, "Barclays", "GB", "United Kingdom"),
            (470000, 470099, "Deutsche Bank", "DE", "Germany"),
            (480000, 480099, "ANZ Bank", "AU", "Australia"),
            (490000, 490099, "BNP Paribas", "FR", "France"),
        ];
        for (start, end, issuer, cc, cn) in &visa_issuers {
            entries.push(BinEntry {
                range_start: *start,
                range_end: *end,
                brand: CardBrand::Visa,
                card_type: CardType::Credit,
                card_lengths: vec![16],
                cvv_length: 3,
                issuer_name: issuer.to_string(),
                country_code: Some(cc.to_string()),
                country_name: Some(cn.to_string()),
            });
        }
        // Catch-all Visa range
        entries.push(BinEntry {
            range_start: 400000,
            range_end: 499999,
            brand: CardBrand::Visa,
            card_type: CardType::Credit,
            card_lengths: vec![13, 16, 19],
            cvv_length: 3,
            issuer_name: "Visa".to_string(),
            country_code: Some("US".to_string()),
            country_name: Some("United States".to_string()),
        });

        // === MASTERCARD ===
        let mc_issuers = [
            (510000, 510099, "Citibank", "US", "United States"),
            (515000, 515099, "HSBC", "GB", "United Kingdom"),
            (520000, 520099, "Bank of America", "US", "United States"),
            (525000, 525099, "Capital One", "US", "United States"),
            (530000, 530099, "ANZ Bank", "AU", "Australia"),
            (540000, 540099, "Royal Bank of Canada", "CA", "Canada"),
            (545400, 545499, "Citibank", "US", "United States"),
            (550000, 550099, "Wells Fargo", "US", "United States"),
        ];
        for (start, end, issuer, cc, cn) in &mc_issuers {
            entries.push(BinEntry {
                range_start: *start,
                range_end: *end,
                brand: CardBrand::MasterCard,
                card_type: CardType::Credit,
                card_lengths: vec![16],
                cvv_length: 3,
                issuer_name: issuer.to_string(),
                country_code: Some(cc.to_string()),
                country_name: Some(cn.to_string()),
            });
        }
        // MasterCard 51-55 range
        entries.push(BinEntry {
            range_start: 510000,
            range_end: 559999,
            brand: CardBrand::MasterCard,
            card_type: CardType::Credit,
            card_lengths: vec![16],
            cvv_length: 3,
            issuer_name: "MasterCard".to_string(),
            country_code: Some("US".to_string()),
            country_name: Some("United States".to_string()),
        });
        // MasterCard 2221-2720 range
        entries.push(BinEntry {
            range_start: 222100,
            range_end: 272099,
            brand: CardBrand::MasterCard,
            card_type: CardType::Credit,
            card_lengths: vec![16],
            cvv_length: 3,
            issuer_name: "MasterCard".to_string(),
            country_code: Some("US".to_string()),
            country_name: Some("United States".to_string()),
        });

        // === AMERICAN EXPRESS ===
        let amex_issuers = [
            (340000, 340099, "American Express", "US", "United States"),
            (341000, 341099, "American Express", "US", "United States"),
            (343000, 343099, "American Express", "GB", "United Kingdom"),
            (370000, 370099, "American Express", "US", "United States"),
            (371449, 371449, "American Express Test", "US", "United States"),
            (374000, 374099, "American Express", "AU", "Australia"),
            (376000, 376099, "American Express", "CA", "Canada"),
            (378000, 378099, "American Express", "JP", "Japan"),
        ];
        for (start, end, issuer, cc, cn) in &amex_issuers {
            entries.push(BinEntry {
                range_start: *start,
                range_end: *end,
                brand: CardBrand::Amex,
                card_type: CardType::Credit,
                card_lengths: vec![15],
                cvv_length: 4,
                issuer_name: issuer.to_string(),
                country_code: Some(cc.to_string()),
                country_name: Some(cn.to_string()),
            });
        }
        // Catch-all Amex
        entries.push(BinEntry {
            range_start: 340000,
            range_end: 349999,
            brand: CardBrand::Amex,
            card_type: CardType::Credit,
            card_lengths: vec![15],
            cvv_length: 4,
            issuer_name: "American Express".to_string(),
            country_code: Some("US".to_string()),
            country_name: Some("United States".to_string()),
        });
        entries.push(BinEntry {
            range_start: 370000,
            range_end: 379999,
            brand: CardBrand::Amex,
            card_type: CardType::Credit,
            card_lengths: vec![15],
            cvv_length: 4,
            issuer_name: "American Express".to_string(),
            country_code: Some("US".to_string()),
            country_name: Some("United States".to_string()),
        });

        // === DISCOVER ===
        entries.push(BinEntry {
            range_start: 601100,
            range_end: 601199,
            brand: CardBrand::Discover,
            card_type: CardType::Credit,
            card_lengths: vec![16, 17, 18, 19],
            cvv_length: 3,
            issuer_name: "Discover Financial".to_string(),
            country_code: Some("US".to_string()),
            country_name: Some("United States".to_string()),
        });
        entries.push(BinEntry {
            range_start: 622126,
            range_end: 622925,
            brand: CardBrand::Discover,
            card_type: CardType::Credit,
            card_lengths: vec![16, 17, 18, 19],
            cvv_length: 3,
            issuer_name: "Discover Financial".to_string(),
            country_code: Some("US".to_string()),
            country_name: Some("United States".to_string()),
        });
        entries.push(BinEntry {
            range_start: 644000,
            range_end: 649999,
            brand: CardBrand::Discover,
            card_type: CardType::Credit,
            card_lengths: vec![16, 17, 18, 19],
            cvv_length: 3,
            issuer_name: "Discover Financial".to_string(),
            country_code: Some("US".to_string()),
            country_name: Some("United States".to_string()),
        });
        entries.push(BinEntry {
            range_start: 650000,
            range_end: 659999,
            brand: CardBrand::Discover,
            card_type: CardType::Credit,
            card_lengths: vec![16, 17, 18, 19],
            cvv_length: 3,
            issuer_name: "Discover Financial".to_string(),
            country_code: Some("US".to_string()),
            country_name: Some("United States".to_string()),
        });

        // === DINERS CLUB ===
        for prefix in 300..=305 {
            entries.push(BinEntry {
                range_start: prefix * 1000,
                range_end: prefix * 1000 + 999,
                brand: CardBrand::DinersClub,
                card_type: CardType::Credit,
                card_lengths: vec![14, 15, 16, 17, 18, 19],
                cvv_length: 3,
                issuer_name: "Diners Club International".to_string(),
                country_code: Some("US".to_string()),
                country_name: Some("United States".to_string()),
            });
        }
        entries.push(BinEntry {
            range_start: 309500,
            range_end: 309599,
            brand: CardBrand::DinersClub,
            card_type: CardType::Credit,
            card_lengths: vec![14, 15, 16, 17, 18, 19],
            cvv_length: 3,
            issuer_name: "Diners Club International".to_string(),
            country_code: Some("US".to_string()),
            country_name: Some("United States".to_string()),
        });
        entries.push(BinEntry {
            range_start: 360000,
            range_end: 369999,
            brand: CardBrand::DinersClub,
            card_type: CardType::Credit,
            card_lengths: vec![14, 15, 16, 17, 18, 19],
            cvv_length: 3,
            issuer_name: "Diners Club International".to_string(),
            country_code: Some("US".to_string()),
            country_name: Some("United States".to_string()),
        });
        entries.push(BinEntry {
            range_start: 380000,
            range_end: 399999,
            brand: CardBrand::DinersClub,
            card_type: CardType::Credit,
            card_lengths: vec![14, 15, 16, 17, 18, 19],
            cvv_length: 3,
            issuer_name: "Diners Club International".to_string(),
            country_code: Some("US".to_string()),
            country_name: Some("United States".to_string()),
        });

        // === JCB ===
        entries.push(BinEntry {
            range_start: 352800,
            range_end: 358999,
            brand: CardBrand::JCB,
            card_type: CardType::Credit,
            card_lengths: vec![15, 16, 17, 18, 19],
            cvv_length: 3,
            issuer_name: "JCB Co., Ltd.".to_string(),
            country_code: Some("JP".to_string()),
            country_name: Some("Japan".to_string()),
        });

        // === UNIONPAY ===
        entries.push(BinEntry {
            range_start: 620000,
            range_end: 629999,
            brand: CardBrand::UnionPay,
            card_type: CardType::Credit,
            card_lengths: vec![16, 17, 18, 19],
            cvv_length: 3,
            issuer_name: "China UnionPay".to_string(),
            country_code: Some("CN".to_string()),
            country_name: Some("China".to_string()),
        });

        // === MAESTRO ===
        let maestro_prefixes: &[(u64, u64)] = &[
            (501800, 501899),
            (502000, 502099),
            (503800, 503899),
            (589300, 589399),
            (630400, 630499),
            (675900, 675999),
            (676100, 676399),
        ];
        for &(start, end) in maestro_prefixes {
            entries.push(BinEntry {
                range_start: start,
                range_end: end,
                brand: CardBrand::Maestro,
                card_type: CardType::Debit,
                card_lengths: vec![12, 13, 14, 15, 16, 17, 18, 19],
                cvv_length: 3,
                issuer_name: "Maestro".to_string(),
                country_code: Some("GB".to_string()),
                country_name: Some("United Kingdom".to_string()),
            });
        }

        // === MIR ===
        entries.push(BinEntry {
            range_start: 220000,
            range_end: 220499,
            brand: CardBrand::Mir,
            card_type: CardType::Debit,
            card_lengths: vec![16, 17, 18, 19],
            cvv_length: 3,
            issuer_name: "NSPK (Mir)".to_string(),
            country_code: Some("RU".to_string()),
            country_name: Some("Russia".to_string()),
        });

        // === RUPAY ===
        entries.push(BinEntry {
            range_start: 600000,
            range_end: 609999,
            brand: CardBrand::RuPay,
            card_type: CardType::Debit,
            card_lengths: vec![16],
            cvv_length: 3,
            issuer_name: "NPCI (RuPay)".to_string(),
            country_code: Some("IN".to_string()),
            country_name: Some("India".to_string()),
        });
        entries.push(BinEntry {
            range_start: 810000,
            range_end: 819999,
            brand: CardBrand::RuPay,
            card_type: CardType::Debit,
            card_lengths: vec![16],
            cvv_length: 3,
            issuer_name: "NPCI (RuPay)".to_string(),
            country_code: Some("IN".to_string()),
            country_name: Some("India".to_string()),
        });
        entries.push(BinEntry {
            range_start: 820000,
            range_end: 829999,
            brand: CardBrand::RuPay,
            card_type: CardType::Debit,
            card_lengths: vec![16],
            cvv_length: 3,
            issuer_name: "NPCI (RuPay)".to_string(),
            country_code: Some("IN".to_string()),
            country_name: Some("India".to_string()),
        });
        entries.push(BinEntry {
            range_start: 508000,
            range_end: 508999,
            brand: CardBrand::RuPay,
            card_type: CardType::Debit,
            card_lengths: vec![16],
            cvv_length: 3,
            issuer_name: "NPCI (RuPay)".to_string(),
            country_code: Some("IN".to_string()),
            country_name: Some("India".to_string()),
        });

        // === VERVE ===
        entries.push(BinEntry {
            range_start: 506099,
            range_end: 506198,
            brand: CardBrand::Verve,
            card_type: CardType::Debit,
            card_lengths: vec![16, 18, 19],
            cvv_length: 3,
            issuer_name: "Interswitch (Verve)".to_string(),
            country_code: Some("NG".to_string()),
            country_name: Some("Nigeria".to_string()),
        });
        entries.push(BinEntry {
            range_start: 650002,
            range_end: 650027,
            brand: CardBrand::Verve,
            card_type: CardType::Debit,
            card_lengths: vec![16, 18, 19],
            cvv_length: 3,
            issuer_name: "Interswitch (Verve)".to_string(),
            country_code: Some("NG".to_string()),
            country_name: Some("Nigeria".to_string()),
        });

        // === UATP ===
        entries.push(BinEntry {
            range_start: 100000,
            range_end: 199999,
            brand: CardBrand::UATP,
            card_type: CardType::Credit,
            card_lengths: vec![15],
            cvv_length: 3,
            issuer_name: "UATP".to_string(),
            country_code: Some("US".to_string()),
            country_name: Some("United States".to_string()),
        });

        // === DANKORT ===
        entries.push(BinEntry {
            range_start: 501900,
            range_end: 501999,
            brand: CardBrand::Dankort,
            card_type: CardType::Debit,
            card_lengths: vec![16],
            cvv_length: 3,
            issuer_name: "PBS (Dankort)".to_string(),
            country_code: Some("DK".to_string()),
            country_name: Some("Denmark".to_string()),
        });

        // === INTERPAYMENT ===
        entries.push(BinEntry {
            range_start: 636000,
            range_end: 636999,
            brand: CardBrand::InterPayment,
            card_type: CardType::Credit,
            card_lengths: vec![16, 17, 18, 19],
            cvv_length: 3,
            issuer_name: "InterPayment".to_string(),
            country_code: Some("US".to_string()),
            country_name: Some("United States".to_string()),
        });

        entries
    }
}

impl Default for BinDatabase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visa_lookup() {
        let db = BinDatabase::new();
        let info = db.lookup("411111").unwrap();
        assert_eq!(info.brand, CardBrand::Visa);
    }

    #[test]
    fn test_mastercard_lookup() {
        let db = BinDatabase::new();
        let info = db.lookup("510000").unwrap();
        assert_eq!(info.brand, CardBrand::MasterCard);
    }

    #[test]
    fn test_amex_lookup() {
        let db = BinDatabase::new();
        let info = db.lookup("371449").unwrap();
        assert_eq!(info.brand, CardBrand::Amex);
    }

    #[test]
    fn test_mastercard_2_series() {
        let db = BinDatabase::new();
        let info = db.lookup("222100").unwrap();
        assert_eq!(info.brand, CardBrand::MasterCard);
    }

    #[test]
    fn test_discover_lookup() {
        let db = BinDatabase::new();
        let info = db.lookup("601100").unwrap();
        assert_eq!(info.brand, CardBrand::Discover);
    }

    #[test]
    fn test_jcb_lookup() {
        let db = BinDatabase::new();
        let info = db.lookup("352800").unwrap();
        assert_eq!(info.brand, CardBrand::JCB);
    }

    #[test]
    fn test_short_bin() {
        let db = BinDatabase::new();
        assert!(db.lookup("41").is_none());
    }
}
