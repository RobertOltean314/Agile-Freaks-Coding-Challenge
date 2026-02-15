use af_coding_challenge::helpers::calculate_distances;
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

    // Shop B Manhattan distance should be 7.0
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

#[test]
fn test_fewer_than_three_shops() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Shop A,0.0,0.0").unwrap();
    writeln!(temp_file, "Shop B,3.0,4.0").unwrap();

    let result = calculate_distances(0.0, 0.0, None, temp_file.path().to_str().unwrap());
    assert!(result.is_ok());

    let shops = result.unwrap();
    assert_eq!(shops.len(), 2); // Only 2 shops, not 3
}

#[test]
fn test_duplicate_shop_names() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Starbucks,0.0,0.0").unwrap();
    writeln!(temp_file, "Starbucks,3.0,4.0").unwrap();
    writeln!(temp_file, "Starbucks,1.0,1.0").unwrap();

    let result = calculate_distances(0.0, 0.0, None, temp_file.path().to_str().unwrap());
    assert!(result.is_ok());

    let shops = result.unwrap();
    assert_eq!(shops.len(), 3); // All 3 shops with same name should be included
    assert_eq!(shops[0].name, "Starbucks");
    assert_eq!(shops[1].name, "Starbucks");
    assert_eq!(shops[2].name, "Starbucks");
}

#[test]
fn test_negative_p_value() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Shop A,0.0,0.0").unwrap();

    let result = calculate_distances(0.0, 0.0, Some(-1), temp_file.path().to_str().unwrap());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("must be a positive integer")
    );
}

#[test]
fn test_zero_p_value() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Shop A,0.0,0.0").unwrap();

    let result = calculate_distances(0.0, 0.0, Some(0), temp_file.path().to_str().unwrap());
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("must be a positive integer")
    );
}
