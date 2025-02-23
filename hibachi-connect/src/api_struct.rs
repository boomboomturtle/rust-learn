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
        price24hAgo: String,
        priceLatest: String,
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

pub mod orderbook_data_response {
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


