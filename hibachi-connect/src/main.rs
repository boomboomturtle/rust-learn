use std::env;
use once_cell::sync::Lazy;
// use std::fmt::format;
use reqwest::blocking::{get, Client};
use reqwest::header::AUTHORIZATION;
use reqwest::Error;

use chrono::Utc;
use std::time::{Duration, Instant};
// use serde_json::Value;

use k256::ecdsa::{SigningKey, Signature, signature::Signer};
use k256::SecretKey;
use sha2::{Digest, Sha256};



mod api_struct;
use api_struct::{get_orderbook_data_api_body::*, market_inventory_api_response::*,open_interest_api_response::*, orderbook_data_api_response::*, price_info_api_response::GetMarketPriceInfo, market_stats_api_response::*};
use api_struct::{market_trades_api_response::*, market_klines_api_response::*, account_balance_api_response::*, account_history_api_response::*, order_details::*, account_info_api_response::*};
use api_struct::{account_trades_api_response::*, settled_trades_api_response::*, pending_orders_api_response::*, exchange_info_api_response::*};

const DATA_API_ENDPOINT: &str = "https://data-api.hibachi.xyz/";
const ACCOUNT_API_ENDPOINT: &str = "https://api.hibachi.xyz/";
const SYMBOL: &str = "BTC/USDT-P";
const CONTRACT_ID: u32 = 2;

static HIBACHI_API_KEY: Lazy<String> = Lazy::new(|| {
    env::var("HIBACHI_API_KEY").expect("HIBACHI_API_KEY not set")
});

static HIBACHI_ACCOUNT_ID: Lazy<String> = Lazy::new(|| {
    env::var("HIBACHI_ACCOUNT_ID").expect("HIBACHI_ACCOUNT_ID not set")
});

static HIBACHI_PRIVATE_KEY: Lazy<String> = Lazy::new(|| {
    env::var("HIBACHI_PRIVATE_KEY").expect("HIBACHI_PRIVATE_KEY not set")
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
    let _response = get_exchange_info();
    // let _response = get_market_inventory();
    // let _response = get_order_book(1, 0.1);
    // let _response = get_open_interest();
    // let _response = get_market_price_info();
    // let _response = get_market_stats();
    // let _response = get_market_trades();
    // let _response = get_market_klines("1h".to_owned(), None, None);
    // let _response = get_account_balance();
    // let _response = get_account_history();
    // let _response = get_account_info();
    // let _response = get_account_trades();
    // let _response = get_setttled_trades();
    // let _response = get_pending_orders();
    let _response = place_order(100004.0, 0.00001,OrderSide::ASK, OrderType::LIMIT);
}

// ('BTC/USDT-P', 'ASK', 'LIMIT', "0.00001", "100004.0", "0.045", 2, 10)
// const orderBody {
//     accountId: 4488,
//     symbol: 'BTC/USDT-P',
//     side: 'ASK',
//     orderType: 'LIMIT',
//     quantity: '0.00001',
//     maxFeesPercent: '0.045',
//     price: '100004.0',
//     nonce: 1741793885924,
//     signature: ''
//   }

use bigdecimal::{BigDecimal, ToPrimitive};
use std::str::FromStr;

const PRICE_MULTIPLIER: u64 = (1u64 << 32); // Adjust as needed

fn price_from_real(price: f64, underlying_decimals: i32) -> BigDecimal {
    let decimals = 6 - underlying_decimals;

    // Convert price (f64) to BigDecimal using string parsing to maintain precision
    let mut price_big = BigDecimal::from_str(&price.to_string()).unwrap();

    // Apply decimal shift
    price_big = if decimals >= 0 {
        price_big * BigDecimal::from(10_i64.pow(decimals as u32))
    } else {
        price_big / BigDecimal::from(10_i64.pow((-decimals) as u32))
    };

    // Apply multiplier
    price_big *= BigDecimal::from(PRICE_MULTIPLIER);

    // Round down to integer (truncate decimals)
    price_big.with_scale(0)
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
    let nonce: i64 = 1741879308376; // now.timestamp_micros();

    let order_side = match side {
        OrderSide::ASK => "ASK".to_string(),
        OrderSide::BID => "BID".to_string(),
    };

    let order_side_int = match side {
        OrderSide::ASK => 0,
        OrderSide::BID => 1,
    };

    let order_type = match o_type {
        OrderType::LIMIT => "LIMIT".to_string(),
        OrderType::MARKET => "MARKET".to_string(),
    };

    let underlying_decimals = 10;
    let price_i128 = price_from_real(price, underlying_decimals).to_i128().expect("Handling the error case");
    // let price_f128 = (price * (1u64 << 32) as f64*10f64.powi(-4)) as u128;

    println!("price_i128={:?}", price_i128);

    let max_fees_percent = 0.045;

    let mut signature= format!("{:016x} {:08x} {:016x} {:08x} {:016x} {:016x}", nonce, CONTRACT_ID, ((quantity*10_000_000_000.0) as u128), order_side_int, price_i128, ((max_fees_percent*100_000_000.0) as u128));
    signature = format!("{:016x}{:08x}{:016x}{:08x}{:016x}{:016x}", nonce, CONTRACT_ID, ((quantity*10_000_000_000.0) as u128), order_side_int, price_i128, ((max_fees_percent*100_000_000.0) as u128));

    let signed_message: String = "".to_string();

    println!("account_id = {}, symbol = {}, nonce = {}, order_type = {}, side = {}, quantity = {}, price = {}, signature = {:?}, maxFeesPercent = {}, contract_id = {}", 
            *HIBACHI_ACCOUNT_ID, SYMBOL, nonce, order_type, order_side, quantity.to_string(), price.to_string(), signature, max_fees_percent, CONTRACT_ID);

    println!("Signed Message = {:?}", signed_message);

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

    println!("{}", response);
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

    // println!("{:?}", response);
    // let elapsed: Duration = start_time.elapsed();    
    // println!("Time taken for GET request: {:?}", elapsed);

    // let start_time: Instant = Instant::now();

    let _parsed_struct: MarketInventory = serde_json::from_str(&response).expect("Failed to parse JSON");

    // let elapsed: Duration = start_time.elapsed();    
    // println!("Time taken to deserialize and parse: {:?}", elapsed);

    // println!("{:?}", _parsed_struct);

    Ok(())
}

fn get_exchange_info() -> Result <(), Error> {
    let mut url: String = DATA_API_ENDPOINT.to_owned();
    let url_appendage: &str = "market/exchange-info";
    url.push_str(url_appendage);

    println!("{}", url);

    // let start_time: Instant = Instant::now();
    // Send a GET request
    let response: String = get(url)?
        .text()?;    

    println!("{:?}", response);
    // let elapsed: Duration = start_time.elapsed();    
    // println!("Time taken for GET request: {:?}", elapsed);

    // let start_time: Instant = Instant::now();

    let _parsed_struct: ExchangeInfo = serde_json::from_str(&response).expect("Failed to parse JSON");

    // let elapsed: Duration = start_time.elapsed();    
    // println!("Time taken to deserialize and parse: {:?}", elapsed);

    println!("{:?}", _parsed_struct);

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_deserialize() {
//         let msg = "{"ask":{"endPrice":"2806.35","levels":[{"price":"2806.35","quantity":"0.350000000"}],"startPrice":"2806.35"},"bid":{"endPrice":"2805.30","levels":[{"price":"2805.30","quantity":"0.290000000"}],"startPrice":"2805.30"}}%";

//     }
// }