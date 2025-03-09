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
    pub struct PriceData {
        #[serde(deserialize_with = "str_to_f64")]
        pub price: f64,
        #[serde(deserialize_with = "str_to_f64")]
        pub quantity: f64,
    }

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
        pub b: Vec<PriceData>,
        pub a: Vec<PriceData>,
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
        pub b: Vec<PriceData>,
        pub a: Vec<PriceData>,
    }


}

// Received 1741282187481 {"e":"depthUpdate","E":1741282187461,"T":1741282187461,"s":"BTCUSDT","U":6948375693146,"u":6948375703528,"pu":6948375692666,"b":[["88719.90","2.903"],["88719.40","0.004"],["88719.20","0.045"],["88718.70","0.170"],["88718.60","0.002"]],"a":[["88720.00","0.595"],["88720.10","0.027"],["88720.20","0.002"],["88720.40","0.031"],["88721.00","0.002"]]}
