use std::env;
// use std::fmt::format;
use reqwest::blocking::get;
use reqwest::Error;

use std::time::{Duration, Instant};
// use serde_json::Value;

mod api_struct;
use api_struct::{get_orderbook_data_api_body::*, market_inventory_api_response::*,open_interest_api_response::*, orderbook_data_api_response::*, price_info_api_response::GetMarketPriceInfo};

const DATA_API_ENDPOINT: &str = "https://data-api.hibachi.xyz/";
const SYMBOL: &str = "ETH/USDT-P";

fn main() {
    let _hibachi_api_key: String = env::var("HIBACHI_API_KEY").expect("HIBACHI_API_KEY not set");
    let _hibachi_private_key: String = env::var("HIBACHI_PRIVATE_KEY").expect("HIBACHI_PRIVATE_KEY not set");

    let _response = get_market_inventory();
    let _response = get_order_book(1, 0.01);
    let _response = get_open_interest();
    let _response = get_market_price_info();
}

// https://data-api.hibachi.xyz/market/data/prices?symbol=ETH/USDT-P
fn get_market_price_info() -> Result<(), Error> {
    let mut url: String = DATA_API_ENDPOINT.to_owned();
    let url_appendage: &str = "market/data/prices";
    url.push_str(url_appendage);

    println!("{}", url);

    let user_string = format!("?symbol={}", SYMBOL);
    url.push_str(&user_string);

    let response = get(url)?
        .text()?;

    let parsed_struct: GetMarketPriceInfo = serde_json::from_str(&response).expect("failed to parse JSON");
    println!("Response = {:?}", parsed_struct);

    Ok(())
}

fn get_open_interest() -> Result <(), Error> {
    let mut url: String = DATA_API_ENDPOINT.to_owned();
    let url_appendage: &str = "market/data/open-interest";
    url.push_str(url_appendage);

    println!("{}", url);

    let user_string = format!("?symbol={}", SYMBOL);
    url.push_str(&user_string);

    let response = get(url)?
        .text()?;

    let _parsed_struct: GetOpenInterest = serde_json::from_str(&response).expect("Failed to parse JSON");

    println!("{:?}", _parsed_struct);
    Ok(())
    
}

fn get_order_book(ob_depth: u32, ob_granularity: f64)  -> Result <(), Error> {
    let mut url: String = DATA_API_ENDPOINT.to_owned();
    let url_appendage: &str = "market/data/orderbook";
    url.push_str(url_appendage);

    // https://data-api.hibachi.xyz/market/data/orderbook?symbol=ETH/USDT-P&depth=3&granularity=0.01

    let body = GetOrderbookDataInput {
        symbol: SYMBOL.to_owned(),
        depth: ob_depth,
        granularity: ob_granularity.to_string(),
    };

    let user_string = format!("?symbol={}&depth={}&granularity={}", body.symbol, body.depth, body.granularity);
    url.push_str(&user_string);

    // println!("{}", url);

    let start_time: Instant = Instant::now();
    // Send a GET request
    let response: String = get(url)?
        .text()?;

    //{"ask":{"endPrice":"2807.88","levels":[{"price":"2807.88","quantity":"4.541690338"}],"startPrice":"2807.88"},"bid":{"endPrice":"2806.76","levels":[{"price":"2806.76","quantity":"1.410000000"}],"startPrice":"2806.76"}}

    // println!("{}", response);
    let _parsed_struct: GetOrderbookData = serde_json::from_str(&response).expect("Failed to parse JSON");

    let elapsed: Duration = start_time.elapsed();    
    // println!("Time taken to deserialize and parse: {:?}", elapsed);

    // println!("{:?}", _parsed_struct);

    Ok(())
}


fn get_market_inventory() -> Result <(), Error> {
    let mut url: String = DATA_API_ENDPOINT.to_owned();
    let url_appendage: &str = "market/inventory";
    url.push_str(url_appendage);

    // println!("{}", url);

    let start_time: Instant = Instant::now();
    // Send a GET request
    let response: String = get(url)?
        .text()?;    

    let elapsed: Duration = start_time.elapsed();    
    // println!("Time taken for GET request: {:?}", elapsed);

    let start_time: Instant = Instant::now();

    let _parsed_struct: MarketInventory = serde_json::from_str(&response).expect("Failed to parse JSON");

    let elapsed: Duration = start_time.elapsed();    
    // println!("Time taken to deserialize and parse: {:?}", elapsed);

    // println!("{:?}", _parsed_struct);

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_deserialize() {
//         let msg = "{"ask":{"endPrice":"2806.35","levels":[{"price":"2806.35","quantity":"0.350000000"}],"startPrice":"2806.35"},"bid":{"endPrice":"2805.30","levels":[{"price":"2805.30","quantity":"0.290000000"}],"startPrice":"2805.30"}}%";

//     }
// }