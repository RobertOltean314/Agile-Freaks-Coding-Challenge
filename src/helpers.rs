use csv::ReaderBuilder;
use reqwest;
use std::{fs, path::Path};

pub const FILENAME: &str = "coffee_shops.csv";

pub async fn download_csv_if_needed(
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

pub fn calculate_distances(
    x: f32,
    y: f32,
    p: Option<i32>,
    filename: &str,
) -> Result<Vec<(String, f32)>, Box<dyn std::error::Error>> {
    let p = p.unwrap_or(2);

    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)?;
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
