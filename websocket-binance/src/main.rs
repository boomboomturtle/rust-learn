// use serde_json::json;
use tokio_stream::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::stream::MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;
use std::time::{SystemTime, UNIX_EPOCH};
use futures_util::{SinkExt};
// use std::time::{Duration, Instant};
// use std::any::type_name;

// use once_cell::sync::Lazy;
// use std::sync::Mutex;

mod datastructs;
use datastructs::{special_data_types::*};

const WS_URL: &str = "wss://fstream.binance.com/";


async fn orderbook_sub() {
    let mut url: String = WS_URL.to_owned();
    let url_appendage: &str = "ws/btcusdt@depth5@0ms";
    url.push_str(url_appendage);

    let ws_url = Url::parse(&url).expect("Failed to parse a URL");

    match connect_async(ws_url.to_string()).await {
        Ok((mut ws_stream,_)) => {
            println!("Websocket connected");
            
            
            while let Some(Ok(message)) = ws_stream.next().await {
                match message {
                    Message::Ping(ping_data) => {
                        ws_stream.send(Message::Pong(ping_data)).await.unwrap();
                    }
                    _ => {
                        let start = SystemTime::now();
                        let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
                        
                        println!("Received TICKSUB - {}", message);
        
                        let _parsed_struct: BinanceOrderbook = serde_json::from_str(&message.to_string()).expect("Failed to parse JSON");
                        let diff = since_epoch.as_millis().saturating_sub(_parsed_struct.E as u128);
                        println!("Ticker received {}: {:?}", diff, _parsed_struct);        
                    }
                }
            }
        }
        Err(e) => {
            println!("Connection failed {}", e);
        }
    }
}

async fn orderbook_ticker() {
    let mut url: String = WS_URL.to_owned();
    let url_appendage: &str = "ws/btcusdt@bookTicker";
    url.push_str(url_appendage);

    let ws_url = Url::parse(&url).expect("Failed to parse a URL");

    match connect_async(ws_url.to_string()).await {
        Ok((mut ws_stream,_)) => {
            println!("Websocket connected");
            
            while let Some(Ok(message)) = ws_stream.next().await {
                match message {
                    Message::Ping(ping_data) => {
                        ws_stream.send(Message::Pong(ping_data)).await.unwrap();
                    }
                    _ => {
                        let start = SystemTime::now();
                        let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
                        
                        println!("Received TICKSUB - {}", message);
        
                        let _parsed_struct: BinanceTicker = serde_json::from_str(&message.to_string()).expect("Failed to parse JSON");
                        let diff = since_epoch.as_millis().saturating_sub(_parsed_struct.E as u128);
                        println!("Ticker received {}: {:?}", diff, _parsed_struct);        
                    }
                }
                
                }
            }

        Err(e) => {
            println!("Connection failed {}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    println!("We are going to try to connect to a websocket a listen to Binance's messages");

    let task_ob = tokio::spawn(orderbook_sub());
    let task_ticker = tokio::spawn(orderbook_ticker());

    let _ =tokio::join!(task_ob);
}

// PARSING THE OB IS INCOMPLETE
