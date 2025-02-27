use std::env;
use once_cell::sync::Lazy;
// use std::fmt::format;
use reqwest::blocking::{get, Client};
use reqwest::header::AUTHORIZATION;
use reqwest::Error;

use chrono::Utc;
use std::time::{Duration, Instant};
// use serde_json::Value;

mod api_struct;
use api_struct::{get_orderbook_data_api_body::*, market_inventory_api_response::*,open_interest_api_response::*, orderbook_data_api_response::*, price_info_api_response::GetMarketPriceInfo, market_stats_api_response::*};
use api_struct::{market_trades_api_response::*, market_klines_api_response::*, account_balance_api_response::*, account_history_api_response::*, order_details::*, account_info_api_response::*};
use api_struct::{account_trades_api_response::*, settled_trades_api_response::*, pending_orders_api_response::*};

const DATA_API_ENDPOINT: &str = "https://data-api.hibachi.xyz/";
const ACCOUNT_API_ENDPOINT: &str = "https://api.hibachi.xyz/";
const SYMBOL: &str = "ETH/USDT-P";
const CONTRACT_ID: u32 = 2;

static HIBACHI_API_KEY: Lazy<String> = Lazy::new(|| {
    env::var("HIBACHI_API_KEY").expect("HIBACHI_API_KEY not set")
});

static HIBACHI_ACCOUNT_ID: Lazy<String> = Lazy::new(|| {
    env::var("HIBACHI_ACCOUNT_ID").expect("HIBACHI_ACCOUNT_ID not set")
});

#[derive(Debug, PartialEq)]
enum OrderSide {
    ASK,
    BID,
}

#[derive(Debug, PartialEq)]
enum OrderType {
    LIMIT,
    MARKET,
}

fn main() {


    // let _hibachi_api_key: String = env::var("HIBACHI_API_KEY").expect("HIBACHI_API_KEY not set");
    let _hibachi_private_key: String = env::var("HIBACHI_PRIVATE_KEY").expect("HIBACHI_PRIVATE_KEY not set");
    let _hibachi_account_id: String = env::var("HIBACHI_ACCOUNT_ID").expect("HIBACHI_ACCOUNT_ID not set");

    let _response = get_market_inventory();
    let _response = get_order_book(1, 0.01);
    let _response = get_open_interest();
    let _response = get_market_price_info();
    let _response = get_market_stats();
    let _response = get_market_trades();
    let _response = get_market_klines("1h".to_owned(), None, None);
    let _response = get_account_balance();
    let _response = get_account_history();
    let _response = get_account_info();
    let _response = get_account_trades();
    let _response = get_setttled_trades();
    let _response = get_pending_orders();
    let _response = place_order(100000.0, 0.0,OrderSide::ASK, OrderType::LIMIT);
}

fn place_order(price: f64, quantity: f64, side: OrderSide, o_type: OrderType) -> Result<(), Error> {

    // Signing Part
    // nonce: 8 bytes
    // contractId: 4 bytes
    // quantity: 8 bytes
    // side: 4 bytes
    // price: 8 bytes
    // maxFeesPercent, should be at least the returned value of /market/exchange-info endpoint. Otherwise, it will be rejected : 8 bytes
    // creationDeadline (Optional): 
    // triggerPrice (Optional)

    // Order details:
    // accountId :should be one of: LIMIT, MARKET
    // symbol: should be one of the symbol from one of the futureContracts returned by /market/exchange-info API
    // nonce: should be a unix timestamp either ms or us unique to this order
    // side: should be one of: ASK, BID
    // orderType
    // quantity
    // price
    // siganture
    // maxFeesPercent

    // "accountId": 128,
    // "symbol": "ETH/USDT-P",
    // "nonce": 1714701600000000,
    // "orderType": "LIMIT",
    // "side": "BID",
    // "quantity": "1.2",
    // "price": "3500.1",
    // "signature": "0000000000000000000000000000000000000000000000000000000000000000",
    // "maxFeesPercent": "0.00045"

    let now = Utc::now();
    let nonce: i64 = 1714701600000000; // now.timestamp_micros();

    let order_side = match side {
        OrderSide::ASK => "ASK".to_string(),
        OrderSide::BID => "BID".to_string(),
    };

    let order_type = match o_type {
        OrderType::LIMIT => "LIMIT".to_string(),
        OrderType::MARKET => "MARKET".to_string(),
    };

    // let signature = construct_signature(nonce, CONTRACT_ID, quantity, order_side, price, max_fees_percent);


    let price_f128 = (price * (1u64 << 32) as f64*10f64.powi(-4)) as u128;

    println!("price_u128={}", price_f128);

    let max_fees_percent = 0;

    let signature= format!("0x{:016x}{:08x}{:016x}{:016x}", nonce, CONTRACT_ID, price_f128, max_fees_percent);

    println!("account_id = {}, symbol = {}, nonce = {}, order_type = {}, side = {}, quantity = {}, price = {}, signature = {}, maxFeesPercent = {}, contract_id = {}", 
            *HIBACHI_ACCOUNT_ID, SYMBOL, nonce, order_type, order_side, quantity.to_string(), price.to_string(), signature, max_fees_percent, CONTRACT_ID);

    // 0xe8100c48581d7944152ba7666b4128c9fc491d30d4bd702f717477d9a6a54ae3
    // NxnTqs8U3KHwD6D6d5Lxw3XEEQiLT0z9lT_ghmC8TpQ=
    // 0x02822208f111ea2a00e06a607681123caf740ba4b65df21bcd19920727a8715d43956dc3aca1caf9ec7b3b2bc29318222bbfebbed5c608b6176617b3e849adc3

    Ok(())
}


fn get_pending_orders() -> Result<(), Error> {
    let mut url: String = ACCOUNT_API_ENDPOINT.to_owned();
    let url_appendage: &str = "trade/orders";
    url.push_str(url_appendage);

    let user_string = format!("?accountId={}", *HIBACHI_ACCOUNT_ID);
    url.push_str(&user_string);

    println!("{}", url);

    let client: Client = Client::new();

    let response = client
        .get(url) // GET request with query parameter
        .header(AUTHORIZATION, HIBACHI_API_KEY.to_owned()) // Authorization header
        .send()?; // Send the request

    type GetPendingOrders = Vec<PendingOrders>;

    let _parsed_struct: GetPendingOrders = serde_json::from_str(&response.text()?).expect("Failed to parse JSON");
    // println!("{:?}", _parsed_struct);

    Ok(())
}


fn get_setttled_trades() -> Result<(), Error> {
    let mut url: String = ACCOUNT_API_ENDPOINT.to_owned();
    let url_appendage: &str = "trade/account/settlements_history";
    url.push_str(url_appendage);

    let user_string = format!("?accountId={}", *HIBACHI_ACCOUNT_ID);
    url.push_str(&user_string);

    println!("{}", url);

    let client: Client = Client::new();

    let response = client
        .get(url) // GET request with query parameter
        .header(AUTHORIZATION, HIBACHI_API_KEY.to_owned()) // Authorization header
        .send()?; // Send the request

    let _parsed_struct: GetSettledTrades = serde_json::from_str(&response.text()?).expect("Failed to parse JSON");
    // println!("{:?}", _parsed_struct);

    Ok(())
}

fn get_account_trades() -> Result<(), Error> {
    let mut url: String = ACCOUNT_API_ENDPOINT.to_owned();
    let url_appendage: &str = "trade/account/trades";
    url.push_str(url_appendage);

    let user_string = format!("?accountId={}", *HIBACHI_ACCOUNT_ID);
    url.push_str(&user_string);

    println!("{}", url);

    let client: Client = Client::new();

    let response = client
        .get(url) // GET request with query parameter
        .header(AUTHORIZATION, HIBACHI_API_KEY.to_owned()) // Authorization header
        .send()?; // Send the request

    // println!("{:?}", response.status());
    // println!("{:?}", response.headers());
    // println!("{:?}", response.text());

    let _parsed_struct: GetAccountTrades = serde_json::from_str(&response.text()?).expect("Failed to parse JSON");
    // println!("{:?}", _parsed_struct);

    Ok(())
}

fn get_account_info() -> Result<(), Error> {
    let mut url: String = ACCOUNT_API_ENDPOINT.to_owned();
    let url_appendage: &str = "trade/account/info";
    url.push_str(url_appendage);

    let user_string = format!("?accountId={}", *HIBACHI_ACCOUNT_ID);
    url.push_str(&user_string);

    println!("{}", url);

    let client: Client = Client::new();

    let response = client
        .get(url) // GET request with query parameter
        .header(AUTHORIZATION, HIBACHI_API_KEY.to_owned()) // Authorization header
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


fn get_account_history() -> Result<(), Error> {
    let mut url: String = ACCOUNT_API_ENDPOINT.to_owned();
    let url_appendage: &str = "capital/history";
    url.push_str(url_appendage);

    let user_string = format!("?accountId={}", *HIBACHI_ACCOUNT_ID);
    url.push_str(&user_string);

    println!("{}", url);

    let client: Client = Client::new();

    let response = client
        .get(url) // GET request with query parameter
        .header(AUTHORIZATION, HIBACHI_API_KEY.to_owned()) // Authorization header
        .send()?; // Send the request

    // println!("{:?}", response.text()?);
    let _parsed_struct: GetAccountHistory = serde_json::from_str(&response.text()?).expect("Failed to parse JSON");
    // println!("{:?}", _parsed_struct);

    Ok(())
}


fn get_account_balance() -> Result<(), Error> {
    let mut url: String = ACCOUNT_API_ENDPOINT.to_owned();
    let url_appendage: &str = "capital/balance";
    url.push_str(url_appendage);

    let user_string = format!("?accountId={}", *HIBACHI_ACCOUNT_ID);
    url.push_str(&user_string);

    println!("{}", url);

    let client: Client = Client::new();

    let response = client
        .get(url) // GET request with query parameter
        .header(AUTHORIZATION, HIBACHI_API_KEY.to_owned()) // Authorization header
        .send()?; // Send the request

    // println!("Response Body: {}", response.text()?);

    // Check that the response was successful
    // if response.status().is_success() {
    //     let response_body = response.text()?;
    //     println!("Response: {}", response_body);
    // } else {
    //     println!("Error: {}", response.status());
    // }

    // println!("{:?}", response);
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