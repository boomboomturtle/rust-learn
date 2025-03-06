// use serde_json::json;
use tokio_stream::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::stream::MaybeTlsStream, WebSocketStream};
use url::Url;
use futures_util::{SinkExt};
use std::time::{Duration, Instant};
// use std::any::type_name;

use once_cell::sync::Lazy;
use std::sync::Mutex;

const WS_URL: &str = "wss://fstream.binance.com/ws/btcusdt@depth@100ms";

async fn orderbook_sub(ob_url: Url) {
    match connect_async(ob_url.to_string()).await {
        Ok((mut ws_stream,_)) => {
            println!("Websocket connected");
            
            while let Some(Ok(message)) = ws_stream.next().await {
                println!("Received {}", message);
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
    let url = Url::parse(WS_URL).expect("Failed to parse a URL");

    let task_ob = tokio::spawn(orderbook_sub(url.clone()));

    let _ =tokio::join!(task_ob);
}

