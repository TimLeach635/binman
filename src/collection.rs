use chrono::NaiveDate;

use crate::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinType {
    GeneralWaste,
    Recycling,
    GardenWaste,
    FoodWaste,
    /// A round type returned by the API that is not recognised by this version of binman.
    Unknown(String),
}

impl std::fmt::Display for BinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BinType::GeneralWaste => "general waste",
            BinType::Recycling => "recycling",
            BinType::GardenWaste => "garden waste",
            BinType::FoodWaste => "food waste",
            BinType::Unknown(s) => s.as_str(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Collection {
    /// The local date of the collection.
    ///
    /// The API returns UTC datetimes (e.g. `2024-03-18T00:00:00Z`). Implementations
    /// must convert to the local date — do not extract the date in UTC, as this can
    /// be off by one day during BST (e.g. `2024-06-24T23:00:00Z` is 25 June locally).
    pub date: NaiveDate,
    /// Always non-empty — guaranteed by the `CollectionSource` implementation.
    pub bins: Vec<BinType>,
}

pub trait CollectionSource {
    /// Returns upcoming collections sorted in ascending date order.
    ///
    /// Each returned `Collection` has a non-empty `bins` list.
    /// Returns an empty `Vec` if no collections are scheduled.
    fn fetch_upcoming(&self) -> Result<Vec<Collection>, Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_type_display_general_waste() {
        assert_eq!(BinType::GeneralWaste.to_string(), "general waste");
    }

    #[test]
    fn bin_type_display_recycling() {
        assert_eq!(BinType::Recycling.to_string(), "recycling");
    }

    #[test]
    fn bin_type_display_garden_waste() {
        assert_eq!(BinType::GardenWaste.to_string(), "garden waste");
    }

    #[test]
    fn bin_type_display_food_waste() {
        assert_eq!(BinType::FoodWaste.to_string(), "food waste");
    }

    #[test]
    fn bin_type_display_unknown_preserves_raw_string() {
        assert_eq!(
            BinType::Unknown("FOOD_WASTE".to_string()).to_string(),
            "FOOD_WASTE"
        );
    }

    #[test]
    fn bin_type_equality() {
        assert_eq!(BinType::GeneralWaste, BinType::GeneralWaste);
        assert_ne!(BinType::GeneralWaste, BinType::Recycling);
        assert_eq!(
            BinType::Unknown("FOO".to_string()),
            BinType::Unknown("FOO".to_string())
        );
        assert_ne!(
            BinType::Unknown("FOO".to_string()),
            BinType::Unknown("BAR".to_string())
        );
    }
}
