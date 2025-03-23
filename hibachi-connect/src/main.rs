use std::env;
use once_cell::sync::Lazy;
// use std::fmt::format;
use reqwest::blocking::{get, Client};
use reqwest::header::AUTHORIZATION;
use reqwest::Error;
use lazy_static::lazy_static;
use std::sync::Mutex;

use chrono::Utc;
use std::time::{Duration, Instant};
use serde_json::json;

// use ethers::signers::{LocalWallet, Signer};
use ethers::utils::keccak256;
use hex;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio;
use tokio::runtime::Runtime;
use sha2::{Sha256, Digest};
use secp256k1::{Message, Secp256k1, ecdsa::Signature};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::U256;

mod api_struct;
use api_struct::{get_orderbook_data_api_body::*, market_inventory_api_response::*,open_interest_api_response::*, orderbook_data_api_response::*, price_info_api_response::GetMarketPriceInfo, market_stats_api_response::*};
use api_struct::{market_trades_api_response::*, market_klines_api_response::*, account_balance_api_response::*, account_history_api_response::*, order_details::*, account_info_api_response::*};
use api_struct::{account_trades_api_response::*, settled_trades_api_response::*, pending_orders_api_response::*, exchange_info_api_response::*};

const DATA_API_ENDPOINT: &str = "https://data-api.hibachi.xyz/";
const ACCOUNT_API_ENDPOINT: &str = "https://api.hibachi.xyz/";
const SYMBOL: &str = "BTC/USDT-P";
const CONTRACT_ID: u32 = 2;


lazy_static! {
    static ref EXCHANGE_INFO: Mutex<FutureContracts> = Mutex::new(set_exchange_data());
}

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
    // let _response = set_exchange_data();
    // let _response = get_market_inventory();
    // let _response = get_order_book(1, 0.1);
    // let _response = get_open_interest();
    // let _response = get_market_price_info();
    // let _response = get_market_stats();
    // let _response = get_market_trades();
    // let _response = get_market_klines("1h".to_owned(), None, None);
    // let _response = get_account_balance();``
    // let _response = get_account_history();
    // let _response = get_account_info();
    // let _response = get_account_trades();
    // let _response = get_setttled_trades();
    // let _response = get_pending_orders();
    let _response = place_order(100004.0, 0.00001, OrderSide::ASK, OrderType::LIMIT);
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

const PRICE_MULTIPLIER: u64 = 1u64 << 32; // Adjust as needed

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
    let now = Utc::now();
    let nonce: i64 = 1742714851498; // now.timestamp_micros();

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

    let local_exch_info = EXCHANGE_INFO.lock().expect("Cannot access EXCHANGE_INFO");
    let price_i128 = price_from_real(price, local_exch_info.underlyingDecimals).to_i128().expect("Handling the error case");
    println!("price_i128={:?}", price_i128);

    let max_fees_percent = 0.045;

    let mut signature= format!("{:016x} {:08x} {:016x} {:08x} {:016x} {:016x}", nonce, CONTRACT_ID, ((quantity*10_000_000_000.0) as u128), order_side_int, price_i128, ((max_fees_percent*100_000_000.0) as u128));
    signature = format!("{:016x}{:08x}{:016x}{:08x}{:016x}{:016x}", nonce, CONTRACT_ID, ((quantity*10_000_000_000.0) as u128), order_side_int, price_i128, ((max_fees_percent*100_000_000.0) as u128));

    let privatekey = HIBACHI_PRIVATE_KEY.to_owned();
    
    // Convert hex string to bytes
    let bytes = hex::decode(signature).expect("Invalid hex string");
    
    // Compute SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let hash_result = hasher.finalize();
    
    // Convert hash to hex format
    let hash_hex = hex::encode(hash_result);

    println!("SHA256: msgHash: {:?}", hash_hex);

    // Convert private key string to bytes
    let private_key_bytes = hex::decode(&privatekey).expect("Failed to decode private key");
    let rt = Runtime::new().unwrap();
    let signed_message = rt.block_on(sign_message(&hash_hex, &private_key_bytes)).expect("Hello");

    println!("Signed Message: {:?}", signed_message);
    
    let mut url = ACCOUNT_API_ENDPOINT.to_owned();
    let url_appendage = "/trade/order";
    url.push_str(url_appendage);

    let order_bundle = json!({
        "accountId": HIBACHI_ACCOUNT_ID.to_owned(),
        "symbol": SYMBOL.to_owned(),
        "nonce": nonce,
        "orderType": order_type,
        "side": order_side,
        "quantity": quantity.to_string(),
        "price": price.to_string(),
        "signature": signed_message,
    });

    let client = Client::new();
    let response = client.post(url)
        .header("Authorization", HIBACHI_API_KEY.to_owned())
        .header("Content-Type", "application/json")
        .json(&order_bundle)
        .send()?;

    println!("Response status: {}", response.status());

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

fn set_exchange_data() -> FutureContracts {
    let mut url: String = DATA_API_ENDPOINT.to_owned();
    let url_appendage: &str = "market/exchange-info";
    url.push_str(url_appendage);

    println!("{}", url);

    // let start_time: Instant = Instant::now();
    // Send a GET request
    let response: String = get(url).expect("Got an invalid response")
                            .text().expect("Failed to convert it to text");

    // println!("{:?}", response);
    // let elapsed: Duration = start_time.elapsed();    
    // println!("Time taken for GET request: {:?}", elapsed);

    // let start_time: Instant = Instant::now();

    let _parsed_struct: ExchangeInfo = serde_json::from_str(&response).expect("Failed to parse JSON");

    // let elapsed: Duration = start_time.elapsed();
    // println!("Time taken to deserialize and parse: {:?}", elapsed);

    // println!("{:?}", _parsed_struct);

    for futures in _parsed_struct.futureContracts {
        if futures.symbol == SYMBOL {
            return futures;
        }
    }

    panic!("I panic here");
}

async fn sign_message(msg_hash: &str, private_key: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    // Convert hex string to bytes
    let msg_bytes = hex::decode(msg_hash)?;
    
    // Create message for signing
    let message = Message::from_slice(&msg_bytes)?;
    
    // Create secret key from bytes
    let secret_key = secp256k1::SecretKey::from_slice(private_key)?;
    
    // Sign the message
    let signature = Secp256k1::new().sign_ecdsa(&message, &secret_key);
    
    // Format the signature as hex string
    let sig_bytes = signature.serialize_compact();
    
    // Create a wallet from the private key
    let wallet = LocalWallet::from_bytes(private_key)?;
    
    // Sign the message with the wallet to get the correct v value
    let sig = wallet.sign_message(msg_bytes).await?;
    
    Ok(format!("{:x}{:x}{:02x}", 
        U256::from_big_endian(&sig_bytes[..32]),
        U256::from_big_endian(&sig_bytes[32..]),
        sig.v-27))
}
