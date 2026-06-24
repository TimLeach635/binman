use crate::collection::{Collection, CollectionSource};
use crate::error::Error;

// Full URL: format!("{COLLECTION_ENDPOINT}/{uprn}/?authority={AUTHORITY}&numberOfCollections={COLLECTION_COUNT}")
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
        todo!()
    }
}

impl CollectionSource for BartecClient {
    fn fetch_upcoming(&self) -> Result<Vec<Collection>, Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "requires BartecClient::new() implementation"]
    fn fetch_upcoming_returns_collections_for_valid_uprn() {
        // let client = BartecClient::new("100050403003");
        // let collections = client.fetch_upcoming().unwrap();
        // assert!(!collections.is_empty());
    }

    #[test]
    #[ignore = "requires BartecClient::new() implementation"]
    fn fetch_upcoming_collections_are_in_ascending_date_order() {
        // let client = BartecClient::new("100050403003");
        // let collections = client.fetch_upcoming().unwrap();
        // let dates: Vec<_> = collections.iter().map(|c| c.date).collect();
        // assert!(dates.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    #[ignore = "requires BartecClient::new() implementation"]
    fn fetch_upcoming_each_collection_has_nonempty_bins() {
        // let client = BartecClient::new("100050403003");
        // let collections = client.fetch_upcoming().unwrap();
        // assert!(collections.iter().all(|c| !c.bins.is_empty()));
    }
}
