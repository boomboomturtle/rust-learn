pub mod special_data_types {
    use serde::{Deserialize, Deserializer, Serialize};
    use std::str::FromStr;
    
    fn str_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        f64::from_str(s).map_err(serde::de::Error::custom)
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct LevelData {
        #[serde(deserialize_with = "str_to_f64")]
        pub price: f64,
        #[serde(deserialize_with = "str_to_f64")]
        pub quantity: f64,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PriceData {
        #[serde(deserialize_with = "str_to_f64")]
        pub endPrice: f64,
        #[serde(deserialize_with = "str_to_f64")]
        pub startPrice: f64,
        pub levels: Vec<LevelData>,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HibachiOrderbookData {
        pub ask: PriceData,
        pub bid: PriceData,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HibachiOrderbook {
        pub data: HibachiOrderbookData,
        pub depth: u32,
        pub granularity: String,
        pub messageType: String,
        pub symbol: String,
        pub topic: String
    }
    
    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FundingRateEstimation {
        #[serde(deserialize_with = "str_to_f64")]
        pub estimatedFundingRate: f64,
        pub nextFundingTimestamp: u32,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FundingRateData {
        pub fundingRateEstimation: FundingRateEstimation,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FundingRate {
        pub data: FundingRateData,
        symbol: String,
        topic: String
    }

    // #[allow(non_snake_case)]
    // #[derive(Debug, Serialize, Deserialize, Clone)]
    // pub struct LevelData {
    //     #[serde(deserialize_with = "str_to_f64")]
    //     pub price: f64,
    //     #[serde(deserialize_with = "str_to_f64")]
    //     pub quantity: f64,
    // }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BinanceOrderbook {
        pub e: String,
        pub E: u64,
        pub T: u64,
        pub s: String,
        pub U: u64,
        pub u: u64,
        pub pu: u64,
        pub b: Vec<LevelData>,
        pub a: Vec<LevelData>,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BinanceTicker {
        pub e: String,
        pub u: u64,
        pub s: String,
        #[serde(deserialize_with = "str_to_f64")]
        pub b: f64,
        #[serde(deserialize_with = "str_to_f64")]
        pub B: f64,
        #[serde(deserialize_with = "str_to_f64")]
        pub a: f64,
        #[serde(deserialize_with = "str_to_f64")]
        pub A: f64,
        pub T: u64,
        pub E: u64,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CurrentBinanceOrderbook {
        pub e: String,
        pub E: u64,
        pub T: u64,
        pub s: String,
        pub U: u64,
        pub u: u64,
        pub pu: u64,
        pub b: Vec<LevelData>,
        pub a: Vec<LevelData>,
    }

    // pub struct OrderbookStats {
    //     pub spread: f64,
    //     pub spread_bps: f64,
    // }

}


// pub mod binance_special_data_types {
//     use serde::{Deserialize, Deserializer, Serialize};
//     use std::str::FromStr;
    
//     fn str_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s: &str = Deserialize::deserialize(deserializer)?;
//         f64::from_str(s).map_err(serde::de::Error::custom)
//     }


// }

