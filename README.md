# Compass Education API for Rust

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

## Documentation

Generate the documentation by cloning the repository with `git clone` and running:

```sh
cargo doc --open # Open the documentation in your browser
```

If it doesn't open automatically, navigate to `target/doc/compass_edu_api/index.html`.
