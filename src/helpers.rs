//! Helper functions for the coffee shop finder application.
//!
//! This module provides utilities for downloading CSV files from the network
//! and calculating distances between coordinates.

pub mod helpers {
    use csv::ReaderBuilder;
    use log::{debug, info, warn};
    use rayon::prelude::*;
    use reqwest;
    use std::{fs, path::Path};

    pub const FILENAME: &str = "coffee_shops.csv";

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
        let p = p.unwrap_or(2); // default p = 2 (Euclidian Distance)

        if p <= 0 {
            return Err(format!(
                "Invalid Minkowski parameter p={}: must be a positive integer (p > 0)",
                p
            )
            .into());
        }

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
                    // Manhattan
                    (x - x1).abs() + (y - y1).abs()
                } else if p == 2 {
                    // Euclidean
                    ((x - x1).powi(2) + (y - y1).powi(2)).sqrt()
                } else {
                    // General Minkowski distance
                    ((x - x1).abs().powi(p) + (y - y1).abs().powi(p)).powf(1.0 / p as f32)
                };

                // Validate if distance != Nan or Infinite
                if !distance.is_finite() {
                    warn!(
                        "Skipping '{}': calculated distance is invalid (NaN or Infinity)",
                        coffee_shop_name
                    );
                    return None;
                }

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
}
