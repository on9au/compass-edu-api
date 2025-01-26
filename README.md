# Compass Education API for Rust

> [!CAUTION]
> This library is prone to breaking if Compass Education changes their API. Use at your own risk.

A rust library for interacting with the Compass Education API.

Uses Reqwest and Tokio under the hood.

This library is a work in progress and is not yet feature complete.

**Note:** This library is not affiliated with Compass Education.

The methods and models are based on investigation of Compass Education's website.

This library may break if Compass Education changes their API.

## Usage

Type this in your terminal:

```sh
cargo add https://github.com/on9au/compass-edu-api
```

or add this to your `Cargo.toml`:

```toml
[dependencies]
compass_edu_api = { git = "https://github.com/on9au/compass-edu-api" }
```

## Simple Example

```rust
#[tokio::main]
async fn main() {
    // Create a new client with your school prefix and auth cookie
    let client = compass_edu_api::client::Client::new("<School Prefix>", "<Auth Cookie>");
    // Login to the API
    client.login().await.unwrap();
    // Get a list of locations from the API
    let locations: Vec<compass_edu_api::models::Location> = client.get_locations(None, None, None).await.unwrap();
    // Print the locations to the console
    println!("{:?}", locations);
}
```

## Documentation

Generate the documentation by cloning the repository with `git clone` and running:

```sh
cargo doc --open # Open the documentation in your browser
```

If it doesn't open automatically, navigate to `target/doc/compass_edu_api/index.html`.
