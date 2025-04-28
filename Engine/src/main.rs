use tokio::net::TcpListener;
use tokio::sync::{Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use axum::{
    body::Body, http::{header,StatusCode,HeaderMap}, response::{IntoResponse, Response}, routing::post, Json, Router,
    extract::{State,Path,Multipart},
};
use serde_json::{Value,to_string,json};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::collections::HashMap;
use bytes::Bytes;
use eyre::Result;
use alloy::{primitives::Address};
use url::Url;
use sha2::{Sha256,Digest};
use tokio::fs::File;
use zip::{write::{FileOptions, ZipWriter,ExtendedFileOptions},CompressionMethod};
use std::time::Duration;
use std::{path::Path as SysPath,env};
use std::io::{Cursor,Write};
use derive_more::Display;
use tower_http::cors::{CorsLayer,Any,AllowOrigin};
use reqwest::header::{ CONTENT_TYPE};
use reqwest::Method;

mod db;
mod ads;
mod transactions;
use ads::recommend_ads as recommend_ads;
use transactions::update_transaction as update_transaction;

#[derive(Clone)]
pub struct Appstate{
    pub RewardPoints : HashMap<String,u32>,
    pub ADVendors_Credit :HashMap<String,u32>,
    pub ADVendors : Vec<String>,
    pub users : Vec<String>,
}
#[derive(Debug, Serialize, Deserialize,Clone,Display)]
#[display("AdMetadata {{ target_link: {}, ad_file: {}, advertiser: {} }}", target_link, ad_file, advertiser)]
pub struct AdMetadata {
    pub target_link: String,
    pub ad_file: String,
    pub advertiser: String,
}


#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Labels{
    labels: Vec<String>,
}

async fn submit_search_history(
    header:HeaderMap,
    State(state):State<Arc<Mutex<Appstate>>>,
    Json(data):Json<Value>
) ->  impl IntoResponse {
    // println!("{:?}",data);
    let mut input_data =  serde_json::to_string(&data).unwrap();
    // println!("{}",input_data);
    let user = header.get("User")
        .and_then(|value| value.to_str().ok())
        .map(String::from).ok_or("Missing User header")
        .expect("Missing User header");
    {   
        let mut _state = state.lock().await;
        let mut rewardpoints = &mut _state.RewardPoints;
        let entry = rewardpoints.entry(user.to_string()).or_insert(0);
        *entry += 3;
        let mut users = &mut _state.users;
        users.push(user.to_string());
    }
    let path = SysPath::new("../ADs/"); 
    let current_dir = env::current_dir().unwrap();
    let ad_dir = current_dir.join(path).canonicalize().unwrap();


    let client = Client::new();

    let response =  client
        .post("http://localhost:8000/analyze") 
        .header("Content-Type", "application/json")
        .body(input_data)
        .send()
        .await;

    // println!("{:?}", response);
    let res = response.expect("error while sending request");

    println!("{:?}", res);

    let mut la:Labels = Labels{
        labels: Vec::new(),
    };
    if res.status().is_success() {
        la = res.json().await.expect("error while deserializing");
        println!("Received labels: {:?}", la);
    } 
    else{
        println!("Failed to get labels from server: {}", res.status());
    }

    let mut ads: Vec<AdMetadata> = Vec::new();
    ads = recommend_ads(la.labels).await.expect("error while getting ads");

    let mut buffer = Cursor::new(Vec::new());
    {
        let mut zip = ZipWriter::new(&mut buffer);
        let options = FileOptions::<()>::default();
        for ad in &ads {
            zip.start_file(ad.ad_file.clone(), options).unwrap();
            let ad_path = format!("{}/{}", ad_dir.display(), ad.ad_file);
            let ad_data = std::fs::read(&ad_path).expect("Failed to read ad file");
            zip.write_all( &ad_data).expect("Failed to write ad file to zip");
            {
            let mut _state= state.lock().await;
            let entry =  &mut _state.ADVendors_Credit.entry(ad.advertiser.clone()).or_insert(0);
            **entry += 1;
            &mut _state.ADVendors.push(ad.advertiser.clone());
            }
        }
        let metadata_json = to_string(&ads).expect("Failed to serialize metadata");
        zip.start_file("metadata.json", options).expect("Failed to start metadata file");;
        zip.write_all( metadata_json.as_bytes()).expect("Failed to write metadata to zip");
        zip.finish().expect("Failed to finish zip");
    }
    let zip_bytes = buffer.into_inner();
    
    println!("Request successful, returning ads as a ZIP file");
    Response::builder()
    .status(StatusCode::OK)
    .header(header::CONTENT_TYPE, "application/zip")
    .header(header::CONTENT_DISPOSITION, "attachment; filename=\"ads.zip\"")
    .body(Body::from(zip_bytes))
    .unwrap()
}



async fn post_ad(
    State(state): State<Arc<Mutex<Appstate>>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut ad_metadata: Option<AdMetadata> = None;
    let mut file_name: Option<String> = None;
    let path = SysPath::new("../ADs/"); 
    let current_dir = env::current_dir().unwrap();
    let ad_dir = current_dir.join(path).canonicalize().unwrap();
    // Process multipart form data
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        if name == "metadata" {
            // Deserialize metadata from JSON string
            let metadata_bytes = field.bytes().await.unwrap();
            ad_metadata = serde_json::from_slice(&metadata_bytes).ok();
        } else if name == "ad_file" {
            // Save ad file to disk
            let original_file_name = field.file_name().unwrap().to_string();

            let file_path = format!("{}{}", ad_dir.display(), original_file_name);
            let mut file = File::create(&file_path).await.unwrap();
            let file_bytes = field.bytes().await.unwrap();
            file.write_all(&file_bytes).await.unwrap();

            file_name = Some(file_path.clone());

        }
    }

    // Check if file and metadata are valid
    if let (Some(metadata), Some(file_name)) = (ad_metadata, file_name) {
        println!(
            "Received ad: {}, from advertiser: {}",
            metadata.ad_file, metadata.advertiser
        );

        Json(json!({
            "status": "Ad uploaded successfully",
            "file": file_name,
            "metadata": metadata
        }))
    } else {
        Json(json!({
            "error": "Invalid file or metadata."
        }))
    }
}



#[tokio::main]
async fn main() {
    let app_state = Arc::new(Mutex::new(Appstate {
        RewardPoints :HashMap::new(),
        ADVendors_Credit : HashMap::new(),
        ADVendors : Vec::new(),
        users : Vec::new(),
    }));
    println!("Server running on http://127.0.0.1:8081/");  // Bind to 127.0.0.1:8081
    let cors = CorsLayer::new()
        // Allow requests from specific origins
        .allow_origin(AllowOrigin::any())
        // Allow credentials if needed
        .allow_credentials(false)
        // Allow specific HTTP methods
        .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
        // Allow specific headers, including the 'user' header
        .allow_headers(vec![
            header::CONTENT_TYPE,           // Default header for JSON
            header::AUTHORIZATION,          // Default for auth
            header::HeaderName::from_static("user"), // Add your custom 'user' header
        ]);

    let app = Router::new()
    .route("/submit_search_history",post(submit_search_history))
    .route("/upload_ad", post(post_ad))
    .with_state(app_state.clone())
    .layer(cors);
    let state = Arc::clone(&app_state);

    tokio::spawn(async move {
        update_transaction(state).await;
    });

    let listener  = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}