//! # API Methods
//! 
//! Expecting API methods here? Try the Client struct in the `client` module.
//! 
//! The API methods are implemented as methods on the `Client` struct.
//! 
//! [Jump to `Client`](../client/struct.Client.html)

// For Devs: This is the API module. It contains the API methods for the Compass Education API.
// API methods are implemented as methods for crate::client::Client.

use serde::Deserialize;

use crate::{
    errors::ApiError,
    models::{Location, Response},
};

impl crate::client::Client {
    /// Get all buildings on Campus.
    /// 
    /// Params:
    /// - `page`: The page number to get. Default is 1.
    /// - `start`: The start index. Default is 0.
    /// - `limit`: The limit of items to get. Default is 25.
    pub async fn get_locations(
        &self,
        page: Option<i64>,
        start: Option<i64>,
        limit: Option<i64>,
    ) -> Result<Vec<Location>, ApiError> {
        // Resolve default values
        let page = page.unwrap_or(1);
        let start = start.unwrap_or(0);
        let limit = limit.unwrap_or(25);

        let url = format!(
            "{}/ReferenceDataCache.svc/GetAllLocations?sessionstate=readonly&page={page}&start={start}&limit={limit}",
            self.inner.read().await.base_url,
        );

        let response = self.inner.read().await.client.get(&url).send().await?;

        let result: Response = response
            .json::<Response>()
            .await
            .map_err(|e| ApiError::ApiError(e.to_string()))?;

        if result.h.is_some() {
            return Err(ApiError::ApiError(result.h.unwrap()));
        }

        // Successfull request
        match result.d {
            Some(d) => Ok(Deserialize::deserialize(d)?),
            None => Err(ApiError::UnexpectedResponse),
        }
    }
    
    /// # **BROKEN. WILL NOT WORK.**
    /// 
    /// This method only stays here for reference.
    /// 
    /// Logs into the Compass Education API.
    ///
    /// Returns `()`, if the login request was successful.
    ///
    /// Returns an error if the login request failed.
    pub async fn login(&self) -> Result<(), ApiError> {
        if self.inner.read().await.user.is_some() {
            return Ok(());
        }

        let url = format!(
            "{}/Accounts.svc/GetAccount",
            self.inner.read().await.base_url
        );

        let response = self
            .inner
            .read()
            .await
            .client
            .post(&url)
            .header(
                reqwest::header::CONTENT_LENGTH,
                reqwest::header::HeaderValue::from_static("0"),
            )
            .send()
            .await?;

        let result: Response = response
            .json::<Response>()
            .await
            .map_err(|e| ApiError::ApiError(e.to_string()))?;

        if result.h.is_some() {
            return Err(ApiError::CompassUnauthorized(result.h.unwrap()));
        }

        // Successfull login
        match result.d {
            Some(d) => self.inner.write().await.user = Some(Deserialize::deserialize(d)?),
            None => return Err(ApiError::UnexpectedResponse),
        }

        Ok(())
    }
}
