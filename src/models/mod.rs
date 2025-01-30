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
    /// No clue.
    #[serde(rename = "__type")]
    pub type_: Option<String>,
    /// User ID.
    pub user_id: i64,
    /// First name.
    pub first_name: String,
    /// Last name.
    pub last_name: String,
    /// Full name.
    pub name: String,
    /// Account ID.
    pub account_id: Option<String>,
    /// Compass person ID.
    pub compass_person_id: Option<String>,
    /// Year level ID.
    pub year_level_id: Option<i64>,
    /// Username.
    pub username: Option<String>,
    /// User status.
    pub user_status: Option<i32>,
    /// School ID.
    pub school_id: Option<String>,
    /// Payment methods.
    pub payment_methods: Option<Vec<Value>>,
    /// Is wallet enabled?
    pub is_wallet_enabled: bool,
    /// Wallet balance.
    pub balance: Option<i64>,
    /// Image URLs.
    pub image_url: Option<String>,
    /// Free meals.
    pub free_meals: Value,
}

/// Location model. Represents a location on campus.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    /// No clue.
    #[serde(rename = "__type")]
    pub type_: Option<String>,
    /// Location ID.
    pub id: i64,
    /// Room name.
    pub room_name: String,
    /// n (?).
    pub n: Option<String>,
    /// Long name.
    pub long_name: String,
    /// Building.
    pub building: Option<String>,
    /// Archived?
    pub archived: bool,
}
