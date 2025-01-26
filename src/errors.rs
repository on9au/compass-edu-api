//! # Error module
//!
//! Contains the error enum. This is used to represent errors that can occur when using the Compass Education API.
//! 
//! Relies on the `thiserror` crate.
//! 
//! Take a look at the [`ApiError`] enum for more information.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    /// Reqwest Failure
    /// 
    /// Raised when there is an error with the reqwest client.
    /// 
    /// This could indicate:
    /// - Network issues.
    /// - Invalid URLs.
    /// - Invalid headers.
    /// - etc.
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// Serde JSON Failure
    /// 
    /// Raised when there is an error with JSON serialization/deserialization.
    /// 
    /// This could indicate:
    /// - Unexpected breaking changes in the API.
    /// - A bug in the library.
    /// - A bug in the API.
    #[error("JSON error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    /// API Error
    /// 
    /// Raised when the API returns an error.
    #[error("API error: {0}")]
    ApiError(String),
    
    /// Compass Invalid Argument
    /// 
    /// *Still unsure what this error means.*
    ///
    /// 50F4942D7A88B78667690C1239C736E7D1330268
    #[error("Compass Invalid Argument: {0}")]
    CompassInvalidArgument(String),

    /// Compass Invalid Credentials
    /// 
    /// *Still unsure what this error means.*
    /// 
    /// 5A5530FF244A9D000B4221B4860BE6764BC56DD9
    #[error("Compass Invalid Credentials: {0}")]
    CompassInvalidCredentials(String),

    /// Compass Unauthorized
    /// 
    /// Raised when the user is not authorised to perform the action
    /// 
    /// B375C43CC62EFAFD95259FEED722C5DC3B4F3187
    #[error("Compass Unauthorized: {0}")]
    CompassUnauthorized(String),

    /// Unexpected API response
    /// 
    /// Raised when the API returns an unexpected response.
    /// 
    /// This could indicate:
    /// - Unexpected breaking changes in the API.
    /// - A bug in the library.
    /// - A bug in the API.
    #[error("Unexpected API response")]
    UnexpectedResponse,
}
