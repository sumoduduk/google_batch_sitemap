mod indexing;

use chrono::{DateTime, Duration, ParseError, Utc};
use eyre::eyre;
use reqwest::Client;
use roxmltree::{Document, ParsingOptions};

pub async fn start_indexing(uri: &str, token: &str) -> eyre::Result<()> {
    let response_bytes: String = Client::new().get(uri).send().await?.text().await?;
    let opt = ParsingOptions {
        allow_dtd: true,
        ..ParsingOptions::default()
    };

    let doc = Document::parse_with_options(&response_bytes, opt)?;

    let mut arr_uris: Vec<String> = Vec::new();

    for node in doc.root().descendants() {
        let mut uri = "";
        let mut timestamp = "";

        if node.has_tag_name("url") {
            for child in node.children() {
                match child.tag_name().name() {
                    "loc" => {
                        uri = child.text().ok_or_else(|| eyre!("loc is none"))?.trim();
                    }
                    "lastmod" => {
                        timestamp = child.text().ok_or_else(|| eyre!("lastmod is none"))?.trim();
                    }
                    _ => (),
                }
            }
        } else {
            continue;
        }

        if !uri.is_empty() && !timestamp.is_empty() {
            arr_uris.push(uri.to_string());
        }
    }

    println!("{:#?}", &arr_uris);
    indexing::begin(arr_uris, token).await;

    Ok(())
}

fn filter_days(timestamp: &str, days: i64) -> Result<bool, ParseError> {
    let timestamp = DateTime::parse_from_rfc3339(timestamp)?;

    let day_now = Utc::now() - Duration::days(days);

    Ok(timestamp < day_now)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_days_ago() {
        let two_days_ago = (Utc::now() - Duration::days(2)).to_rfc3339();
        assert_eq!(filter_days(&two_days_ago, 2).unwrap(), true);
    }

    #[test]
    fn test_three_days_ago() {
        let three_days_ago = (Utc::now() - Duration::days(3)).to_rfc3339();
        assert_eq!(filter_days(&three_days_ago, 2).unwrap(), true);
    }

    #[test]
    fn test_one_day_ago() {
        let one_day_ago = (Utc::now() - Duration::days(1)).to_rfc3339();
        assert_eq!(filter_days(&one_day_ago, 2).unwrap(), false);
    }

    #[test]
    fn test_now() {
        let now = Utc::now().to_rfc3339();
        assert_eq!(filter_days(&now, 2).unwrap(), false);
    }

    #[test]
    fn test_invalid_timestamp() {
        assert!(filter_days("invalid timestamp", 2).is_err());
    }
}

