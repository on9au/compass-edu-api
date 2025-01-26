//! [Jump to `Navigating The Documentation`](#navigating-the-documentation)
//!
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![deny(unreachable_pub)]

//!
//! ## Navigating the documentation
//!
//! This crate is split into modules, each representing a different part of the API.
//!
//! For example, the `api` module contains the API methods, the `client` module contains the client, and the `models` module contains the API models.
//!
//! You can find the documentation for each module below.

pub mod api;
pub mod client;
pub mod errors;
pub mod models;

// test library lol
#[cfg(test)]
mod tests {
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn test() {
        dotenv().ok();
        let test_school_prefix = env::var("TEST_SCHOOL_PREFIX")
            .expect("TEST_SCHOOL_PREFIX must be set. Do you have a .env file or env vars set?");
        let test_cookie = env::var("TEST_COOKIE")
            .expect("TEST_COOKIE must be set. Do you have a .env file or env vars set?");

        let client = crate::client::Client::new(test_school_prefix, test_cookie);
        client.get_locations(None, None, None).await.unwrap();
        client.login().await.unwrap();
    }
}
