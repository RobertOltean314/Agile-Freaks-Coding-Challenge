# Coffee Shop Finder - Usage Guide

A Rust application that finds the three closest coffee shops to a given location.

## Prerequisites

- Rust and Cargo installed

## Building the Project

```bash
cargo build --release
```

## Running the Program

### Basic Usage (No Logs)

```bash
cargo run -- <y_coordinate> <x_coordinate> <csv_url>
```

**Example:**

```bash
cargo run -- 47.6 -122.4 https://raw.githubusercontent.com/Agilefreaks/test_oop/master/coffee_shops.csv
```

### With Logging

Use the `RUST_LOG` environment variable to enable different log levels:

**Info level (recommended):**

```bash
RUST_LOG=info cargo run -- 47.6 -122.4 https://raw.githubusercontent.com/Agilefreaks/test_oop/master/coffee_shops.csv
```

**Debug level (verbose):**

```bash
RUST_LOG=debug cargo run -- 47.6 -122.4 https://raw.githubusercontent.com/Agilefreaks/test_oop/master/coffee_shops.csv
```

**Error level only:**

```bash
RUST_LOG=error cargo run -- 47.6 -122.4 https://raw.githubusercontent.com/Agilefreaks/test_oop/master/coffee_shops.csv
```

## Expected Output

### Without Logging

```
Starbucks Seattle2,0.0645
Starbucks Seattle,0.0861
Starbucks SF,10.0793
```

### With Info Logging

```
[2026-02-15T10:30:45Z INFO  af_coding_challenge] Starting coffee shop finder application
[2026-02-15T10:30:45Z INFO  af_coding_challenge] User coordinates: y=47.6, x=-122.4
[2026-02-15T10:30:45Z INFO  af_coding_challenge] Data source URL: https://raw.githubusercontent.com/Agilefreaks/test_oop/master/coffee_shops.csv
[2026-02-15T10:30:45Z INFO  af_coding_challenge::helpers] File 'coffee_shops.csv' already exists, skipping download
[2026-02-15T10:30:45Z INFO  af_coding_challenge::helpers] Processing 6 coffee shop records
[2026-02-15T10:30:45Z INFO  af_coding_challenge::helpers] Successfully calculated distances for 6 coffee shops
[2026-02-15T10:30:45Z INFO  af_coding_challenge] Found 6 coffee shops, displaying top 3 closest
Starbucks Seattle2,0.0645
Starbucks Seattle,0.0861
Starbucks SF,10.0793
[2026-02-15T10:30:45Z INFO  af_coding_challenge] Application completed successfully
```

### With Debug Logging

Includes all info-level logs plus detailed distance calculations for each coffee shop.

## Input Format

- **Y Coordinate**: Latitude (e.g., 47.6)
- **X Coordinate**: Longitude (e.g., -122.4)
- **CSV URL**: Network location of the coffee shop data file

## Output Format

The program outputs three lines, each containing:

```
<coffee_shop_name>,<distance>
```

Where:

- `coffee_shop_name` is the name from the CSV file
- `distance` is the Euclidean distance rounded to 4 decimal places

## Testing

Run unit tests:

```bash
cargo test
```

Run unit tests with output:

```bash
cargo test -- --nocapture
```

Run integration tests only:

```bash
cargo test --test integration_test
```

## Error Handling

The program will exit with an error message if:

- Incorrect number of arguments provided
- Invalid coordinates (non-numeric values)
- Network errors (unable to download file)
- Malformed CSV entries (missing or invalid data)
- File read/write errors

## CSV File Format

The CSV file should contain comma-separated values with no header:

```
Name,Y Coordinate,X Coordinate
```

**Example:**

```
Starbucks Seattle,47.5809,-122.3160
Starbucks SF,37.5209,-122.3340
```
