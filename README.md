# lexactivator

[![Latest Version](https://img.shields.io/crates/v/lexactivator.svg)](https://crates.io/crates/lexactivator)
[![Build](https://github.com/cryptlex/lexactivator-rust/actions/workflows/crate-publish.yml/badge.svg)](https://github.com/cryptlex/lexactivator-rust/actions/workflows/crate-publish.yml)
[![Documentation](https://docs.rs/lexactivator/badge.svg)](https://docs.rs/lexactivator)
![License](https://img.shields.io/crates/l/lexactivator)

lexactivator is a rust wrapper for cryptlex's licensing SDK that lets you implement any type of licensing model such as node-locked, hosted floating licenses, trials and much more. This SDK offers support for online and offline activations.

## Usage

In your Cargo.toml:

```toml
[dependencies]
lexactivator = { version = "3.24.3"}
```

Simple example usage:

```rust

use lexactivator::*;

fn main() {
    let license_key: String = String::from("LICENSE_KEY");
    let result = lexactivator::set_license_key(license_key);
    match result {
    Ok(()) => {
        // License Key set successfully
        println!("License key set successfully.");
    }
    Err(error) => {
        // Error occurred while setting license key
        println!("Error while setting license key: {:?}", error);
    }
    }
    let activation_result: Result<LexActivatorStatus, LexActivatorError> = lexactivator::activate_license();
    match activation_result {
        Ok(LexActivatorStatus::LA_OK) => {
            println!("License activated successfully");
        }
        Ok(_) => {
            // Other success cases if needed
        }
        
        Err(error) => {
            println!("License activation failed: {:?}", error);
        }
    }

}
```
## License

This project is licensed under 

* [MIT License](https://opensource.org/licenses/MIT)
