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

use once_cell::sync::Lazy;
use std::sync::Mutex;

const WS_URL: &str = "wss://fstream.binance.com/";

static CURR_OB: Lazy<Mutex<BinanceOrderbook>> = Lazy::new(|| Mutex::new( BinanceOrderbook {
    e: "CurrentOrderbook".to_string(), // Type of event
    E: 0, // event time
    s:"BTCUSDT".to_string(), // symbol
    T: 0,
    U: 0, // last update id in the event
    u:0, // first update id in the event
    pu: 0, // final update id in the stream
    b: vec![PriceData {price: 0.0, quantity: 0.0}], // bids
    a: vec![PriceData {price: 0.0, quantity: 0.0}], // asks
}
));



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
                        
                        println!("Received OBSUB - {}", message);
        
                        let _parsed_struct: BinanceOrderbook = serde_json::from_str(&message.to_string()).expect("Failed to parse JSON");
                        let _diff = since_epoch.as_millis().saturating_sub(_parsed_struct.E as u128);
                        // println!("Ticker received {}: {:?}", diff, _parsed_struct);    

                        let mut data_value = CURR_OB.lock().unwrap();
                        data_value.T = _parsed_struct.T;
                        data_value.E = _parsed_struct.E;
                        data_value.U = _parsed_struct.U;
                        data_value.a = _parsed_struct.a;
                        data_value.b = _parsed_struct.b;
                        data_value.e = _parsed_struct.e;
                        data_value.pu = _parsed_struct.pu;
                        data_value.s = _parsed_struct.s;
                        data_value.u = _parsed_struct.u;
    
                        println!("Updating our Current orderbook: {:?}", data_value);
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
                        let _diff = since_epoch.as_millis().saturating_sub(_parsed_struct.E as u128);
                        // println!("Ticker received {}: {:?}", diff, _parsed_struct);        
                        let mut data_value = CURR_OB.lock().unwrap();

                        data_value.T = _parsed_struct.T;
                        data_value.E = _parsed_struct.E;
                        data_value.u = _parsed_struct.u;
                        data_value.e = _parsed_struct.e;

                        let first_bid: PriceData = data_value.b[0].clone();
                        let first_ask: PriceData = data_value.a[0].clone();

                        for x in 0..data_value.b.len()  {
                            match x {
                                0 => {
                                    data_value.b[x].price = _parsed_struct.b;
                                    data_value.b[x].quantity = _parsed_struct.B;
                                }
                                _ => {
                                    data_value.b[x].price = _parsed_struct.b - (first_bid.price - data_value.b[x].price);
                                }
                            }
                            match x {
                                0 => {
                                    data_value.a[x].price = _parsed_struct.a;
                                    data_value.a[x].quantity = _parsed_struct.A;
                                }
                                _ => {
                                    data_value.a[x].price = _parsed_struct.a + (data_value.a[x].price - first_ask.price);
                                }
                            }
                        }

                        println!("Updating our Current orderbook: {:?}", data_value);
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

    let _ =tokio::join!(task_ob, task_ticker);
}
