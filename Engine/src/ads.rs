use url::Url;
use std::path::Path;
use std::env;
use std::fs;
use serde::{Serialize, Deserialize};
use derive_more::Display;
use regex::Regex;
use crate::AdMetadata;

use crate::db;
use db::get_data;
use db::delete_data;
use db::put_data;

pub async fn recommend_ads(labels: Vec<String>) -> Result<Vec<AdMetadata>, Box<dyn std::error::Error>> {
    let mut ad_files: Vec<AdMetadata> = Vec::new(); // (filename, label)
    let path = Path::new("../ADs/"); // Directory where ad files are stored
    let current_dir = env::current_dir().unwrap();
    // Resolve the relative path against the current directory
    let ad_dir = current_dir.join(path).canonicalize().unwrap();
    let patterns: Vec<Regex> = labels.iter()
        .map(|label| Regex::new(&format!(r"(?i){}", regex::escape(label)))
            .map_err(|e| format!("Invalid regex for label {}: {}", label, e)))
        .collect::<Result<Vec<_>, _>>()?;

        if let Ok(entries) = std::fs::read_dir(&ad_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                        for  pattern in patterns.iter() {
                            if pattern.is_match(filename) {
                                let meta = get_data(filename.to_string()).expect("Failed to get data");
                                let metadata: AdMetadata = serde_json::from_slice(&meta).expect("Failed to deserialize metadata");
                                ad_files.push(metadata);
                                break; // Take the first matching label
                            }
                        }
                    }
                }
            }

    // Ensure exactly 3 ads
        if let Ok(entries) = fs::read_dir(&ad_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    let meta = get_data(filename.to_string())?;
                    let metadata: AdMetadata = serde_json::from_slice(&meta)?;
                    ad_files.push(metadata);
                }
                if ad_files.len() >= 3 {
                    break; // Stop adding more ads if we have enough
                }
            }
        }
    ad_files.truncate(3);
    Ok(ad_files)
}