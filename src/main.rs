use std::{fs, path::Path, result};

use reqwest;
use tokio;

/**
 * Malformed entries should cause the program to exit appropriately => Proper error handling
 * Added reqwest and tokip crates so I can have access to simple HTTP interactions
 */

// initialy wanted to go with f16 to use less memory for my variables but it seems this data type is unstable as pe this known issue
// <https://github.com/rust-lang/rust/issues/116909>
pub fn take_user_imput(x_coordinate: f32, y_coordinate: f32, input_file: String) {}

async fn download_csv_if_needed(
    url: &str,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(filename).exists() {
        println!(" The {} file already exists", filename);
        return Ok(());
    }

    print!("Downloading {} from {}...", filename, url);
    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    let _ = fs::write(filename, body);
    println!("Successfully downloaded {}", filename);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://raw.githubusercontent.com/Agilefreaks/test_oop/master/coffee_shops.csv";

    Ok(())
}
