use reqwest::Client;
use std::fs::File;
use std::io::{self, Read, Write,copy};
use zip::read::ZipArchive;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get command-line arguments
    let url = "http://127.0.0.1:8081/submit_search_history";

    // Read data.json
    let mut file = File::open("data.json")?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;
    println!("Read JSON data: {}", json_data);

    // Create HTTP client
    let client = Client::new();

    // Send POST request
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("User","0xF6E6B3549a463d21229a7Ceb2E1e2FaEC9b1cCBE")
        .body(json_data)
        .send()
        .await?;
    println!("Response: {:?}", response);
    // Check response status
    if response.status().is_success() {
        // println!("Request successful, saving response..."
        // println!("data: {:?}", response);   
        let mut file = File::create("ads.zip")?;
        let mut content = response.bytes().await?;
        copy(&mut content.as_ref(), &mut file)?;
        println!("Ads saved to ads.zip");
       
    } else {
        println!("Request failed with status: {}", response.status());
        let error_text = response.text().await?;
        println!("Error response: {}", error_text);
    }

    Ok(())
}