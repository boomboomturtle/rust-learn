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


// Those are examples of message send by server
// [
//     {
//         "account": "<accountId>",
//         "data": {
//             "orderId": 578745035695106058,
//             "price": "69762.24077",
//             "quantity": "0.1288935088",
//             "side": "BID",
//             "symbol": "BTC/USDT-P",
//             "timestamp": 1712781049,
//             "nonce": 1714701600000000
//         },
//         "event": "order_creation"
//     },
//     {
//         "account": "<accountId>",
//         "data": {
//             "orderId": 578745035695106058,
//             "price": "69762.24077",
//             "quantity": "0.1288935088",
//             "side": "BID",
//             "symbol": "BTC/USDT-P",
//             "timestamp": 1712781049,
//             "executed_quantity": "0.096987527"
//         },
//         "event": "order_update"
//     },
//     {
//         "account": "<accountId>",
//         "data": {
//             "orderId": 578745034376521742,
//             "side": "ASK",
//             "symbol": "DDNG/USDT-P",
//             "timestamp": 1712781045
//         },
//         "event": "order_cancellation"
//     },
//     {
//         "account": "<accountId>",
//         "data": {
//             "depositQuantity": "123.000000"
//         },
//         "event": "balance_update"
//     },
//     {
//         "account": "<accountId>",
//         "data": {
//             "withdrawQuantity": "123.000000"
//         },
//         "event": "balance_update"
//     },
//     {
//         "account": "<accountId>",
//         "data": {
//             "updatedCollateralBalance": "1234.000000"
//         },
//         "event": "balance_update"
//     },
//     {
//         "account": "<accountId>",
//         "data": {
//             "isTaker": true,
//             "orderId": "582871374158365696",
//             "orderType": "Limit",
//             "price": "2345.000000",
//             "quantity": "0.000421590",
//             "realizedPnl": "0.022107",
//             "side": "BID",
//             "symbol": "ETH/USDT-P",
//             "timestamp": 1728526563
//         },
//         "event": "trade_update"
//     },
//     {
//         "account": "<accountId>",
//         "data": {
//             "symbol": "BTC/USDT-P",
//             "updatedPosition": {
//                 "direction": "Closed",
//                 "entryNotional": "0.000000",
//                 "quantity": "0.000000"
//             }
//         },
//         "event": "position_update"
//     },
//     {
//         "account": "<accountId>",
//         "data": {
//             "symbol": "HFT/USDT-P",
//             "updatedPosition": {
//                 "direction": "Long",
//                 "entryNotional": "11926.363515",
//                 "quantity": "28179.202190"
//             }
//         },
//         "event": "position_update"
//     },
//     {
//         "account": "<accountId>",
//         "data": {
//             "indexPrice": "61097.00801",
//             "quantity": "0.5580484584",
//             "settledAmount": "0.8618780736305764",
//             "side": "Buy",
//             "symbol": "BTC/USDT-P",
//             "timestamp": 1719446400,
//             "unrealizedFunding": "-734.608435"
//         },
//         "event": "funding_settlement"
//     },
//     {
//         "account": "<accountId>",
//         "event": "account_created",
//         "data": {
//             "ledgerAccountId": 123,
//             "timestamp": 123456789
//         }
//     },
//     {
//         "event": "deposit_status_update",
//         "account": "<accountId>",
//         "data": {
//             "depositQuantity": 1234,
//             "depositTxnHash": "0xabcdef",
//             "etaTsSecond": 10,
//             "status": "Processing/Accepted/Rejected/Succeeded",
//             "note": "Note"
//         }
//     },
//     {
//         "account": 2,
//         "data": {
//             "matchedQuantity": "0.000000004",
//             "orderId": "3",
//             "orderType": "MarketOrder",
//             "remainingQuantity": "0.000000005",
//             "symbol": "ETH/USDT-P",
//             "timestamp": 123456789
//         },
//         "event": "order_not_fully_matched"
//     },
//     {
//         "event": "order_request_rejected",
//         "account": "<accountId>",
//         "data": {
//             "orderId": 1234,
//             "error": "RiskLimitExceeded",
//             "requestType": "Update/New"
//         }
//     },
//     {
//         "event": "stream_expired",
//         "account": "<accountId>",
//         "params": {
//             "listenKey": "123",
//             "timestampMs": 123456789
//         }
//     },
//     {
//         "event": "transfer_status_update",
//         "sourceAccountId": "<accountId>",
//         "data": {
//             "transferQuantity": "1234",
//             "destAccountPublicKey": "0xabcdef",
//             "status": "Processing/Accepted/Rejected/Succeeded",
//             "isInstantWithdrawal": true,
//             "note": "Note"
//         }
//     },
//     {
//         "event": "withdraw_rejection",
//         "account": "<accountId>",
//         "data": {
//             "withdraw_quantity": 1234,
//             "timestamp": 123456789
//         }
//     },
//     {
//         "event": "withdraw_status_update",
//         "account": "<accountId>",
//         "data": {
//             "withdrawQuantity": 1234,
//             "withdrawAddress": "0xabcdef",
//             "etaTsSecond": 10,
//             "status": "Processing/Accepted/Rejected/Succeeded",
//             "note": "Note"
//         }
//     },
//     {
//         "event": "order_matched",
//         "account": "<accountId>",
//         "data": {
//             "orderId": 1234,
//             "symbol": "ETH/USDT-P",
//             "orderType": "LIMIT",
//             "matchedQuantity": "1234",
//             "remainingQuantity": "1234",
//             "timestamp": 123456789,
//             "nonce": 1714701600000000
//         }
//     },
//     {
//         "event": "trade_rejected",
//         "account": "<accountId>",
//         "data": {
//             "symbol": "ETH/USDT-P",
//             "orderId": 1234,
//             "quantity": "1234",
//             "timestamp": 123456789
//         }
//     }
// ]