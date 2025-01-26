//! # API Models
//! 
//! Contains the API models (structs) for the Compass Education API.

use serde::Deserialize;
use serde_json::Value;

/// The response from most API requests.
/// 
/// Most likely obfuscated to prevent reverse engineering (makes our lives harder ;-;).
/// 
/// - `h`: Seems to be returned on error.
/// - `d`: The data returned from the API.
#[derive(Debug, Deserialize)]
pub(crate) struct Response {
    /// Seems to be returned on error.
    pub h: Option<String>,
    /// The data returned from the API. Can be any value, and is deserialized in respective API methods/functions.
    pub d: Option<Value>, // Can be any value...
}

/// Account model.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    #[serde(rename = "__type")]
    pub type_: Option<String>,
    pub user_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub name: String,
    pub account_id: Option<String>,
    pub compass_person_id: Option<String>,
    pub year_level_id: Option<i64>,
    pub username: Option<String>,
    pub user_status: Option<i32>,
    pub school_id: Option<String>,
    pub payment_methods: Option<Vec<Value>>,
    pub is_wallet_enabled: bool,
    pub balance: Option<i64>,
    pub image_url: Option<String>,
    pub free_meals: Value,
}

/// Location model. Represents a location on campus.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[serde(rename = "__type")]
    pub type_: Option<String>,
    pub id: i64,
    pub room_name: String,
    pub n: Option<String>,
    pub long_name: String,
    pub building: Option<String>,
    pub archived: bool,
}
