use serde::{Deserialize, Deserializer};
use std::str::FromStr;

fn str_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    f64::from_str(s).map_err(serde::de::Error::custom)
}


pub mod market_inventory_api_response {
    use serde::{Deserialize, Serialize};

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MarketInventory {
        crossChainAssets: Vec<CrossChainAssets>,
        feeConfig: FeeConfig,
        markets: Vec<MarketsData>,
        tradingTiers: Vec<TradingTiers>,
    }

    // "crossChainAssets": 
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

    //   "markets": [
        // {
        //   "contract": {
        //   },
        //   "info": {
        //     "category": "Meme",
        //     "markPrice": "16.349525685",
        //     "price24hAgo": "17.034793082",
        //     "priceLatest": "16.333810056",
        //     "tags": [
        //       "meme"
        //     ]
        //   }
        // },

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    struct MarketsData {
        contract: ContractsData,
        info: InfoData,
    }

        //   "contract": {
        //     "displayName": "TRUMP/USDT Perps",
        //     "id": 8,
        //     "maintenanceFactorForPositions": "0.030000",
        //     "marketCloseTimestamp": null,
        //     "marketOpenTimestamp": null,
        //     "minNotional": "1",
        //     "minOrderSize": "0.00001",
        //     "orderbookGranularities": [
        //       "0.00001",
        //       "0.0001",
        //       "0.001",
        //       "0.01"
        //     ],
        //     "riskFactorForOrders": "0.500000",
        //     "riskFactorForPositions": "0.370000",
        //     "settlementDecimals": 6,
        //     "settlementSymbol": "USDT",
        //     "status": "LIVE",
        //     "stepSize": "0.00001",
        //     "symbol": "TRUMP/USDT-P",
        //     "tickSize": "0.000000001",
        //     "underlyingDecimals": 5,
        //     "underlyingSymbol": "TRUMP"
        //   },

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    struct ContractsData {
        displayName: String,
        id: i32,
        maintenanceFactorForPositions: String,
        marketCloseTimestamp: Option<String>, // TODO: This can be null. How will you handle it?
        marketOpenTimestamp: Option<String>, // TODO: This can be null. How will you handle it?
        minNotional: String,
        minOrderSize: String,
        orderbookGranularities: Vec<String>,
        riskFactorForOrders: String,
        riskFactorForPositions: String,
        settlementDecimals: i32,
        settlementSymbol: String,
        status: String,
        stepSize: String,
        symbol: String,
        tickSize: String,
        underlyingDecimals: u32,
        underlyingSymbol: String,
    }

        //   "info": {
        //     "category": "Meme",
        //     "markPrice": "16.349525685",
        //     "price24hAgo": "17.034793082",
        //     "priceLatest": "16.333810056",
        //     "tags": [
        //       "meme"
        //     ]
        //   }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    struct InfoData {
        category: Option<String>,
        markPrice: String,
        price24hAgo: Option<String>,
        priceLatest: Option<String>,
        tags: Vec<String>,
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
}

pub mod open_interest_api_response {
    use serde::{Deserialize, Serialize};
    // {"totalQuantity":"38.552813167"}

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetOpenInterest {
        pub totalQuantity: String,
    }
}


pub mod orderbook_data_api_response {
    use serde::{Deserialize, Serialize};
    
    // {
    //     "ask": {
    //         "endPrice": "2803.32",
    //         "levels": [
    //             {
    //                 "price": "2803.30",
    //                 "quantity": "4.549070814"
    //             }
    //         ],
    //         "startPrice": "2803.30"
    //     },
    //     "bid": {
    //         "endPrice": "2801.61",
    //         "levels": [
    //             {
    //                 "price": "2801.71",
    //                 "quantity": "1.630000000"
    //             }
    //         ],
    //         "startPrice": "2801.71"
    //     }
    // }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PriceInfo {
        pub price: String,
        pub quantity: String,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AllPriceInfo {
        pub endPrice: String,
        pub levels: Vec<PriceInfo>,
        pub startPrice: String,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetOrderbookData {
        pub bid: AllPriceInfo,
        pub ask: AllPriceInfo,        
    }


}


pub mod get_orderbook_data_api_body {
    use serde::{Deserialize, Serialize};
    
    // symbol (string)
    // depth (optional 32-bit unsigned integer representing the number of levels to query)
    // granularity (optional string representing the precision of the levels returned)        

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetOrderbookDataInput {
        pub symbol: String,
        pub depth: u32,
        pub granularity: String,
    }
}


pub mod price_info_api_response {
    use serde::{Deserialize, Serialize};
    
    // {
    //     "askPrice": "2673.245523",
    //     "bidPrice": "2672.320000",
    //     "fundingRateEstimation": {
    //       "estimatedFundingRate": "-0.000132",
    //       "nextFundingTimestamp": 1740412800
    //     },
    //     "markPrice": "2672.560800",
    //     "spotPrice": "2672.873040",
    //     "symbol": "ETH/USDT-P",
    //     "tradePrice": "2670.160000"
    //   }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FundingRateEstimateInfo {
        pub estimatedFundingRate: String,
        pub nextFundingTimestamp: u32,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetMarketPriceInfo {
        pub askPrice: String,
        pub bidPrice: String,
        pub fundingRateEstimation: FundingRateEstimateInfo,
        pub markPrice: String,
        pub spotPrice: String,
        pub symbol: String,
        pub tradePrice: String,
    }
}


pub mod market_stats_api_response {
    use serde::{Deserialize, Serialize};    
// Response = "{\"high24h\":\"2842.350000\",\"low24h\":\"2664.210000\",\"symbol\":\"ETH/USDT-P\",\"volume24h\":\"847053.598157\"}"

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetMarketStatsInfo {
        pub high24h: String,
        pub low24h: String,
        pub symbol: String,
        pub volume24h: String,
    }
}


pub mod market_trades_api_response {
    use serde::{Deserialize, Serialize};    

    // {
    //     "trades": [
    //       {
    //         "price": "3512.431902",
    //         "quantity": "1.414780098",
    //         "takerSide": "Buy",
    //         "timestamp": 1712692147
    //       },
    //       {
    //         "price": "3512.783321",
    //         "quantity": "1.138242707",
    //         "takerSide": "Sell",
    //         "timestamp": 1712692147
    //       }
    //     ]
    //   }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TradesInfo {
        pub price: String,
        pub quantity: String,
        pub takerSide: String,
        pub timestamp: u32,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetMarketTradesInfo {
        pub trades: Vec<TradesInfo>,
    }
}


pub mod market_klines_api_response {
    use serde::{Deserialize, Serialize};    

// {
//     "klines": [
//       {
//         "close": "3704.751036",
//         "high": "3716.530378",
//         "interval": "1h",
//         "low": "3699.627883",
//         "open": "3716.406894",
//         "timestamp": 1712628000,
//         "volumeNotional": "1637355.846362"
//       },
//       {
//         "close": "3693.029781",
//         "high": "3717.863717",
//         "interval": "1h",
//         "low": "3682.131347",
//         "open": "3706.001256",
//         "timestamp": 1712631600,
//         "volumeNotional": "3590375.750775"
//       }
//     ]
//   }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KlinesInfo {
        pub close: String,
        pub high: String,
        pub interval: String,
        pub low: String,
        pub open: String,
        pub timestamp: u32,
        pub volumeNotional: String,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetMarketKlinesInfo {
        pub klines: Vec<KlinesInfo>,
    }
}


pub mod account_balance_api_response {
    use serde::{Deserialize, Serialize};    

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountBalance {
        pub balance: String,
    }
}


pub mod account_history_api_response {
    use serde::{Deserialize, Serialize};    

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Transaction {
        pub assetId: u32,
        pub blockNumber: u32,
        pub chain: Option<u32>,
        pub etaTsSec: u32,
        pub id: u32,
        pub quantity: String,
        pub status: String,
        pub timestamp: u32,
        pub token: Option<String>,
        pub transactionHash: String,
        pub transactionType: String,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountHistory {
        pub transactions: Vec<Transaction>,
    }
}

pub mod order_details {
    use serde::{Deserialize, Serialize};

    // {
    //     "symbol": "BTC/USDT-P",
    //     "side": "ASK",
    //     "nonce": 1714701600000000,
    //     "quantity": "1 BTC", 
    //     "price": "100,000 USDT",
    //     "maxFees": 0,
    //   }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OrderDetails {
        pub symbol: String,
        pub side: String,
        pub nonce: String,
        pub quantity: f64,
        pub price: f64,
        pub maxFees: u32,
    }
}


pub mod account_info_api_response {
    use serde::{Deserialize, Serialize};    

    // {
    //     "assets": [
    //       {
    //         "quantity": "13.110000",
    //         "symbol": "USDT"
    //       }
    //     ],
    //     "balance": "13.110000",
    //     "maximalWithdraw": "13.110000",
    //     "numFreeTransfersRemaining": 100,
    //     "positions": [
    //     {
    //         "direction": "Short",
    //         "entryNotional": "10.302213",
    //         "notionalValue": "10.225008",
    //         "quantity": "0.004310550",
    //         "symbol": "ETH/USDT-P",
    //         "unrealizedFundingPnl": "0.000000",
    //         "unrealizedTradingPnl": "0.077204"
    //       },
      
    //     ],
    // //     "totalOrderNotional": "0.000000",
    //     "totalPositionNotional": "0.000000",
    //     "totalUnrealizedFundingPnl": "0.000000",
    //     "totalUnrealizedPnl": "0.000000",
    //     "totalUnrealizedTradingPnl": "0.000000",
    //     "tradeMakerFeeRate": "0.00015000",
    //     "tradeTakerFeeRate": "0.00045000"
    //   }
    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PositionInfo {
        pub direction: String,
        pub entryNotional: String,
        pub notionalValue: String,
        pub quantity: String,
        pub unrealizedFundingPnl: String,
        pub unrealizedTradingPnl: String,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AssetInfo {
        pub quantity: String,
        pub symbol: String,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountInfo {
        pub assets: Vec<AssetInfo>,
        pub balance: String,
        pub maximalWithdraw: String,
        pub numFreeTransfersRemaining: u32,
        pub positions: Vec<PositionInfo>,
        pub totalOrderNotional: String,
        pub totalPositionNotional: String,
        pub totalUnrealizedFundingPnl: String,
        pub totalUnrealizedPnl: String,
        pub totalUnrealizedTradingPnl: String,
        pub tradeMakerFeeRate: String,
        pub tradeTakerFeeRate: String,
    }
}

pub mod account_trades_api_response {
    use serde::{Deserialize, Serialize};    

    // {
    //     "trades": [
    //       {
    //         "askAccountId": 126,
    //         "askOrderId": 582869893016453100,
    //         "bidAccountId": 131,
    //         "bidOrderId": 582869893208343600,
    //         "fee": "0.141460",
    //         "id": 1818286,
    //         "orderType": "LIMIT",
    //         "price": "2369.078949",
    //         "quantity": "0.398074810",
    //         "realizedPnl": "0.000000",
    //         "side": "Sell",
    //         "symbol": "ETH/USDT-P",
    //         "timestamp": 1728516134
    //       },
    //     ]
    //   }
    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Trades {
        pub askAccountId: u32,
        pub askOrderId: u64,
        pub bidAccountId: u32,
        pub bidOrderId: u64,
        pub fee: String,
        pub id: u32,
        pub orderType: String,
        pub price: String,
        pub quantity: String,
        pub realizedPnL: String,
        pub side: String,
        pub symbol: String,
        pub timestamp: u32,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountTrades {
        pub trades: Vec<Trades>,
    }
}

pub mod settled_trades_api_response {
    use serde::{Deserialize, Serialize};    

    // {
    //     "settlements": [
    //       {
    //         "direction": "Short",
    //         "indexPrice": "57202.23399",
    //         "quantity": "0.2409505256",
    //         "settledAmount": "4.271524297077262",
    //         "symbol": "BTC/USDT-P",
    //         "timestamp": 1720512543
    //       }
    //     ]
    //   }
    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Settlement {
        pub direction: String,
        pub indexPrice: String,
        pub quantity: String,
        pub settledAmount: String,
        pub symbol: String,
        pub timestamp: u32,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetSettledTrades {
        pub settlements: Vec<Settlement>,
    }
}

pub mod pending_orders_api_response {
    use serde::{Deserialize, Serialize};    

//   {
//     "accountId": 126,
//     "availableQuantity": "0.389560448",
//     "contractId": 1,
//     "creationTime": 1731609679,
//     "orderId": "582870002718998528",
//     "orderType": "LIMIT",
//     "price": "2344.032723",
//     "side": "BID",
//     "status": "PLACED",
//     "symbol": "ETH/USDT-P",
//     "totalQuantity": "0.389560448"
//   },


    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PendingOrders {
        pub accountId: u32,
        pub availableQuantity: String,
        pub contractId: u32,
        pub creationTime: u32,
        pub orderId: String,
        pub orderType: String,
        pub price: String,
        pub side: String,
        pub status: String,        
        pub symbol: String,
        pub totalQuantity: String,
    }

    // #[allow(non_snake_case)]
    // #[derive(Debug, Serialize, Deserialize)]
    // pub struct GetPendingOrders {
    //     pub orders: Vec<PendingOrders>,
    // }
}

pub mod exchange_info_api_response {
    use serde::{Deserialize, Serialize};
    use crate::api_struct::str_to_f64;

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ExchangeInfo {
        pub feeConfig: FeeConfig,
        pub futureContracts: Vec<FutureContracts>,
        pub instantWithdrawalLimit: InstantWithdrawalLimit,
        pub maintenanceWindow: Vec<MaintenanceWindow>,
        pub status: String,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FeeConfig {
        #[serde(deserialize_with = "str_to_f64")]
        pub depositFees: f64,
        pub instantWithdrawDstPublicKey: String,
        pub instantWithdrawalFees: Vec<(i32, f64)>,
        #[serde(deserialize_with = "str_to_f64")]
        pub tradeMakerFeeRate: f64,
        #[serde(deserialize_with = "str_to_f64")]
        pub tradeTakerFeeRate: f64,
        #[serde(deserialize_with = "str_to_f64")]
        pub transferFeeRate: f64,
        #[serde(deserialize_with = "str_to_f64")]
        pub withdrawalFees: f64,
    }

        #[allow(non_snake_case)]
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FutureContracts {
            pub displayName: String,
            pub id: u32,
            #[serde(deserialize_with = "str_to_f64")]
            pub maintenanceFactorForPositions: f64,
            pub marketCloseTimestamp: Option<String>,
            pub marketOpenTimestamp: Option<String>,
            #[serde(deserialize_with = "str_to_f64")]
            pub minNotional: f64,
            #[serde(deserialize_with = "str_to_f64")]
            pub minOrderSize: f64,
            pub orderbookGranularities: Vec<String>,
            #[serde(deserialize_with = "str_to_f64")]
            pub riskFactorForOrders: f64,
            #[serde(deserialize_with = "str_to_f64")]
            pub riskFactorForPositions: f64,
            pub settlementDecimals: u32,
            pub settlementSymbol: String,
            pub status: String,
            #[serde(deserialize_with = "str_to_f64")]
            pub stepSize: f64,
            pub symbol: String,
            #[serde(deserialize_with = "str_to_f64")]
            pub tickSize: f64,
            pub underlyingDecimals: i32,
            pub underlyingSymbol: String,      
        }
        
        #[allow(non_snake_case)]
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InstantWithdrawalLimit {
            #[serde(deserialize_with = "str_to_f64")]
            pub lowerLimit: f64,
            #[serde(deserialize_with = "str_to_f64")]
            pub upperLimit: f64,
        }

        #[allow(non_snake_case)]
        #[derive(Debug, Serialize, Deserialize)]
        pub struct MaintenanceWindow {
            pub begin: f64,
            pub end: f64,
            pub note: String,
        }

}

pub mod place_order_api_response {
    use serde::{Deserialize, Serialize};    

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OrderResponse {
        pub orderId: String,
    }

}
