use std::fs;
use std::process::Command;

#[test]
fn test_full_program_flow() {
    let test_csv = "test_coffee_shops.csv";
    let csv_content = "Starbucks Seattle,47.5809,-122.3160\n\
                       Starbucks SF,37.5209,-122.3340\n\
                       Starbucks Seattle2,47.5869,-122.3368";

    fs::write(test_csv, csv_content).unwrap();

    let output = Command::new("cargo")
        .args(&["run", "--", "47.6", "-122.4", test_csv])
        .output()
        .expect("Failed to execute program");

    // Clean up
    fs::remove_file(test_csv).ok();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Starbucks Seattle2"));
    assert!(stdout.contains("0.0645"));
}

#[test]
fn test_invalid_coordinates() {
    let output = Command::new("cargo")
        .args(&["run", "--", "random_invalid_number", "-122.4", "dummy.csv"])
        .output()
        .expect("Failed to execute program");

    assert!(!output.status.success());
}

#[test]
fn test_missing_arguments() {
    let output = Command::new("cargo")
        .args(&["run", "--", "47.6", "-122.4"])
        .output()
        .expect("Failed to execute program");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage"));
}
