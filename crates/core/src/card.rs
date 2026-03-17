use serde::{Deserialize, Serialize};
use std::fmt;

/// Major Industry Identifier (first digit of card number)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MajorIndustry {
    Airlines,
    AirlinesFinancial,
    TravelEntertainment,
    BankingFinancial4,
    BankingFinancial5,
    MerchandisingBanking,
    Petroleum,
    HealthcareTelecom,
    NationalAssignment,
}

impl MajorIndustry {
    pub fn from_digit(d: u8) -> Option<Self> {
        match d {
            1 => Some(Self::Airlines),
            2 => Some(Self::AirlinesFinancial),
            3 => Some(Self::TravelEntertainment),
            4 => Some(Self::BankingFinancial4),
            5 => Some(Self::BankingFinancial5),
            6 => Some(Self::MerchandisingBanking),
            7 => Some(Self::Petroleum),
            8 => Some(Self::HealthcareTelecom),
            9 => Some(Self::NationalAssignment),
            _ => None,
        }
    }
}

/// Card network/brand
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardBrand {
    Visa,
    MasterCard,
    Amex,
    Discover,
    DinersClub,
    JCB,
    UnionPay,
    Maestro,
    Mir,
    RuPay,
    Verve,
    UATP,
    Dankort,
    InterPayment,
}

impl fmt::Display for CardBrand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Visa => write!(f, "Visa"),
            Self::MasterCard => write!(f, "MasterCard"),
            Self::Amex => write!(f, "American Express"),
            Self::Discover => write!(f, "Discover"),
            Self::DinersClub => write!(f, "Diners Club"),
            Self::JCB => write!(f, "JCB"),
            Self::UnionPay => write!(f, "UnionPay"),
            Self::Maestro => write!(f, "Maestro"),
            Self::Mir => write!(f, "Mir"),
            Self::RuPay => write!(f, "RuPay"),
            Self::Verve => write!(f, "Verve"),
            Self::UATP => write!(f, "UATP"),
            Self::Dankort => write!(f, "Dankort"),
            Self::InterPayment => write!(f, "InterPayment"),
        }
    }
}

impl CardBrand {
    pub fn cvv_length(&self) -> u8 {
        match self {
            Self::Amex => 4,
            _ => 3,
        }
    }

    pub fn default_length(&self) -> u8 {
        match self {
            Self::Amex | Self::UATP => 15,
            Self::DinersClub => 16,
            _ => 16,
        }
    }

    pub fn valid_lengths(&self) -> &[u8] {
        match self {
            Self::Visa => &[13, 16, 19],
            Self::MasterCard => &[16],
            Self::Amex => &[15],
            Self::Discover => &[16, 17, 18, 19],
            Self::DinersClub => &[14, 15, 16, 17, 18, 19],
            Self::JCB => &[15, 16, 17, 18, 19],
            Self::UnionPay => &[16, 17, 18, 19],
            Self::Maestro => &[12, 13, 14, 15, 16, 17, 18, 19],
            Self::Mir => &[16, 17, 18, 19],
            Self::RuPay => &[16],
            Self::Verve => &[16, 18, 19],
            Self::UATP => &[15],
            Self::Dankort => &[16],
            Self::InterPayment => &[16, 17, 18, 19],
        }
    }

    pub fn code_name(&self) -> &str {
        match self {
            Self::Visa => "visa",
            Self::MasterCard => "mastercard",
            Self::Amex => "amex",
            Self::Discover => "discover",
            Self::DinersClub => "diners",
            Self::JCB => "jcb",
            Self::UnionPay => "unionpay",
            Self::Maestro => "maestro",
            Self::Mir => "mir",
            Self::RuPay => "rupay",
            Self::Verve => "verve",
            Self::UATP => "uatp",
            Self::Dankort => "dankort",
            Self::InterPayment => "interpayment",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_lowercase().as_str() {
            "visa" => Some(Self::Visa),
            "mastercard" | "mc" => Some(Self::MasterCard),
            "amex" | "americanexpress" => Some(Self::Amex),
            "discover" => Some(Self::Discover),
            "diners" | "dinersclub" => Some(Self::DinersClub),
            "jcb" => Some(Self::JCB),
            "unionpay" => Some(Self::UnionPay),
            "maestro" => Some(Self::Maestro),
            "mir" => Some(Self::Mir),
            "rupay" => Some(Self::RuPay),
            "verve" => Some(Self::Verve),
            "uatp" => Some(Self::UATP),
            "dankort" => Some(Self::Dankort),
            "interpayment" => Some(Self::InterPayment),
            _ => None,
        }
    }

    pub fn all() -> &'static [CardBrand] {
        &[
            Self::Visa,
            Self::MasterCard,
            Self::Amex,
            Self::Discover,
            Self::DinersClub,
            Self::JCB,
            Self::UnionPay,
            Self::Maestro,
            Self::Mir,
            Self::RuPay,
            Self::Verve,
            Self::UATP,
            Self::Dankort,
            Self::InterPayment,
        ]
    }
}

/// Card type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardType {
    Credit,
    Debit,
    Prepaid,
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Credit => write!(f, "Credit"),
            Self::Debit => write!(f, "Debit"),
            Self::Prepaid => write!(f, "Prepaid"),
        }
    }
}

/// Validation result with detailed breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub luhn_valid: bool,
    pub structure_valid: bool,
    pub bin_range_valid: bool,
    pub length_valid: bool,
    pub checksum_consistent: bool,
    pub overall_valid: bool,
    pub card_brand: Option<CardBrand>,
    pub card_info: Option<CardInfo>,
    pub confidence_score: f64,
}

/// Information about a card from BIN lookup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardInfo {
    pub brand: CardBrand,
    pub card_type: CardType,
    pub issuer_name: String,
    pub country_code: Option<String>,
    pub country_name: Option<String>,
}

/// A fully generated card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedCard {
    pub number: String,
    pub number_formatted: String,
    pub brand: CardBrand,
    pub card_type: CardType,
    pub expiration_month: String,
    pub expiration_year: String,
    pub cvv: String,
    pub issuer: Option<String>,
    pub country: Option<String>,
    pub luhn_valid: bool,
}

impl GeneratedCard {
    /// Format card number with spaces according to brand
    pub fn format_number(number: &str, brand: &CardBrand) -> String {
        match brand {
            CardBrand::Amex => {
                // 4-6-5 grouping
                if number.len() == 15 {
                    format!(
                        "{} {} {}",
                        &number[0..4],
                        &number[4..10],
                        &number[10..15]
                    )
                } else {
                    Self::default_format(number)
                }
            }
            _ => Self::default_format(number),
        }
    }

    fn default_format(number: &str) -> String {
        number
            .chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// Output format for generated cards
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputFormat {
    Pipe,
    Csv,
    Tsv,
    Json,
    JsonArray,
    Xml,
    Yaml,
    Sql,
    CardOnly,
    Formatted,
    StripeTest,
    PayPalSandbox,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pipe" => Some(Self::Pipe),
            "csv" => Some(Self::Csv),
            "tsv" => Some(Self::Tsv),
            "json" => Some(Self::Json),
            "json_array" | "jsonarray" => Some(Self::JsonArray),
            "xml" => Some(Self::Xml),
            "yaml" => Some(Self::Yaml),
            "sql" => Some(Self::Sql),
            "card_only" | "cardonly" => Some(Self::CardOnly),
            "formatted" => Some(Self::Formatted),
            "stripe" | "stripetest" => Some(Self::StripeTest),
            "paypal" | "paypalsandbox" => Some(Self::PayPalSandbox),
            _ => None,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Pipe => "Pipe",
            Self::Csv => "CSV",
            Self::Tsv => "TSV",
            Self::Json => "JSON",
            Self::JsonArray => "JSON Array",
            Self::Xml => "XML",
            Self::Yaml => "YAML",
            Self::Sql => "SQL INSERT",
            Self::CardOnly => "Card Only",
            Self::Formatted => "Formatted",
            Self::StripeTest => "Stripe Test",
            Self::PayPalSandbox => "PayPal Sandbox",
        }
    }
}

/// Generation request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub bin_pattern: String,
    pub count: u32,
    pub include_expiry: bool,
    pub include_cvv: bool,
    pub format: OutputFormat,
    pub min_expiry_years: u32,
    pub max_expiry_years: u32,
    pub unique: bool,
    pub card_length: Option<u8>,
}

impl Default for GenerateRequest {
    fn default() -> Self {
        Self {
            bin_pattern: "4xxxxxxxxxxxxx".to_string(),
            count: 10,
            include_expiry: true,
            include_cvv: true,
            format: OutputFormat::Pipe,
            min_expiry_years: 1,
            max_expiry_years: 5,
            unique: true,
            card_length: None,
        }
    }
}

/// Generation result with statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateResult {
    pub cards: Vec<GeneratedCard>,
    pub formatted_output: String,
    pub stats: GenerationStats,
}

/// Statistics about the generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationStats {
    pub total_generated: u32,
    pub valid_count: u32,
    pub generation_time_us: u64,
    pub unique_count: u32,
}
