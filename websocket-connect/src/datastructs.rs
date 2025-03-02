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
    #[derive(Debug, Serialize, Deserialize)]
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
    pub struct OrderbookData {
        pub ask: PriceData,
        pub bid: PriceData,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Orderbook {
        pub data: OrderbookData,
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

// Received {"data":{"fundingRateEstimation":{"estimatedFundingRate":"-0.000301","nextFundingTimestamp":1740902400}},"symbol":"ETH/USDT-P","topic":"funding_rate_estimation"}

}


// {
//     "data": {
//       "ask": {
//         "endPrice": "2229.71",
//         "levels": [
//           {
//             "price": "2224.58",
//             "quantity": "0.035800000"
//           }
//         ],
//         "startPrice": "2220.56"
//       },
//       "bid": {
//         "endPrice": "2178.18",
//         "levels": [
//           {
//             "price": "2218.37",
//             "quantity": "18.195200000"
//           },
//           {
//             "price": "2217.26",
//             "quantity": "27.292800000"
//           },
//           {
//             "price": "2216.15",
//             "quantity": "45.452200000"
//           },
//           {
//             "price": "2178.18",
//             "quantity": "27.292800000"
//           }
//         ],
//         "startPrice": "2219.31"
//       }
//     },
//     "depth": 20,
//     "granularity": "0.01",
//     "messageType": "Update",
//     "symbol": "ETH/USDT-P",
//     "topic": "orderbook"
//   }

