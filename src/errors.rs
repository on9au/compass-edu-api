//! # Error module
//!
//! Contains the error enum

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    /// Reqwest Failure
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// Serde JSON Failure
    #[error("JSON error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    /// API Error
    #[error("API error: {0}")]
    ApiError(String),

    /// 50F4942D7A88B78667690C1239C736E7D1330268
    #[error("Compass Invalid Argument: {0}")]
    CompassInvalidArgument(String),

    /// 5A5530FF244A9D000B4221B4860BE6764BC56DD9
    #[error("Compass Invalid Credentials: {0}")]
    CompassInvalidCredentials(String),

    /// Raised when the user is not authorised to perform the action
    /// 
    /// B375C43CC62EFAFD95259FEED722C5DC3B4F3187
    #[error("Compass Unauthorized: {0}")]
    CompassUnauthorized(String),

    /// Unexpected API response
    #[error("Unexpected API response")]
    UnexpectedResponse,
}
