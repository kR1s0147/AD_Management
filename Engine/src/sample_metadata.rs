use std::fs::{self, File};
use std::io::{self, Write};
use zip::write::{FileOptions, ZipWriter};
use serde::{Serialize, Deserialize};
use serde_json::to_string;
use std::env;
use derive_more::Display;

// Metadata structure for each ad
mod db;

use db::get_data;
use db::put_data;
use db::delete_data;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize,Clone,Display)]
#[display("AdMetadata {{ target_link: {}, ad_file: {}, advertiser: {} }}", target_link, ad_file, advertiser)]
pub struct AdMetadata {
    pub target_link: String,
    pub ad_file: String,
    pub advertiser: String,
}

fn main() -> io::Result<()> {
    // Hardcoded user ID (replace with dynamic value if needed) 
    let user_id = "0x5A2d04E974f4885756a1C4a33E36095Dd6aa7A01".to_string();
    let path = Path::new("../ADs/"); 
    let current_dir = env::current_dir().unwrap();
    let ad_dir = current_dir.join(path).canonicalize().unwrap();
    // Sample labels (replace with actual labels) 
    if let Ok(entries) = std::fs::read_dir(&ad_dir) {  
    for entry in entries.flatten() {
        let path = entry.path();
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            let meta =  AdMetadata {
                target_link: "https://www.amazon.com/".to_string(),
                ad_file: filename.to_string(),
                advertiser: user_id.clone(),
            };
            let data:Vec<u8> = serde_json::to_vec(&meta).expect("Failed to serialize metadata");
            let res = put_data(filename.to_string(),data).expect("Failed to put data");
            println!("Metadata saved for file: {}", filename);
            println!("Metadata: {:?}",meta);  
            }
        }
    }
    Ok(())
}