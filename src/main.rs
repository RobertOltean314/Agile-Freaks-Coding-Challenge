mod helpers;

use helpers::{FILENAME, calculate_distances, download_csv_if_needed};
use std::env;

async fn proccess_user_input() -> Result<(f32, f32), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!(
            "Usage: {} <user x coordinate> <user y coordinate> <shop data url>",
            args[0]
        );
        std::process::exit(1);
    }

    let y: f32 = args[1]
        .parse()
        .expect("Invalid x coordinate: must be a number");
    let x: f32 = args[2]
        .parse()
        .expect("Invalid y coordinate: must be a number");
    let shop_data_url = &args[3];

    download_csv_if_needed(shop_data_url, FILENAME).await?;

    Ok((y, x))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (y, x) = proccess_user_input().await?;

    let mut distances = calculate_distances(y, x, None, FILENAME)?;

    // Debug: print all distances before sorting
    // println!("\nAll distances (before sorting):");
    // for (name, distance) in &distances {
    //     println!("{},{:.4}", name, distance);
    // }

    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    for (name, distance) in distances.iter().take(3) {
        println!("{},{:.4}", name, distance);
    }

    Ok(())
}
