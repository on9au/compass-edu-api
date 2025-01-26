//! # API Models

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub h: Option<String>,
    pub d: Option<Value>, // Can be any value...
}

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
