use std::fs::File;
use std::io::{self, Cursor, Write};
use zip::write::{FileOptions, ZipWriter};
use serde_json::to_string;
use serde::Serialize;

// Structure for metadata to include in the ZIP
#[derive(Serialize)]
struct AdMetadata {
    ad_file: String,
    target_link: String,
    label: String,
}

// Function to generate a sample ZIP file
fn generate_sample_zip() -> io::Result<()> {
    // Create a file to write the ZIP to
    println!("Attempting to create sample_ads.zip...");
    let file = match File::create("sample_ads.zip") {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to create file: {}", e);
            return Err(e);
        }
    };
    let mut zip = ZipWriter::new(file);

    // Options for ZIP file entries
    let options:FileOptions<'_,()> = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    // Sample ad files (mock JPEG content)
    let ad_files = vec![
        ("ads_entertainment_games_freefire.jpeg", "entertainment"),
        ("ads_fashion_clothing_trendy.jpeg", "fashion"),
        ("ads_travel_beach_resort.jpeg", "travel"),
    ];

    // Add ad files to the ZIP
    println!("Adding ad files to ZIP...");
    for (filename, label) in &ad_files {
        println!("Adding file: {}", filename);
        zip.start_file(filename, options)?;
        let mock_jpeg = b"\xFF\xD8\xFF\xE0\x00\x10JFIF\x00\x01"; // Mock JPEG start marker
        zip.write_all(mock_jpeg)?;
    }

    // Create metadata
    let metadata: Vec<AdMetadata> = ad_files.into_iter().enumerate().map(|(i, (filename, label))| AdMetadata {
        ad_file: filename.to_string(),
        target_link: format!("https://example.com/ad/{}", i + 1),
        label: label.to_string(),
    }).collect();

    // Serialize metadata to JSON
    println!("Serializing metadata...");
    let metadata_json = match to_string(&metadata) {
        Ok(json) => json,
        Err(e) => {
            println!("Failed to serialize metadata: {}", e);
            return Err(io::Error::new(io::ErrorKind::Other, e));
        }
    };

    // Add metadata.json to the ZIP
    println!("Adding metadata.json to ZIP...");
    zip.start_file("metadata.json", options)?;
    zip.write_all(metadata_json.as_bytes())?;

    // Finish the ZIP file
    println!("Finalizing ZIP...");
    zip.finish()?;
    println!("Successfully generated sample_ads.zip");
    Ok(())
}

fn main() {
    match generate_sample_zip() {
        Ok(()) => println!("ZIP generation completed successfully"),
        Err(e) => eprintln!("Error generating ZIP: {}", e),
    }
}