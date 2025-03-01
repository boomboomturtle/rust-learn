use tokio_tungstenite::connect_async;
use url::Url;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    let websocket_url = "wss://data-api.hibachi.xyz/ws/market"; // Replace with your URL
    let url = Url::parse(websocket_url).expect("Invalid WebSocket URL");

    println!("Connecting to {}", websocket_url);

    match connect_async(url).await {
        Ok((mut ws_stream, _)) => {
            println!("✅ Connected!");

            // Send a ping message
            if let Err(e) = ws_stream.send(Message::Ping(vec![])).await {
                println!("❌ Error sending ping: {:?}", e);
                return;
            }
            println!("✅ Ping sent!");

            // Wait for a response
            if let Some(Ok(msg)) = ws_stream.next().await {
                println!("✅ Received: {:?}", msg);
            }

            // Close connection
            let _ = ws_stream.close(None).await;
            println!("✅ WebSocket closed.");
        }
        Err(e) => {
            println!("❌ WebSocket connection failed: {:?}", e);
        }
    }
}
