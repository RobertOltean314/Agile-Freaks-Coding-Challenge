use std::{env, fs, path::Path};

use reqwest;
use tokio;

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

    fs::write(filename, body)?;
    println!("Successfully downloaded {}", filename);

    Ok(())
}

async fn proccess_user_input() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!(
            "Usage: {} <user x coordinate> <user y coordinate> <shop data url>",
            args[0]
        );
        std::process::exit(1);
    }

    let x_coordinate: f32 = args[1]
        .parse()
        .expect("Invalid x coordinate: must be a number");
    let y_coordinate: f32 = args[2]
        .parse()
        .expect("Invalid y coordinate: must be a number");
    let shop_data_url = &args[3];

    let filename = "coffee_shops.csv";
    download_csv_if_needed(shop_data_url, filename).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    proccess_user_input().await?;
    Ok(())
}
