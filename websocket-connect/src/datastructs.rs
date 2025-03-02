pub mod special_data_types {
    use serde::{Deserialize, Deserializer, Serialize};
    use std::str::FromStr;
    
    // #[allow(non_snake_case)]
    // #[derive(Debug, Serialize, Deserialize)]
    // pub struct OrderbookData {
    //     data: OrderbookData,
    //     depth: u32,
    //     granularity: String,
    //     messageType: String,
    //     symbol: String,
    //     topic: String
    // }

    // #[allow(non_snake_case)]
    // #[derive(Debug, Serialize, Deserialize)]
    // pub struct Orderbook {
    //     data: OrderbookData,
    //     depth: u32,
    //     granularity: String,
    //     messageType: String,
    //     symbol: String,
    //     topic: String
    // }

    fn str_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        f64::from_str(s).map_err(serde::de::Error::custom)
    }
    
    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FundingRateEstimation {
        #[serde(deserialize_with = "str_to_f64")]
        estimatedFundingRate: f64,
        nextFundingTimestamp: u32,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FundingRateData {
        fundingRateEstimation: FundingRateEstimation,
    }

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FundingRate {
        data: FundingRateData,
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

