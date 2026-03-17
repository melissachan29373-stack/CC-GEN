use crate::card::{GeneratedCard, OutputFormat};
use serde_json;

/// Format a list of generated cards in the specified output format
pub fn format_cards(cards: &[GeneratedCard], format: OutputFormat) -> String {
    match format {
        OutputFormat::Pipe => format_pipe(cards),
        OutputFormat::Csv => format_csv(cards),
        OutputFormat::Tsv => format_tsv(cards),
        OutputFormat::Json => format_json(cards),
        OutputFormat::JsonArray => format_json_array(cards),
        OutputFormat::Xml => format_xml(cards),
        OutputFormat::Yaml => format_yaml(cards),
        OutputFormat::Sql => format_sql(cards),
        OutputFormat::CardOnly => format_card_only(cards),
        OutputFormat::Formatted => format_formatted(cards),
        OutputFormat::StripeTest => format_stripe(cards),
        OutputFormat::PayPalSandbox => format_paypal(cards),
    }
}

fn format_pipe(cards: &[GeneratedCard]) -> String {
    cards
        .iter()
        .map(|c| format!("{}|{}|{}|{}", c.number, c.expiration_month, c.expiration_year, c.cvv))
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_csv(cards: &[GeneratedCard]) -> String {
    let mut out = String::from("number,month,year,cvv,brand\n");
    for c in cards {
        out.push_str(&format!(
            "{},{},{},{},{}\n",
            c.number, c.expiration_month, c.expiration_year, c.cvv, c.brand
        ));
    }
    out
}

fn format_tsv(cards: &[GeneratedCard]) -> String {
    let mut out = String::from("number\tmonth\tyear\tcvv\tbrand\n");
    for c in cards {
        out.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\n",
            c.number, c.expiration_month, c.expiration_year, c.cvv, c.brand
        ));
    }
    out
}

fn format_json(cards: &[GeneratedCard]) -> String {
    cards
        .iter()
        .map(|c| {
            format!(
                r#"{{"card":"{}","month":"{}","year":"{}","cvv":"{}","brand":"{}"}}"#,
                c.number, c.expiration_month, c.expiration_year, c.cvv, c.brand
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_json_array(cards: &[GeneratedCard]) -> String {
    serde_json::to_string_pretty(cards).unwrap_or_else(|_| "[]".to_string())
}

fn format_xml(cards: &[GeneratedCard]) -> String {
    let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<cards>\n");
    for c in cards {
        out.push_str(&format!(
            "  <card>\n    <number>{}</number>\n    <month>{}</month>\n    <year>{}</year>\n    <cvv>{}</cvv>\n    <brand>{}</brand>\n  </card>\n",
            c.number, c.expiration_month, c.expiration_year, c.cvv, c.brand
        ));
    }
    out.push_str("</cards>");
    out
}

fn format_yaml(cards: &[GeneratedCard]) -> String {
    let mut out = String::from("cards:\n");
    for c in cards {
        out.push_str(&format!(
            "  - number: \"{}\"\n    month: \"{}\"\n    year: \"{}\"\n    cvv: \"{}\"\n    brand: \"{}\"\n",
            c.number, c.expiration_month, c.expiration_year, c.cvv, c.brand
        ));
    }
    out
}

fn format_sql(cards: &[GeneratedCard]) -> String {
    cards
        .iter()
        .map(|c| {
            format!(
                "INSERT INTO cards (number, exp_month, exp_year, cvv, brand) VALUES ('{}', '{}', '{}', '{}', '{}');",
                c.number, c.expiration_month, c.expiration_year, c.cvv, c.brand
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_card_only(cards: &[GeneratedCard]) -> String {
    cards
        .iter()
        .map(|c| c.number.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_formatted(cards: &[GeneratedCard]) -> String {
    cards
        .iter()
        .map(|c| {
            format!(
                "{} | {}/{} | {}",
                c.number_formatted, c.expiration_month, c.expiration_year, c.cvv
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_stripe(cards: &[GeneratedCard]) -> String {
    cards
        .iter()
        .map(|c| {
            format!(
                "card[number]={}&card[exp_month]={}&card[exp_year]={}&card[cvc]={}",
                c.number, c.expiration_month, c.expiration_year, c.cvv
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_paypal(cards: &[GeneratedCard]) -> String {
    cards
        .iter()
        .map(|c| {
            format!(
                r#"{{"card_number":"{}","expire_month":"{}","expire_year":"{}","cvv2":"{}","brand":"{}"}}"#,
                c.number, c.expiration_month, c.expiration_year, c.cvv, c.brand
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{CardBrand, CardType};

    fn sample_card() -> GeneratedCard {
        GeneratedCard {
            number: "4111111111111111".to_string(),
            number_formatted: "4111 1111 1111 1111".to_string(),
            brand: CardBrand::Visa,
            card_type: CardType::Credit,
            expiration_month: "03".to_string(),
            expiration_year: "2028".to_string(),
            cvv: "123".to_string(),
            issuer: Some("Chase".to_string()),
            country: Some("US".to_string()),
            luhn_valid: true,
        }
    }

    #[test]
    fn test_pipe_format() {
        let out = format_cards(&[sample_card()], OutputFormat::Pipe);
        assert_eq!(out, "4111111111111111|03|2028|123");
    }

    #[test]
    fn test_csv_format() {
        let out = format_cards(&[sample_card()], OutputFormat::Csv);
        assert!(out.contains("4111111111111111,03,2028,123,Visa"));
    }

    #[test]
    fn test_json_format() {
        let out = format_cards(&[sample_card()], OutputFormat::Json);
        assert!(out.contains("4111111111111111"));
        assert!(out.contains("\"card\""));
    }

    #[test]
    fn test_xml_format() {
        let out = format_cards(&[sample_card()], OutputFormat::Xml);
        assert!(out.contains("<number>4111111111111111</number>"));
    }

    #[test]
    fn test_sql_format() {
        let out = format_cards(&[sample_card()], OutputFormat::Sql);
        assert!(out.starts_with("INSERT INTO cards"));
    }

    #[test]
    fn test_card_only_format() {
        let out = format_cards(&[sample_card()], OutputFormat::CardOnly);
        assert_eq!(out, "4111111111111111");
    }
}
