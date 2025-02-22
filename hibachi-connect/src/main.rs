use std::{array, env};
use reqwest::blocking::get;
use reqwest::Error;
use std::time::{Duration, Instant};
use serde_json::Value;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct MarketInventory {
    crossChainAssets: CrossChainAssets,
    feeConfig: FeeConfig,
    markets: Markets,
    tradingTiers: TradingTiers,
}


// "crossChainAssets": [
//     {
//       "chain": "Base",
//       "exchangeRateFromUSDT": "0.990000",
//       "exchangeRateToUSDT": "0.990000",
//       "instantWithdrawalLowerLimitInUSDT": "0.033438705575322546",
//       "instantWithdrawalUpperLimitInUSDT": "8074.747474747474",
//       "token": "USDC"
//     }

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct CrossChainAssets {
    chain: String,
    exchangeRateFromUSDT: String,
    exchangeRateToUSDT: String,
    instantWithdrawalLowerLimitInUSDT: String,
    instantWithdrawalUpperLimitInUSDT: String,
    token: String,
}

// "feeConfig": {
//     "depositFees": "0.013209",
//     "instantWithdrawDstPublicKey": "a4fff986badd3b58ead09cc617a82ff1b5b77b98d560baa27fbcffa4c08610b6372f362f3e8e530291f24251f2c332d958bf776c88ae4370380eee943cddf859",
//     "instantWithdrawalFees": [
//       [
//         1000,
//         0.01
//       ]
//     ],
//     "tradeMakerFeeRate": "0.00015000",
//     "tradeTakerFeeRate": "0.00045000",
//     "transferFeeRate": "0.00010000",
//     "withdrawalFees": "0.013209"
//   },

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct FeeConfig {
    depositFees: String,
    instantWithdrawDstPublicKey: String,
    instantWithdrawalFees: Vec<(i32, f64)>,
    tradeMakerFeeRate: String,
    tradeTakerFeeRate: String,
    transferFeeRate: String,
    withdrawalFees: String,
}


#[derive(Debug, Serialize, Deserialize)]
struct Markets {
}

// "tradingTiers": [
//     {
//       "level": 0,
//       "lowerThreshold": "0.000000",
//       "title": "Flicker",
//       "upperThreshold": "0.000000"
//     },

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct TradingTiers {
    level: i32,
    lowerThreshold: String,
    title: String,
    upperThreshold: String
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

    println!("{}", parsed_value);

    // for key in parsed_value.as_object().unwrap().keys(){
    //     println!("{}", key);
    // }

    Ok(())
}


