# Actix Web Minimal Starter

A basic actix web starter repository.

## Basic configuration

An actix-web application, with CLI interface to configure host and port. Includes a rough app structure that separates 
tests from app code inspired by [zero-to-prod](https://github.com/LukeMathWalker/zero-to-production)

### Main dependencies

- [Actix web](https://actix.rs)
- [lexopt](https://docs.rs/lexopt/latest/lexopt/) for CLI interface
- GitHub actions that depend on https://github.com/dtolnay/rust-toolchain

## Usage

To use this repository you will need:
- Rust

### Local

To run locally:

1. Run cargo

    ```bash
    cargo run
    ```
    
    Visit http://127.0.0.1:8080 to view the app.
    
    Optionally run on a different port
    
    ```bash
    cargo run -- -p 5000
    ```
    
    View other CLI options with:
    
    ```bash
    cargo run -- --help
    Usage: actix_starter [-h|--host=X.X.X.X] [-p|--port=XXXX]
    ```

2. To run the tests:

   ```bash
   cargo test
   ```
