use std::env;
// use std::fmt::format;
use reqwest::blocking::{get, Client};
use reqwest::header::AUTHORIZATION;
use reqwest::Error;

use std::time::{Duration, Instant};
// use serde_json::Value;

mod api_struct;
use api_struct::{get_orderbook_data_api_body::*, market_inventory_api_response::*,open_interest_api_response::*, orderbook_data_api_response::*, price_info_api_response::GetMarketPriceInfo, market_stats_api_response::*};
use api_struct::{market_trades_api_response::*, market_klines_api_response::*, account_balance_api_response::*, account_history_api_response::*, order_details::*, account_info_api_response::*};
use api_struct::{account_trades_api_response::*, settled_trades_api_response::*, pending_orders_api_response::*};

const DATA_API_ENDPOINT: &str = "https://data-api.hibachi.xyz/";
const ACCOUNT_API_ENDPOINT: &str = "https://api.hibachi.xyz/";
const SYMBOL: &str = "ETH/USDT-P";

fn main() {
    let _hibachi_api_key: String = env::var("HIBACHI_API_KEY").expect("HIBACHI_API_KEY not set");
    let _hibachi_private_key: String = env::var("HIBACHI_PRIVATE_KEY").expect("HIBACHI_PRIVATE_KEY not set");
    let _hibachi_account_id: String = env::var("HIBACHI_ACCOUNT_ID").expect("HIBACHI_ACCOUNT_ID not set");

    let _response = get_market_inventory();
    let _response = get_order_book(1, 0.01);
    let _response = get_open_interest();
    let _response = get_market_price_info();
    let _response = get_market_stats();
    let _response = get_market_trades();
    let _response = get_market_klines("1h".to_owned(), None, None);
    let _response = get_account_balance(&_hibachi_account_id, &_hibachi_api_key);
    let _response = get_account_history(&_hibachi_account_id, &_hibachi_api_key);
    let _response = get_account_info(&_hibachi_account_id, &_hibachi_api_key);
    let _response = get_account_trades(&_hibachi_account_id, &_hibachi_api_key);
    let _response = get_setttled_trades(&_hibachi_account_id, &_hibachi_api_key);
    let _response = get_pending_orders(&_hibachi_account_id, &_hibachi_api_key);

    // let _response = place_order(order_details);
}

fn get_pending_orders(account_id: &String, hibachi_api_key: &String) -> Result<(), Error> {
    let mut url: String = ACCOUNT_API_ENDPOINT.to_owned();
    let url_appendage: &str = "trade/orders";
    url.push_str(url_appendage);

    let user_string = format!("?accountId={}", account_id);
    url.push_str(&user_string);

    println!("{}", url);

    let client: Client = Client::new();

    let response = client
        .get(url) // GET request with query parameter
        .header(AUTHORIZATION, hibachi_api_key) // Authorization header
        .send()?; // Send the request

    type GetPendingOrders = Vec<PendingOrders>;

    let _parsed_struct: GetPendingOrders = serde_json::from_str(&response.text()?).expect("Failed to parse JSON");
    println!("{:?}", _parsed_struct);

    Ok(())
}


fn get_setttled_trades(account_id: &String, hibachi_api_key: &String) -> Result<(), Error> {
    let mut url: String = ACCOUNT_API_ENDPOINT.to_owned();
    let url_appendage: &str = "trade/account/settlements_history";
    url.push_str(url_appendage);

    let user_string = format!("?accountId={}", account_id);
    url.push_str(&user_string);

    println!("{}", url);

    let client: Client = Client::new();

    let response = client
        .get(url) // GET request with query parameter
        .header(AUTHORIZATION, hibachi_api_key) // Authorization header
        .send()?; // Send the request

    let _parsed_struct: GetSettledTrades = serde_json::from_str(&response.text()?).expect("Failed to parse JSON");
    // println!("{:?}", _parsed_struct);

    Ok(())
}

fn get_account_trades(account_id: &String, hibachi_api_key: &String) -> Result<(), Error> {
    let mut url: String = ACCOUNT_API_ENDPOINT.to_owned();
    let url_appendage: &str = "trade/account/trades";
    url.push_str(url_appendage);

    let user_string = format!("?accountId={}", account_id);
    url.push_str(&user_string);

    println!("{}", url);

    let client: Client = Client::new();

    let response = client
        .get(url) // GET request with query parameter
        .header(AUTHORIZATION, hibachi_api_key) // Authorization header
        .send()?; // Send the request

    // println!("{:?}", response.status());
    // println!("{:?}", response.headers());
    // println!("{:?}", response.text());

    let _parsed_struct: GetAccountTrades = serde_json::from_str(&response.text()?).expect("Failed to parse JSON");
    println!("{:?}", _parsed_struct);

    Ok(())
}

// https://api.hibachi.xyz/trade/account/trades?accountId=<accountId>

fn get_account_info(account_id: &String, hibachi_api_key: &String) -> Result<(), Error> {
    let mut url: String = ACCOUNT_API_ENDPOINT.to_owned();
    let url_appendage: &str = "trade/account/info";
    url.push_str(url_appendage);

    let user_string = format!("?accountId={}", account_id);
    url.push_str(&user_string);

    println!("{}", url);

    let client: Client = Client::new();

    let response = client
        .get(url) // GET request with query parameter
        .header(AUTHORIZATION, hibachi_api_key) // Authorization header
        .send()?; // Send the request

    // println!("{:?}", response.status());
    // println!("{:?}", response.headers());
    // println!("{:?}", response.text());

    let _parsed_struct: GetAccountInfo = serde_json::from_str(&response.text()?).expect("Failed to parse JSON");
    // println!("{:?}", _parsed_struct);

    Ok(())
}

// fn place_order(order_details: OrderDetails) {
    
// }


fn get_account_history(account_id: &String, hibachi_api_key: &String) -> Result<(), Error> {
    let mut url: String = ACCOUNT_API_ENDPOINT.to_owned();
    let url_appendage: &str = "capital/history";
    url.push_str(url_appendage);

    let user_string = format!("?accountId={}", account_id);
    url.push_str(&user_string);

    println!("{}", url);

    let client: Client = Client::new();

    let response = client
        .get(url) // GET request with query parameter
        .header(AUTHORIZATION, hibachi_api_key) // Authorization header
        .send()?; // Send the request

    // println!("{:?}", response.text()?);
    let _parsed_struct: GetAccountHistory = serde_json::from_str(&response.text()?).expect("Failed to parse JSON");
    // println!("{:?}", _parsed_struct);

    Ok(())
}


fn get_account_balance(account_id: &String, hibachi_api_key: &String) -> Result<(), Error> {
    let mut url: String = ACCOUNT_API_ENDPOINT.to_owned();
    let url_appendage: &str = "capital/balance";
    url.push_str(url_appendage);

    let user_string = format!("?accountId={}", account_id);
    url.push_str(&user_string);

    println!("{}", url);

    let client: Client = Client::new();

    let response = client
        .get(url) // GET request with query parameter
        .header(AUTHORIZATION, hibachi_api_key) // Authorization header
        .send()?; // Send the request

    // println!("Response Body: {}", response.text()?);

    // Check that the response was successful
    // if response.status().is_success() {
    //     let response_body = response.text()?;
    //     println!("Response: {}", response_body);
    // } else {
    //     println!("Error: {}", response.status());
    // }

    // println!("{}", response);
    let _parsed_struct: GetAccountBalance = serde_json::from_str(&response.text()?).expect("Failed to parse JSON");

    // println!("{:?}", _parsed_struct);
    
    Ok(())
}

fn get_market_klines(interval: String, from_ms: Option<u32>, to_ms: Option<u32>)  -> Result <(), Error> {
    let mut url: String = DATA_API_ENDPOINT.to_owned();
    let url_appendage: &str = "market/data/klines";
    url.push_str(url_appendage);

    let user_string = format!("?symbol={}&interval={}", SYMBOL, interval);
    url.push_str(&user_string);

    match from_ms {
        Some(value) => url.push_str(&(format!("?fromMs={}", value))),
        None => ()
    }
    match to_ms {
        Some(value) => url.push_str(&(format!("?toMs={}", value))),
        None => ()
    }

    println!("{}", url);

    // Send a GET request
    let response: String = get(url)?
        .text()?;

    // println!("{}", response);
    let _parsed_struct: GetMarketKlinesInfo = serde_json::from_str(&response).expect("Failed to parse JSON");

    // println!("{:?}", _parsed_struct);

    Ok(())
}


fn get_market_trades() -> Result<(), Error> {
    let mut url: String = DATA_API_ENDPOINT.to_owned();
    let url_appendage: &str = "market/data/trades";
    url.push_str(url_appendage);

    println!("{}", url);

    let user_string = format!("?symbol={}", SYMBOL);
    url.push_str(&user_string);

    let response = get(url)?
        .text()?;

    let _parsed_struct: GetMarketTradesInfo = serde_json::from_str(&response).expect("failed to parse JSON");
    // println!("Response = {:?}", _parsed_struct);

    Ok(())

}

fn get_market_stats() -> Result<(), Error> {
    let mut url: String = DATA_API_ENDPOINT.to_owned();
    let url_appendage: &str = "market/data/stats";
    url.push_str(url_appendage);

    println!("{}", url);

    let user_string = format!("?symbol={}", SYMBOL);
    url.push_str(&user_string);

    let response = get(url)?
        .text()?;

    let _parsed_struct: GetMarketStatsInfo = serde_json::from_str(&response).expect("failed to parse JSON");
    // println!("Response = {:?}",_parsed_struct);

    Ok(())

}

fn get_market_price_info() -> Result<(), Error> {
    let mut url: String = DATA_API_ENDPOINT.to_owned();
    let url_appendage: &str = "market/data/prices";
    url.push_str(url_appendage);

    // println!("{}", url);

    let user_string = format!("?symbol={}", SYMBOL);
    url.push_str(&user_string);

    let response = get(url)?
        .text()?;

    let _parsed_struct: GetMarketPriceInfo = serde_json::from_str(&response).expect("failed to parse JSON");
    // println!("Response = {:?}", _parsed_struct);

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

    // println!("{:?}", _parsed_struct);
    Ok(())
    
}

fn get_order_book(ob_depth: u32, ob_granularity: f64)  -> Result <(), Error> {
    let mut url: String = DATA_API_ENDPOINT.to_owned();
    let url_appendage: &str = "market/data/orderbook";
    url.push_str(url_appendage);

    let body = GetOrderbookDataInput {
        symbol: SYMBOL.to_owned(),
        depth: ob_depth,
        granularity: ob_granularity.to_string(),
    };

    let user_string = format!("?symbol={}&depth={}&granularity={}", body.symbol, body.depth, body.granularity);
    url.push_str(&user_string);

    println!("{}", url);

    // let start_time: Instant = Instant::now();
    // Send a GET request
    let response: String = get(url)?
        .text()?;

    //{"ask":{"endPrice":"2807.88","levels":[{"price":"2807.88","quantity":"4.541690338"}],"startPrice":"2807.88"},"bid":{"endPrice":"2806.76","levels":[{"price":"2806.76","quantity":"1.410000000"}],"startPrice":"2806.76"}}

    // println!("{}", response);
    let _parsed_struct: GetOrderbookData = serde_json::from_str(&response).expect("Failed to parse JSON");

    // let elapsed: Duration = start_time.elapsed();    
    // println!("Time taken to deserialize and parse: {:?}", elapsed);

    // println!("{:?}", _parsed_struct);

    Ok(())
}


fn get_market_inventory() -> Result <(), Error> {
    let mut url: String = DATA_API_ENDPOINT.to_owned();
    let url_appendage: &str = "market/inventory";
    url.push_str(url_appendage);

    println!("{}", url);

    // let start_time: Instant = Instant::now();
    // Send a GET request
    let response: String = get(url)?
        .text()?;    

    // let elapsed: Duration = start_time.elapsed();    
    // println!("Time taken for GET request: {:?}", elapsed);

    // let start_time: Instant = Instant::now();

    let _parsed_struct: MarketInventory = serde_json::from_str(&response).expect("Failed to parse JSON");

    // let elapsed: Duration = start_time.elapsed();    
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