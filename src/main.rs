mod helpers;

use helpers::{FILENAME, calculate_distances, download_csv_if_needed};
use log::{debug, error, info};
use std::env;

async fn proccess_user_input() -> Result<(f32, f32), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        error!("Incorrect number of arguments provided");
        eprintln!(
            "Usage: {} <user y coordinate> <user x coordinate> <shop data url>",
            args[0]
        );
        eprintln!("\nExample:");
        eprintln!(
            "  {} 47.6 -122.4 https://example.com/coffee_shops.csv",
            args[0]
        );
        std::process::exit(1);
    }

    let y: f32 = args[1].parse().map_err(|_| {
        format!(
            "Invalid y coordinate '{}': must be a valid floating-point number (e.g., 47.6)",
            args[1]
        )
    })?;
    let x: f32 = args[2].parse().map_err(|_| {
        format!(
            "Invalid x coordinate '{}': must be a valid floating-point number (e.g., -122.4)",
            args[2]
        )
    })?;
    let shop_data_url = &args[3];

    info!("User coordinates: y={}, x={}", y, x);
    info!("Data source URL: {}", shop_data_url);

    download_csv_if_needed(shop_data_url, FILENAME).await?;

    Ok((y, x))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger (set RUST_LOG environment variable to control level)
    // Example: RUST_LOG=debug cargo run -- 47.6 -122.4 https://raw.githubusercontent.com/Agilefreaks/test_oop/master/coffee_shops.csv
    env_logger::init();

    info!("Starting coffee shop finder application");

    let (y, x) = match proccess_user_input().await {
        Ok(coords) => coords,
        Err(e) => {
            error!("Failed to process user input: {}", e);
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let mut shops = match calculate_distances(y, x, None, FILENAME) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to calculate distances: {}", e);
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    if shops.is_empty() {
        error!("No coffee shops found");
        eprintln!("Error: No valid coffee shops found in the data file");
        std::process::exit(1);
    }

    debug!("Sorting {} coffee shops by distance", shops.len());
    shops.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    info!(
        "Found {} coffee shops, displaying top 3 closest",
        shops.len()
    );

    for shop in shops.iter().take(3) {
        println!("{},{:.4}", shop.name, shop.distance);
    }

    info!("Application completed successfully");
    Ok(())
}
