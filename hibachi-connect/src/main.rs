use std::env;
use reqwest::blocking::get;
use reqwest::Error;


const GLOBAL_CONSTANT: i32 = 5;

fn main() {
    let _hibachi_api_key: String = env::var("HIBACHI_API_KEY").expect("HIBACHI_API_KEY not set");
    let _hibachi_private_key: String = env::var("HIBACHI_PRIVATE_KEY").expect("HIBACHI_PRIVATE_KEY not set");
    println!("The value of the GLOBAL_CONSTANT is = {}", GLOBAL_CONSTANT);

    let response = check_get_request();
    println!("Response = {:?}", response);
}

fn check_get_request() -> Result <(), Error> {
    // Send a GET request
    let response: String = get("https://api.example.com/data")?
        .text()?;
    println!("Response: {}", response);
    Ok(())
}
