use crate::error::{CcGenError, Result};

/// A parsed BIN pattern segment
#[derive(Debug, Clone)]
pub enum PatternSegment {
    /// Exact digit (0-9)
    Fixed(u8),
    /// Any digit 0-9 ('x' or 'X')
    Random,
    /// Digit in range [min-max]
    Range(u8, u8),
    /// One of specific digits [135]
    OneOf(Vec<u8>),
    /// Any digit except 0 ('?')
    NonZero,
}

/// A parsed BIN pattern
#[derive(Debug, Clone)]
pub struct BinPattern {
    pub raw: String,
    pub segments: Vec<PatternSegment>,
    pub target_length: Option<u8>,
}

impl BinPattern {
    /// Parse a BIN pattern string
    pub fn parse(input: &str) -> Result<Self> {
        let raw = input.to_string();
        // Strip formatting characters (spaces, dashes)
        let cleaned: String = input.chars().filter(|c| !matches!(*c, ' ' | '-')).collect();

        let mut segments = Vec::new();
        let mut chars = cleaned.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '0'..='9' => {
                    segments.push(PatternSegment::Fixed(ch as u8 - b'0'));
                }
                'x' | 'X' | '_' => {
                    segments.push(PatternSegment::Random);
                }
                '?' => {
                    segments.push(PatternSegment::NonZero);
                }
                '[' => {
                    let mut bracket_content = String::new();
                    let mut found_close = false;
                    for bc in chars.by_ref() {
                        if bc == ']' {
                            found_close = true;
                            break;
                        }
                        bracket_content.push(bc);
                    }
                    if !found_close {
                        return Err(CcGenError::InvalidPattern(
                            "Unclosed bracket in pattern".to_string(),
                        ));
                    }

                    // Check if it's a range like "3-7" or a set like "13579"
                    if bracket_content.len() == 3 && bracket_content.as_bytes()[1] == b'-' {
                        let lo = bracket_content.as_bytes()[0] - b'0';
                        let hi = bracket_content.as_bytes()[2] - b'0';
                        if lo > 9 || hi > 9 || lo > hi {
                            return Err(CcGenError::InvalidPattern(format!(
                                "Invalid range [{bracket_content}]"
                            )));
                        }
                        segments.push(PatternSegment::Range(lo, hi));
                    } else {
                        let digits: Vec<u8> = bracket_content
                            .chars()
                            .filter_map(|c| {
                                if c.is_ascii_digit() {
                                    Some(c as u8 - b'0')
                                } else {
                                    None
                                }
                            })
                            .collect();
                        if digits.is_empty() {
                            return Err(CcGenError::InvalidPattern(format!(
                                "Empty digit set [{bracket_content}]"
                            )));
                        }
                        segments.push(PatternSegment::OneOf(digits));
                    }
                }
                '{' => {
                    let mut brace_content = String::new();
                    let mut found_close = false;
                    for bc in chars.by_ref() {
                        if bc == '}' {
                            found_close = true;
                            break;
                        }
                        brace_content.push(bc);
                    }
                    if !found_close {
                        return Err(CcGenError::InvalidPattern(
                            "Unclosed brace in pattern".to_string(),
                        ));
                    }
                    // {4,5} means either 4 or 5
                    let digits: Vec<u8> = brace_content
                        .split(',')
                        .filter_map(|s| s.trim().parse::<u8>().ok())
                        .filter(|&d| d <= 9)
                        .collect();
                    if digits.is_empty() {
                        return Err(CcGenError::InvalidPattern(format!(
                            "Invalid alternatives {{{brace_content}}}"
                        )));
                    }
                    segments.push(PatternSegment::OneOf(digits));
                }
                '*' => {
                    // Fill remaining with random digits
                    // This is handled at generation time by expanding to target length
                    segments.push(PatternSegment::Random);
                }
                _ => {
                    // Skip unknown characters
                }
            }
        }

        if segments.is_empty() {
            return Err(CcGenError::InvalidPattern(
                "Pattern produced no segments".to_string(),
            ));
        }

        Ok(Self {
            raw,
            segments,
            target_length: None,
        })
    }

    /// Set the target length for generation
    pub fn with_length(mut self, length: u8) -> Self {
        self.target_length = Some(length);
        self
    }

    /// Get the fixed prefix digits from this pattern (for BIN detection)
    pub fn fixed_prefix(&self) -> String {
        let mut prefix = String::new();
        for seg in &self.segments {
            if let PatternSegment::Fixed(d) = seg {
                prefix.push((d + b'0') as char);
            } else {
                break;
            }
        }
        prefix
    }

    /// Count the number of explicit segments
    pub fn explicit_length(&self) -> usize {
        self.segments.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_pattern() {
        let p = BinPattern::parse("4111xxxxxxxxxxxx").unwrap();
        assert_eq!(p.segments.len(), 16);
        assert_eq!(p.fixed_prefix(), "4111");
    }

    #[test]
    fn test_pattern_with_dashes() {
        let p = BinPattern::parse("4111-xxxx-xxxx-xxxx").unwrap();
        assert_eq!(p.segments.len(), 16);
        assert_eq!(p.fixed_prefix(), "4111");
    }

    #[test]
    fn test_range_pattern() {
        let p = BinPattern::parse("4[3-7]xxxxxxxxxxxxxx").unwrap();
        assert_eq!(p.segments.len(), 16);
        assert_eq!(p.fixed_prefix(), "4");
        matches!(&p.segments[1], PatternSegment::Range(3, 7));
    }

    #[test]
    fn test_oneof_pattern() {
        let p = BinPattern::parse("4[13579]xxxxxxxxxxxxxx").unwrap();
        assert_eq!(p.fixed_prefix(), "4");
        if let PatternSegment::OneOf(ref digits) = p.segments[1] {
            assert_eq!(digits, &[1, 3, 5, 7, 9]);
        } else {
            panic!("Expected OneOf");
        }
    }

    #[test]
    fn test_brace_pattern() {
        let p = BinPattern::parse("{4,5}xxxxxxxxxxxxxxx").unwrap();
        if let PatternSegment::OneOf(ref digits) = p.segments[0] {
            assert_eq!(digits, &[4, 5]);
        } else {
            panic!("Expected OneOf for brace pattern");
        }
    }

    #[test]
    fn test_nonzero_pattern() {
        let p = BinPattern::parse("4?xxxxxxxxxxxxxx").unwrap();
        matches!(&p.segments[1], PatternSegment::NonZero);
    }

    #[test]
    fn test_empty_pattern_fails() {
        assert!(BinPattern::parse("").is_err());
    }
}
