use serde_json::json;
// use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::{stream::MaybeTlsStream, protocol::Message}, WebSocketStream};
use url::Url;
use futures_util::{SinkExt};
use std::time::{Duration, Instant};
use std::any::type_name;
use std::time::{SystemTime, UNIX_EPOCH};

mod datastructs;
use datastructs::special_data_types::*;

use once_cell::sync::Lazy;
use std::sync::Mutex;


static CURR_HIBACHI_OB: Lazy<Mutex<HibachiOrderbookData>> = Lazy::new(|| Mutex::new( HibachiOrderbookData {
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

static CURR_FR: Lazy<Mutex<FundingRateEstimation>> = Lazy::new(|| Mutex::new( FundingRateEstimation {
                                                                                    estimatedFundingRate: 0.0,
                                                                                    nextFundingTimestamp: 100,
                                                                                }
                                                                    ));



static CURR_BINANCE_OB: Lazy<Mutex<BinanceOrderbook>> = Lazy::new(|| Mutex::new( BinanceOrderbook {
    e: "CurrentOrderbook".to_string(), // Type of event
    E: 0, // event time
    s:"BTCUSDT".to_string(), // symbol
    T: 0,
    U: 0, // last update id in the event
    u:0, // first update id in the event
    pu: 0, // final update id in the stream
    b: vec![LevelData {price: 0.0, quantity: 0.0}], // bids
    a: vec![LevelData {price: 0.0, quantity: 0.0}], // asks
}
));

                                                                    

fn _print_type<T>(_: &T) {
    println!("Type: {}", type_name::<T>());
}

const HIBACHI_WS_URL: &str = "wss://data-api.hibachi.xyz/ws/market";
const WS_URL: &str = "wss://fstream.binance.com/";

async fn hibachi_orderbook_sub(ob_json_string: String, ob_url: Url) {
    match connect_async(ob_url.to_string()).await {
        Ok((mut ws_stream,_)) => {
            println!("Websocket connected");
            
            match ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(ob_json_string.into()))
                .await {
                    Ok(_) => println!("Message sent"),
                    Err(e) => println!("Message failed = {}", e),
                }

            while let Some(Ok(message)) = ws_stream.next().await {
                println!("Received {}", message);

                let _parsed_struct: HibachiOrderbook = serde_json::from_str(&message.to_string()).expect("Failed to parse JSON");
                if _parsed_struct.messageType == "Snapshot" {
                    // CURR_HIBACHI_OB = Lazy::new(|| Mutex::new(_parsed_struct.data));; // _parsed_struct.data;
                    let mut data_value = CURR_HIBACHI_OB.lock().unwrap();
                    data_value.ask = _parsed_struct.data.ask;
                    data_value.bid = _parsed_struct.data.bid;

                    println!("Updated orderbook: Snapshot: {:?}", data_value);
                }

                else if _parsed_struct.messageType == "Update" {

                    let mut data_value = CURR_HIBACHI_OB.lock().unwrap();
                    println!("Len of two messages: {} {}", _parsed_struct.data.ask.levels.len(), _parsed_struct.data.bid.levels.len());
                    
                    // LETS HANDLE THE BIDS FIRST

                    let mut curr_counter: usize = 0;
                    let mut update_counter: usize = 0;

                    println!("\nOrderbook updates incoming\n");

                    while (update_counter < (_parsed_struct.data.bid.levels.len())) && (curr_counter < (data_value.bid.levels.len())) {
                        if data_value.bid.levels[curr_counter].price == _parsed_struct.data.bid.levels[update_counter].price {
                            if _parsed_struct.data.bid.levels[update_counter].quantity == 0.0 {
                                update_counter += 1;

                                data_value.bid.levels.remove(curr_counter);
                            }
                            else {
                                data_value.bid.levels[curr_counter].quantity = _parsed_struct.data.bid.levels[update_counter].quantity;
                                
                                update_counter += 1;
                                curr_counter += 1;
                            }
                        }
                        else if data_value.bid.levels[curr_counter].price > _parsed_struct.data.bid.levels[update_counter].price {
                            curr_counter += 1;
                        }
                        else {
                            update_counter += 1;
                            curr_counter += 1;
                        }
                    }

                    println!("====== BID DONE ====== ASK NOW =======");
                    // LETS HANDLE THE ASKS NOW
                    curr_counter = 0;
                    update_counter = 0;

                    while (update_counter < (_parsed_struct.data.ask.levels.len())) && (curr_counter < (data_value.ask.levels.len())) {
                        if data_value.ask.levels[curr_counter].price == _parsed_struct.data.ask.levels[update_counter].price {
                            if _parsed_struct.data.ask.levels[update_counter].quantity == 0.0 {
                                update_counter += 1;

                                data_value.ask.levels.remove(curr_counter);
                            }
                            else {
                                data_value.ask.levels[curr_counter].quantity = _parsed_struct.data.ask.levels[update_counter].quantity;
                                
                                update_counter += 1;
                                curr_counter += 1;
                            }
                        }
                        else if data_value.ask.levels[curr_counter].price < _parsed_struct.data.ask.levels[update_counter].price {
                            curr_counter += 1;
                        }
                        else {
                            data_value.ask.levels.insert(curr_counter, LevelData { price: _parsed_struct.data.ask.levels[update_counter].price,
                                quantity: _parsed_struct.data.ask.levels[update_counter].quantity});

                            update_counter += 1;
                            curr_counter += 1;
                        }
                    }


                    println!("Updated orderbook: Update: {:?}", data_value);
                }
            };

        }
        Err(e) => {
            println!("Connection failed {}", e);
        }
    }

}

async fn hibachi_funding_rate_sub(ob_json_string: String, ob_url: Url) {
    match connect_async(ob_url.to_string()).await {
        Ok((mut ws_stream,_)) => {
            println!("Websocket connected");
            
            match ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(ob_json_string.into()))
                .await {
                    Ok(_) => println!("Message sent"),
                    Err(e) => println!("Message failed = {}", e),
                }
                
            while let Some(Ok(message)) = ws_stream.next().await {
                // println!("Received {}", message);

                // let start_time: Instant = Instant::now();
                let _parsed_struct: FundingRate = serde_json::from_str(&message.to_string()).expect("Failed to parse JSON");
                if true {
                    // CURR_HIBACHI_OB = Lazy::new(|| Mutex::new(_parsed_struct.data));; // _parsed_struct.data;
                    let mut data_value = CURR_FR.lock().unwrap();
                    data_value.estimatedFundingRate = _parsed_struct.data.fundingRateEstimation.estimatedFundingRate;
                    data_value.nextFundingTimestamp = _parsed_struct.data.fundingRateEstimation.nextFundingTimestamp;

                    println!("Updated funding rate: {:?}", data_value);
                }
                // let elapsed: Duration = start_time.elapsed();    
                // println!("Time taken to deserialize and parse: {:?}", elapsed);

            };

        }
        Err(e) => {
            println!("Connection failed {}", e);
        }
    }

}

async fn binance_orderbook_sub() {
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

                        let mut data_value = CURR_BINANCE_OB.lock().unwrap();
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

async fn binance_orderbook_ticker() {
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
                        let mut data_value = CURR_BINANCE_OB.lock().unwrap();

                        data_value.T = _parsed_struct.T;
                        data_value.E = _parsed_struct.E;
                        data_value.u = _parsed_struct.u;
                        data_value.e = _parsed_struct.e;

                        let first_bid: LevelData = data_value.b[0].clone();
                        let first_ask: LevelData = data_value.a[0].clone();

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

async fn ob_evaluation() {
    let mut curr_binance_ob = CURR_BINANCE_OB.lock().unwrap();
    let mut curr_hibachi_ob = CURR_HIBACHI_OB.lock().unwrap();

    
}

#[tokio::main]
async fn main() {
    println!("We are going to try to connect to a websocket a listen to Hibachi's messages");
    let url = Url::parse(HIBACHI_WS_URL).expect("Failed to parse a URL");

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

    let task_hibachi_ob = tokio::spawn(hibachi_orderbook_sub(ob_json_string, url.clone()));
    let task_hibachi_fr = tokio::spawn(hibachi_funding_rate_sub(fr_json_string, url.clone()));

    let task_binance_ob = tokio::spawn(binance_orderbook_sub());
    let task_binance_ticker = tokio::spawn(binance_orderbook_ticker());

    let task_ob_evaluation = tokio::spawn(ob_evaluation());

    let _ =tokio::join!(task_hibachi_ob, task_hibachi_fr, task_binance_ob, task_binance_ticker);
}

