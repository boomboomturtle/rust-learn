use serde_json::json;
use tokio_stream::StreamExt;
use tokio_tungstenite::connect_async;
use url::Url;
use futures_util::{SinkExt};

const WS_URL: &str = "wss://data-api.hibachi.xyz/ws/market";

#[tokio::main]
async fn main() {
    println!("We are going to try to connect to a websocket a listen to Hibachi's messages");
    let url = Url::parse(WS_URL).expect("Failed to parse a URL");

    match connect_async(url).await {
        Ok((mut ws_stream,_)) => {
            println!("Websocket connected");
            
            let json_message = json!({
                "method": "subscribe",
                "parameters": {
                    "subscriptions": [
                        {
                            "symbol": "ETH/USDT-P",
                            "topic": "mark_price"
                        },
                        {
                            "symbol": "ETH/USDT-P",
                            "topic": "spot_price"
                        },
                        {
                            "symbol": "ETH/USDT-P",
                            "topic": "funding_rate_estimation"
                        },
                        {
                            "symbol": "ETH/USDT-P",
                            "topic": "trades"
                        },
                        {
                            "symbol": "ETH/USDT-P",
                            "topic": "klines"
                        },
                        {
                            "symbol": "ETH/USDT-P",
                            "topic": "orderbook"
                        },
                        {
                            "symbol": "ETH/USDT-P",
                            "topic": "ask_bid_price"
                        }
                    ]
                }
            });

            let json_string = json_message.to_string();

            match ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(json_string))
                .await {
                    Ok(_) => println!("Message sent"),
                    Err(e) => println!("Message failed = {}", e),
                }
                
            while let Some(Ok(message)) = ws_stream.next().await {
                println!("Received {}", message);
            };

        }
        Err(e) => {
            println!("Connection failed {}", e);
        }
    }
}
