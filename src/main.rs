use std::{env, fs, path::Path};

use csv::ReaderBuilder;
use reqwest;
use tokio;

const FILENAME: &str = "coffee_shops.csv";

async fn download_csv_if_needed(
    url: &str,
    _filename_var: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(FILENAME).exists() {
        println!(" The {} file already exists", FILENAME);
        return Ok(());
    }

    print!("Downloading {} from {}...", FILENAME, url);
    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    fs::write(FILENAME, body)?;
    println!("Successfully downloaded {}", FILENAME);

    Ok(())
}

async fn proccess_user_input() -> Result<(f32, f32), Box<dyn std::error::Error>> {
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

    download_csv_if_needed(shop_data_url, FILENAME).await?;

    Ok((x_coordinate, y_coordinate))
}

fn calculate_distances(
    x: f32,
    y: f32,
    p: Option<i32>,
    _filename_var: &str,
) -> Result<Vec<(String, f32)>, Box<dyn std::error::Error>> {
    let p = p.unwrap_or(2);

    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(_filename_var)?;
    let mut distances = Vec::new();

    for result in reader.records() {
        let record = result?;

        let coffee_shop_name = record.get(0).unwrap_or("N/A").to_string();
        let x1: f32 = record.get(1).unwrap_or("0").parse()?;
        let y1: f32 = record.get(2).unwrap_or("0").parse()?;

        let distance = if p == 1 {
            (x - x1).abs() + (y - y1).abs()
        } else if p == 2 {
            ((x - x1).powi(2) + (y - y1).powi(2)).sqrt()
        } else {
            ((x - x1).abs().powi(p) + (y - y1).abs().powi(p)).powf(1.0 / p as f32)
        };

        distances.push((coffee_shop_name, distance));
    }

    Ok(distances)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (x, y) = proccess_user_input().await?;

    let mut distances = calculate_distances(x, y, None, FILENAME)?;

    // Debug: print all distances before sorting
    // println!("\nAll distances (before sorting):");
    // for (name, distance) in &distances {
    //     println!("{},{:.4}", name, distance);
    // }

    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    println!("\nThe 3 closest coffee shops:");
    for (name, distance) in distances.iter().take(3) {
        println!("{},{:.4}", name, distance);
    }

    Ok(())
}
