use std::env;
use reqwest::blocking::get;
use reqwest::Error;
use std::time::{Duration, Instant};
use serde_json::Value;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
struct MyObject {
    level: i32,                    // Assuming level is of type i32
    lower_threshold: String,       // Lower threshold as a String
    title: String,                 // Title as a String
    upper_threshold: String,       // Upper threshold as a String
}

#[derive(Debug, Serialize, Deserialize)]
struct MyData {
    data: Option <Vec<MyObject>>,  // Optionally contains a vector of MyObject
}


const DATA_API_ENDPOINT: &str = "https://data-api.hibachi.xyz/";

fn main() {
    let _hibachi_api_key: String = env::var("HIBACHI_API_KEY").expect("HIBACHI_API_KEY not set");
    let _hibachi_private_key: String = env::var("HIBACHI_PRIVATE_KEY").expect("HIBACHI_PRIVATE_KEY not set");

    let start_time: Instant = Instant::now();
    let response = get_market_inventory();
    let elapsed: Duration = start_time.elapsed();
    println!("Response = {:?}, time taken = {:?}", response, elapsed);
}

fn get_market_inventory() -> Result <(), Error> {
    let mut url: String = DATA_API_ENDPOINT.to_owned();
    let url_appendage: &str = "market/inventory";
    url.push_str(url_appendage);

    println!("{}", url);
    // Send a GET request
    let response: String = get(url)?
        .text()?;
    
    let parsed_value: Value = serde_json::from_str(&response).expect("Failed to parse JSON");

    for key in parsed_value.as_object().unwrap().keys(){
        println!("{}", key);
    }

    Ok(())
}


