use wasm_bindgen::prelude::*;
use ccgen_core::card::*;
use ccgen_core::generator::CardGenerator;
use ccgen_core::validator;
use ccgen_core::bin_database::BinDatabase;
use std::cell::RefCell;

thread_local! {
    static GENERATOR: RefCell<CardGenerator> = RefCell::new(CardGenerator::new());
    static DATABASE: BinDatabase = BinDatabase::new();
}

/// Initialize the WASM module
#[wasm_bindgen]
pub fn init() {
    GENERATOR.with(|_| {});
    DATABASE.with(|_| {});
}

/// Generate credit card numbers
/// Returns a JSON string with the generation result
#[wasm_bindgen]
pub fn generate(
    bin_pattern: &str,
    count: u32,
    include_expiry: bool,
    include_cvv: bool,
    format: &str,
    min_years: u32,
    max_years: u32,
    unique: bool,
    card_length: i32,
) -> String {
    let output_format = OutputFormat::from_str(format).unwrap_or(OutputFormat::Pipe);

    let request = GenerateRequest {
        bin_pattern: bin_pattern.to_string(),
        count,
        include_expiry,
        include_cvv,
        format: output_format,
        min_expiry_years: min_years,
        max_expiry_years: max_years,
        unique,
        card_length: if card_length > 0 { Some(card_length as u8) } else { None },
    };

    GENERATOR.with(|cgen| {
        match cgen.borrow_mut().generate(&request) {
            Ok(result) => serde_json::to_string(&result).unwrap_or_else(|e| {
                format!(r#"{{"error":"Serialization failed: {}"}}"#, e)
            }),
            Err(e) => format!(r#"{{"error":"{}"}}"#, e),
        }
    })
}

/// Validate a card number
/// Returns a JSON string with the validation result
#[wasm_bindgen]
pub fn validate_card(card_number: &str) -> String {
    DATABASE.with(|db| {
        let result = validator::triple_verify(card_number, db);
        serde_json::to_string(&result).unwrap_or_else(|e| {
            format!(r#"{{"error":"{}"}}"#, e)
        })
    })
}

/// Look up a BIN (first 6-8 digits)
/// Returns a JSON string with card info
#[wasm_bindgen]
pub fn lookup_bin(bin: &str) -> String {
    DATABASE.with(|db| {
        match db.lookup(bin) {
            Some(info) => serde_json::to_string(&info).unwrap_or_else(|e| {
                format!(r#"{{"error":"{}"}}"#, e)
            }),
            None => r#"{"error":"BIN not found in database"}"#.to_string(),
        }
    })
}

/// Get all supported card brands
#[wasm_bindgen]
pub fn get_brands() -> String {
    let brands: Vec<serde_json::Value> = CardBrand::all()
        .iter()
        .map(|b| {
            serde_json::json!({
                "code": b.code_name(),
                "name": b.to_string(),
                "cvv_length": b.cvv_length(),
                "default_length": b.default_length(),
                "valid_lengths": b.valid_lengths(),
            })
        })
        .collect();
    serde_json::to_string(&brands).unwrap_or_else(|_| "[]".to_string())
}

/// Get the default BIN pattern for a brand
#[wasm_bindgen]
pub fn get_default_bin(brand_code: &str) -> String {
    DATABASE.with(|db| {
        if let Some(brand) = CardBrand::from_code(brand_code) {
            db.get_default_bin(brand).to_string()
        } else {
            String::new()
        }
    })
}

/// Detect card brand from number prefix
#[wasm_bindgen]
pub fn detect_brand(number: &str) -> String {
    match validator::detect_brand_from_number(number) {
        Some(brand) => serde_json::json!({
            "brand": brand.code_name(),
            "name": brand.to_string(),
        }).to_string(),
        None => r#"{"brand":null}"#.to_string(),
    }
}
