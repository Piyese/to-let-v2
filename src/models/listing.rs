use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};

#[derive(Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "amenity")]
pub enum Amenity {
    Parking,
    Pool,
    Gated,
    Security,
    WheelchairAccessible,
    Elevator,
    Electricity,
    Water,
}

#[derive(Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "listing_type")]
pub enum ListingType {
    Single,
    Bedsitter,
    SelfContained,
}

#[derive(Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "listing")]
pub struct Listing {
    type_of_listing: ListingType,
    price: i32,
    number_of_bedrooms: i16, //0 FOR SINGLE AND BEDSITTERS
    available_units: i32,
    images: Vec<String>,
    additional_fees: Vec<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Collection {
    pub id: i32,
    pub title: String,
    pub display_image_url: String,
    pub description: Option<String>,
    pub location: String,
    pub contact_information: String,
    pub amenities: Vec<Amenity>,
    pub listings: Vec<Listing>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub rules: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CollectionSchema {
    pub title: String,
    pub description: Option<String>,
    pub display_image_url: String,
    pub location: String,
    pub contact_information: String,
    pub amenities: Vec<Amenity>,
    pub listings: Vec<Listing>,
    pub rules: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateCollectionSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub display_image_url: String,
    pub location: Option<String>,
    pub contact_information: Option<String>,
    pub amenities: Vec<Amenity>,
    pub listings: Option<Vec<Listing>>,
    pub rules: Option<Vec<String>>,
}
