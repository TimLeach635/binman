use chrono::Local;
use serde::Deserialize;

use crate::collection::{BinType, Collection, CollectionSource};
use crate::error::Error;

const COLLECTION_ENDPOINT: &str =
    "https://servicelayer3c.azure-api.net/wastecalendar/collection/search";
const AUTHORITY: &str = "CCC";
const COLLECTION_COUNT: u32 = 255;

pub struct BartecClient {
    uprn: String,
    http: reqwest::blocking::Client,
}

impl BartecClient {
    pub fn new(uprn: impl Into<String>) -> Self {
        Self {
            uprn: uprn.into(),
            http: reqwest::blocking::Client::new(),
        }
    }
}

impl CollectionSource for BartecClient {
    fn fetch_upcoming(&self) -> Result<Vec<Collection>, Error> {
        let url = format!(
            "{COLLECTION_ENDPOINT}/{}/?authority={AUTHORITY}&numberOfCollections={COLLECTION_COUNT}",
            self.uprn
        );
        let resp: ApiResponse = self
            .http
            .get(&url)
            .send()
            .map_err(|e| Error::ApiRequest(e.to_string()))?
            .json()
            .map_err(|e| Error::ApiParse(e.to_string()))?;

        let mut collections = resp
            .collections
            .into_iter()
            .map(parse_collection)
            .collect::<Result<Vec<_>, _>>()?;

        collections.sort_by_key(|c| c.date);
        Ok(collections)
    }
}

#[derive(Deserialize)]
struct ApiResponse {
    collections: Vec<ApiCollection>,
}

#[derive(Deserialize)]
struct ApiCollection {
    date: String,
    #[serde(rename = "roundTypes")]
    round_types: Vec<String>,
}

fn parse_collection(api: ApiCollection) -> Result<Collection, Error> {
    let local_date = chrono::DateTime::parse_from_rfc3339(&api.date)
        .map_err(|e| Error::ApiParse(format!("invalid date '{}': {e}", api.date)))?
        .with_timezone(&Local)
        .date_naive();

    let bins = api.round_types.iter().map(|rt| parse_bin_type(rt)).collect();
    Ok(Collection { date: local_date, bins })
}

fn parse_bin_type(s: &str) -> BinType {
    match s {
        "DOMESTIC" => BinType::GeneralWaste,
        "RECYCLE" => BinType::Recycling,
        "ORGANIC" => BinType::GardenWaste,
        "FOOD" => BinType::FoodWaste,
        other => BinType::Unknown(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn api_collection(date: &str, round_types: &[&str]) -> ApiCollection {
        ApiCollection {
            date: date.to_string(),
            round_types: round_types.iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn parse_bin_type_known_types() {
        assert_eq!(parse_bin_type("DOMESTIC"), BinType::GeneralWaste);
        assert_eq!(parse_bin_type("RECYCLE"), BinType::Recycling);
        assert_eq!(parse_bin_type("ORGANIC"), BinType::GardenWaste);
        assert_eq!(parse_bin_type("FOOD"), BinType::FoodWaste);
    }

    #[test]
    fn parse_bin_type_unknown_is_preserved() {
        assert_eq!(
            parse_bin_type("FOOD_WASTE"),
            BinType::Unknown("FOOD_WASTE".to_string())
        );
    }

    #[test]
    fn parse_collection_maps_round_types() {
        let c = parse_collection(api_collection("2024-01-15T00:00:00Z", &["DOMESTIC", "ORGANIC"]))
            .unwrap();
        assert!(c.bins.contains(&BinType::GeneralWaste));
        assert!(c.bins.contains(&BinType::GardenWaste));
    }

    #[test]
    fn parse_collection_invalid_date_returns_error() {
        assert!(matches!(
            parse_collection(api_collection("not-a-date", &["DOMESTIC"])),
            Err(Error::ApiParse(_))
        ));
    }

    #[test]
    fn collections_sort_ascending_by_date() {
        let mut cols = vec![
            parse_collection(api_collection("2024-03-25T00:00:00Z", &["RECYCLE"])).unwrap(),
            parse_collection(api_collection("2024-03-11T00:00:00Z", &["DOMESTIC"])).unwrap(),
            parse_collection(api_collection("2024-04-01T00:00:00Z", &["RECYCLE"])).unwrap(),
        ];
        cols.sort_by_key(|c| c.date);
        assert!(cols.windows(2).all(|w| w[0].date <= w[1].date));
    }

    #[test]
    #[ignore = "integration test: requires network"]
    fn fetch_upcoming_returns_collections_for_valid_uprn() {
        let client = BartecClient::new("100050403003");
        let collections = client.fetch_upcoming().unwrap();
        assert!(!collections.is_empty());
    }

    #[test]
    #[ignore = "integration test: requires network"]
    fn fetch_upcoming_collections_are_in_ascending_date_order() {
        let client = BartecClient::new("100050403003");
        let collections = client.fetch_upcoming().unwrap();
        assert!(collections.windows(2).all(|w| w[0].date <= w[1].date));
    }

    #[test]
    #[ignore = "integration test: requires network"]
    fn fetch_upcoming_each_collection_has_nonempty_bins() {
        let client = BartecClient::new("100050403003");
        let collections = client.fetch_upcoming().unwrap();
        assert!(collections.iter().all(|c| !c.bins.is_empty()));
    }
}
