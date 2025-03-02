use serde_json::json;
// use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::stream::MaybeTlsStream, WebSocketStream};
use url::Url;
use futures_util::{SinkExt};

use std::any::type_name;

mod datastructs;
use datastructs::special_data_types::*;

use once_cell::sync::Lazy;
use std::sync::Mutex;



static CURR_OB: Lazy<Mutex<OrderbookData>> = Lazy::new(|| Mutex::new( OrderbookData {
                                                                            bid: PriceData {
                                                                                endPrice: 0.0,
                                                                                startPrice: 0.0,
                                                                                levels :vec![LevelData {
                                                                                    price: 0.0,
                                                                                    quantity: 0.0,
                                                                                }],
                                                                            },
                                                                            ask: PriceData {
                                                                                endPrice: 0.0,
                                                                                startPrice: 0.0,
                                                                                levels : vec![LevelData {
                                                                                    price: 0.0,
                                                                                    quantity: 0.0,
                                                                                }],
                                                                            },
                                                                        }
                                                            ));
    
    
    
// #[derive(Debug, PartialEq)]
// enum OrderbookMessageType {
//     Update,
//     Snapshot,
// }


fn print_type<T>(_: &T) {
    println!("Type: {}", type_name::<T>());
}

const WS_URL: &str = "wss://data-api.hibachi.xyz/ws/market";

async fn orderbook_sub(ob_json_string: String, ob_url: Url) {
    match connect_async(ob_url).await {
        Ok((mut ws_stream,_)) => {
            println!("Websocket connected");
            
            match ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(ob_json_string))
                .await {
                    Ok(_) => println!("Message sent"),
                    Err(e) => println!("Message failed = {}", e),
                }

            // print_type(&ws_stream);
                
            while let Some(Ok(message)) = ws_stream.next().await {
                println!("Received {}", message);

                let _parsed_struct: Orderbook = serde_json::from_str(&message.to_string()).expect("Failed to parse JSON");
                if _parsed_struct.messageType == "Snapshot" {
                    // CURR_OB = Lazy::new(|| Mutex::new(_parsed_struct.data));; // _parsed_struct.data;
                    let mut data_value = CURR_OB.lock().unwrap();
                    data_value.ask = _parsed_struct.data.ask;
                    data_value.bid = _parsed_struct.data.bid;

                    println!("Updated orderbook: {:?}", data_value);
                }
                // println!("{:?}", _parsed_struct);
            };

        }
        Err(e) => {
            println!("Connection failed {}", e);
        }
    }

}
// TODO: Orderbook data comes in 2 forms: Update and Snapshot: Update should update the orderbook and snapshot should give us a fresh copy.
// Figure out how to update your notion of the orderbook with the two different types of data.

async fn funding_rate_sub(ob_json_string: String, ob_url: Url) {
    match connect_async(ob_url).await {
        Ok((mut ws_stream,_)) => {
            println!("Websocket connected");
            
            match ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(ob_json_string))
                .await {
                    Ok(_) => println!("Message sent"),
                    Err(e) => println!("Message failed = {}", e),
                }

            // print_type(&ws_stream);
                
            while let Some(Ok(message)) = ws_stream.next().await {
                // println!("Received {}", message);

                let _parsed_struct: FundingRate = serde_json::from_str(&message.to_string()).expect("Failed to parse JSON");
                println!("{:?}", _parsed_struct);
            };

        }
        Err(e) => {
            println!("Connection failed {}", e);
        }
    }

}

#[tokio::main]
async fn main() {
    println!("We are going to try to connect to a websocket a listen to Hibachi's messages");
    let url = Url::parse(WS_URL).expect("Failed to parse a URL");

    let ob_json_message = json!({
        "method": "subscribe",
        "parameters": {
            "subscriptions": [
                {
                    "symbol": "ETH/USDT-P",
                    "topic": "orderbook"
                },
            ]
        }
    });

    let ob_json_string = ob_json_message.to_string();

    // orderbook_sub(ob_json_string, url.clone()).await;

    let fr_json_message = json!({
        "method": "subscribe",
        "parameters": {
            "subscriptions": [
                {
                    "symbol": "ETH/USDT-P",
                    "topic": "funding_rate_estimation"
                },
            ]
        }
    });

    let fr_json_string = fr_json_message.to_string();

    // funding_rate_sub(fr_json_string, url.clone()).await;

    let task_ob = tokio::spawn(orderbook_sub(ob_json_string, url.clone()));
    let task_fr = tokio::spawn(funding_rate_sub(fr_json_string, url.clone()));

    let _ =tokio::join!(task_ob, task_fr);
}



// {
//     "method": "subscribe",
//     "parameters": {
//         "subscriptions": [
//             {
//                 "symbol": "ETH/USDT-P",
//                 "topic": "mark_price"
//             },
//             {
//                 "symbol": "ETH/USDT-P",
//                 "topic": "spot_price"
//             },
//             {
//                 "symbol": "ETH/USDT-P",
//                 "topic": "funding_rate_estimation"
//             },
//             {
//                 "symbol": "ETH/USDT-P",
//                 "topic": "trades"
//             },
//             {
//                 "symbol": "ETH/USDT-P",
//                 "topic": "klines"
//             },
//             {
//                 "symbol": "ETH/USDT-P",
//                 "topic": "orderbook"
//             },
//             {
//                 "symbol": "ETH/USDT-P",
//                 "topic": "ask_bid_price"
//             }
//         ]
//     }
// }




// static mut CURR_OB: Lazy<Mutex<Orderbook>> = Lazy::new(|| Mutex::new(Orderbook { depth: 0, 
//     granularity: "0.01".to_string(), 
//     messageType: "NoType".to_string(),
//     symbol: "BLANK".to_string(),
//     topic: "orderbook".to_string(),
//     data: OrderbookData {
//         bid: PriceData {
//             endPrice: 0.0,
//             startPrice: 0.0,
//             levels :vec![LevelData {
//                 price: 0.0,
//                 quantity: 0.0,
//             }],
//         },
//         ask: PriceData {
//             endPrice: 0.0,
//             startPrice: 0.0,
//             levels : vec![LevelData {
//                 price: 0.0,
//                 quantity: 0.0,
//             }],
//         },
//     }
//     }));
