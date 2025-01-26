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
    #[tokio::test]
    async fn test() {
        let client = crate::client::Client::new("-", "-");
        client.get_locations(None, None, None).await.unwrap();  
        client.login().await.unwrap();
    }
}