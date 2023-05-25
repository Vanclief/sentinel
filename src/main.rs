use std::fs;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use rxing;

fn main() {
    let dir_path = "./tmp";

    println!("Scanning directory: {}", dir_path);

    loop {
        scan_directory(dir_path);
        thread::sleep(Duration::from_millis(100));
    }
}

fn scan_directory(dir_path: &str) -> Result<(), std::io::Error> {
    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        if let Some(extension) = path.extension() {
            if ["jpg", "jpeg", "png", "gif"].contains(&extension.to_str().unwrap_or_default()) {
                match path.to_str() {
                    Some(file_name) => {
                        let now = Instant::now();
                        detect_barcodes(file_name);
                        let elapsed = now.elapsed();
                        println!("Scanned file: {} took {:.2?}", file_name, elapsed);
                        fs::remove_file(file_name)?;
                    }
                    _ => (),
                }
            }
        }
    }

    Ok(())
}

fn detect_barcodes(file_name: &str) {
    let detect_results = rxing::helpers::detect_multiple_in_file(file_name);
    match detect_results {
        Ok(results) => print_results(results),
        Err(e) => println!("Error: {}", e),
    };
}

fn print_results(results: Vec<rxing::RXingResult>) {
    for result in results {
        print_result(result);
    }
}

fn print_result(result: rxing::RXingResult) {
    println!("{} -> {}", result.getBarcodeFormat(), result.getText())
}
