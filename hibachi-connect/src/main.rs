use std::env;
use reqwest::blocking::get;
use reqwest::Error;
use std::time::{Duration, Instant};
// use serde_json::Value;

mod api_response;
use api_response::api_response::*;

const DATA_API_ENDPOINT: &str = "https://data-api.hibachi.xyz/";

fn main() {
    let _hibachi_api_key: String = env::var("HIBACHI_API_KEY").expect("HIBACHI_API_KEY not set");
    let _hibachi_private_key: String = env::var("HIBACHI_PRIVATE_KEY").expect("HIBACHI_PRIVATE_KEY not set");

    let _response = get_market_inventory();
    // let _response = get_order_book();
}

// fn get_order_book()  -> Result <(), Error> {
//     let mut url: String = DATA_API_ENDPOINT.to_owned();
//     let url_appendage: &str = "/market/data/orderbook";
//     url.push_str(url_appendage);

//     println!("{}", url);
//     Ok(())
// }


fn get_market_inventory() -> Result <(), Error> {
    let mut url: String = DATA_API_ENDPOINT.to_owned();
    let url_appendage: &str = "market/inventory";
    url.push_str(url_appendage);

    println!("{}", url);

    let start_time: Instant = Instant::now();
    // Send a GET request
    let response: String = get(url)?
        .text()?;    

    let elapsed: Duration = start_time.elapsed();    
    println!("Time taken for GET request: {:?}", elapsed);

    let start_time: Instant = Instant::now();

    let parsed_struct: MarketInventory = serde_json::from_str(&response).expect("Failed to parse JSON");

    let elapsed: Duration = start_time.elapsed();    
    println!("Time taken to deserialize and parse: {:?}", elapsed);

    println!("{:?}", parsed_struct);

    // for key in parsed_struct.crossChainAssets{
    //     println!("Cross Chain Assets: \n {:?}", key);
    // }

    // println!("feeConfig: \n {:?}", parsed_struct.feeConfig);

    // for key in parsed_struct.markets{
    //     println!("Markets Data: \n {:?}", key);
    // }

    // for key in parsed_struct.tradingTiers{
    //     println!("Trading Tiers: \n {:?}", key);
    // }

    Ok(())
}


