use thiserror::Error;

#[derive(Error, Debug)]
pub enum CcGenError {
    #[error("Invalid BIN pattern: {0}")]
    InvalidPattern(String),

    #[error("Invalid card number: {0}")]
    InvalidCardNumber(String),

    #[error("Unsupported card brand: {0}")]
    UnsupportedBrand(String),

    #[error("Invalid card length {length} for brand {brand}")]
    InvalidLength { brand: String, length: u8 },

    #[error("Generation failed after {0} attempts")]
    GenerationFailed(u32),

    #[error("Quantity must be between 1 and {0}")]
    InvalidQuantity(u32),
}

pub type Result<T> = std::result::Result<T, CcGenError>;
