//! # Compass Education API Client

#![forbid(unsafe_code)]
#![deny(unreachable_pub)]

pub mod api;
pub mod client;
pub mod errors;
pub mod models;

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test() {
        let client = crate::client::Client::new("kambryacollege-vic", "3feeee63-4305-45ee-a9bd-0edf47467e28");
        client.get_locations(None, None, None).await.unwrap();  
        client.login().await.unwrap();
    }
}