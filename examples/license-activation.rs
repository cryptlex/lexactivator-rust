use std::io::{self, BufRead}; // for user input (pause)

use lexactivator::*;

extern "C" fn license_callback(status: u32) {
    if status == LexActivatorStatus::LA_OK as u32 {
        println!("License is active!");
    } else if status == LexActivatorStatus::LA_EXPIRED as u32 {
        println!("License has expired!");
    } else if status == LexActivatorStatus::LA_SUSPENDED as u32 {
        println!("License has been suspended!");
    } else if status == LexActivatorStatus::LA_GRACE_PERIOD_OVER as u32 {
        println!("License grace period is over!");
    } else {
        println!("License status code: {}", status);
    }
}

fn main() {
    let product_data: String = "Product.dat_content".to_string();
    let product_id: String = "Product_id".to_string();
    let license_key: String = "License_key".to_string();

    let mut result: Result<(), LexActivatorError> = lexactivator::set_product_data(product_data);
    println!("SetProductData: {:?}", result);

    result = lexactivator::set_product_id(product_id, PermissionFlags::LaUser);
    println!("SetProductId: {:?}", result);

    result = lexactivator::set_license_key(license_key);
    println!("Set_License_Key: {:?}", result);

    let activation_result: Result<LexActivatorStatus, LexActivatorError> =
        lexactivator::activate_license();
    match activation_result {
        Ok(LexActivatorStatus::LA_OK) => {
            println!("License activated successfully");
        }
        Ok(_) => {
            println!("License activation not successful");
        }

        Err(error) => {
            println!("License activation failed: {:?}", error);
        }
    }
    let callback_result: Result<(), LexActivatorError> =
        lexactivator::set_license_callback(license_callback);
    println!("SetLicenseCallback: {:?}", callback_result);

    let validation_result: Result<LexActivatorStatus, LexActivatorError> =
        lexactivator::is_license_genuine();
    match validation_result {
        Ok(LexActivatorStatus::LA_OK) => {}
        Ok(_) => {
            println!("License is not genuine");
        }
        Err(error) => {
            println!("License validation failed: {:?}", error);
        }
    }
    println!("Program paused.");
    let stdin = io::stdin();
    let _ = stdin.lock().lines().next();

    // let result = lexactivator::reset();
    // println!("Reset: {:?}", result);
}
