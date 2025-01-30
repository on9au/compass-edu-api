impl crate::client::Client {
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
