//! Helper functions for the coffee shop finder application.
//!
//! This module provides utilities for downloading CSV files from the network
//! and calculating distances between coordinates.

use csv::ReaderBuilder;
use log::{debug, info, warn};
use rayon::prelude::*;
use reqwest;
use std::{fs, path::Path};

/// Represents a coffee shop with its location and calculated distance.
#[derive(Debug, Clone, PartialEq)]
pub struct CoffeeShop {
    pub name: String,
    pub y: f32,
    pub x: f32,
    pub distance: f32,
}

impl CoffeeShop {
    pub fn new(name: String, y: f32, x: f32, distance: f32) -> Self {
        Self {
            name,
            y,
            x,
            distance,
        }
    }
}

/// Default filename for storing the downloaded coffee shop data.
pub const FILENAME: &str = "coffee_shops.csv";

/// Downloads a CSV file from a URL if it doesn't already exist locally.
///
/// If the file already exists at the specified path, this function returns
/// immediately without downloading. Otherwise, it fetches the file from the
/// provided URL and saves it to the local filesystem.
///
/// # Arguments
///
/// * `url` - The URL from which to download the CSV file
/// * `filename` - The local path where the file should be saved
///
/// # Returns
///
/// * `Ok(())` if the file already exists or was successfully downloaded
/// * `Err` if the download or file write operation fails
///
/// # Examples
///
/// ```no_run
/// # use af_coding_challenge::helpers::download_csv_if_needed;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// download_csv_if_needed(
///     "https://example.com/data.csv",
///     "local_data.csv"
/// ).await?;
/// # Ok(())
/// # }
/// ```
pub async fn download_csv_if_needed(
    url: &str,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(filename).exists() {
        info!("File '{}' already exists, skipping download", filename);
        return Ok(());
    }

    info!("Downloading '{}' from '{}'...", filename, url);

    let response = reqwest::get(url).await.map_err(|e| {
        format!(
            "Failed to download from '{}': {}. Check your internet connection and URL.",
            url, e
        )
    })?;

    if !response.status().is_success() {
        return Err(format!(
            "Server returned error {} when downloading from '{}'. The file may not exist or the server may be unavailable.",
            response.status(),
            url
        ).into());
    }

    let body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body from '{}': {}", url, e))?;

    fs::write(filename, &body).map_err(|e| {
        format!(
            "Failed to write file '{}': {}. Check if you have write permissions.",
            filename, e
        )
    })?;

    info!(
        "Successfully downloaded '{}' ({} bytes)",
        filename,
        body.len()
    );

    Ok(())
}

/// Calculates distances from a given point to all locations in a CSV file.
///
/// Reads a CSV file containing location data (name, y-coordinate, x-coordinate)
/// and calculates the distance from the given point to each location using the
/// Minkowski distance formula.
///
/// # Arguments
///
/// * `y` - The y-coordinate of the reference point
/// * `x` - The x-coordinate of the reference point
/// * `p` - The Minkowski distance parameter (defaults to 2 for Euclidean distance)
///   * `Some(1)` - Manhattan distance (L1 norm)
///   * `Some(2)` or `None` - Euclidean distance (L2 norm)
///   * `Some(n)` - General Minkowski distance with parameter n
/// * `filename` - Path to the CSV file containing location data
///
/// # Returns
///
/// * `Ok(Vec<CoffeeShop>)` - A vector of CoffeeShop instances with calculated distances
/// * `Err` if the file cannot be read or contains invalid data
///
/// # CSV Format
///
/// The CSV file should have no headers and each row should contain:
/// `name,y_coordinate,x_coordinate`
///
/// # Examples
///
/// ```no_run
/// # use af_coding_challenge::helpers::calculate_distances;
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let shops = calculate_distances(47.6, -122.4, None, "shops.csv")?;
/// for shop in shops {
///     println!("{}: {:.4}", shop.name, shop.distance);
/// }
/// # Ok(())
/// # }
/// ```
pub fn calculate_distances(
    x: f32,
    y: f32,
    p: Option<i32>,
    filename: &str,
) -> Result<Vec<CoffeeShop>, Box<dyn std::error::Error>> {
    let p = p.unwrap_or(2);
    debug!("Calculating distances using p={} (Minkowski parameter)", p);

    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)
        .map_err(|e| {
            format!(
                "Failed to open CSV file '{}': {}. Make sure the file exists and is readable.",
                filename, e
            )
        })?;

    let records: Vec<_> = reader.records()
        .collect::<Result<_, _>>()
        .map_err(|e| format!("Failed to read CSV records from '{}': {}. The file may be corqrupted or not a valid CSV.", filename, e))?;

    info!("Processing {} coffee shop records", records.len());

    // Process records in parallel for better performance on large datasets,
    // although for our .csv we don't really need this, I did this to play a bit with parallel proccessing
    // The actual performance starts to show up on datasets with 10.000+ entries
    let shops: Vec<CoffeeShop> = records
        .par_iter()
        .filter_map(|record| {
            let coffee_shop_name = record.get(0)?.to_string();
            let x1: f32 = record.get(1)?.parse().ok()?;
            let y1: f32 = record.get(2)?.parse().ok()?;

            let distance = if p == 1 {
                (x - x1).abs() + (y - y1).abs()
            } else if p == 2 {
                ((x - x1).powi(2) + (y - y1).powi(2)).sqrt()
            } else {
                ((x - x1).abs().powi(p) + (y - y1).abs().powi(p)).powf(1.0 / p as f32)
            };

            debug!(
                "Calculated distance for '{}': {:.4}",
                coffee_shop_name, distance
            );
            Some(CoffeeShop::new(coffee_shop_name, y1, x1, distance))
        })
        .collect();

    if shops.is_empty() {
        warn!(
            "No coffee shops found in '{}'. The file may be empty.",
            filename
        );
    } else {
        info!(
            "Successfully calculated distances for {} coffee shops",
            shops.len()
        );
    }

    Ok(shops)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_calculate_distances_euclidean() {
        // Temp CSV file
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Shop A,0.0,0.0").unwrap();
        writeln!(temp_file, "Shop B,3.0,4.0").unwrap();
        writeln!(temp_file, "Shop C,1.0,1.0").unwrap();

        let result = calculate_distances(0.0, 0.0, None, temp_file.path().to_str().unwrap());
        assert!(result.is_ok());

        let shops = result.unwrap();
        assert_eq!(shops.len(), 3);

        // Shop A should be at distance 0
        assert_eq!(shops[0].name, "Shop A");
        assert!((shops[0].distance - 0.0).abs() < 0.0001);

        // Shop B should be at distance 5.0
        assert_eq!(shops[1].name, "Shop B");
        assert!((shops[1].distance - 5.0).abs() < 0.0001);

        // Shop C should be at distance sqrt(2) ≈ 1.4142
        assert_eq!(shops[2].name, "Shop C");
        assert!((shops[2].distance - 1.4142).abs() < 0.001);
    }

    #[test]
    fn test_calculate_distances_manhattan() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Shop A,0.0,0.0").unwrap();
        writeln!(temp_file, "Shop B,3.0,4.0").unwrap();

        let result = calculate_distances(0.0, 0.0, Some(1), temp_file.path().to_str().unwrap());
        assert!(result.is_ok());

        let shops = result.unwrap();

        // Shop B Manhattan distance should be 7.0 (3 + 4)
        assert_eq!(shops[1].name, "Shop B");
        assert!((shops[1].distance - 7.0).abs() < 0.0001);
    }

    #[test]
    fn test_calculate_distances_empty_file() {
        let temp_file = NamedTempFile::new().unwrap();

        let result = calculate_distances(0.0, 0.0, None, temp_file.path().to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_calculate_distances_invalid_file() {
        let result = calculate_distances(0.0, 0.0, None, "nonexistent_file.csv");
        assert!(result.is_err());
    }
}
