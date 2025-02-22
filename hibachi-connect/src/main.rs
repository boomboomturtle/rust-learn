use std::env;

const GLOBAL_CONSTANT: i32 = 5;

fn main() {
    let _hibachi_api_key: String = env::var("HIBACHI_API_KEY").expect("HIBACHI_API_KEY not set");
    let _hibachi_private_key: String = env::var("HIBACHI_PRIVATE_KEY").expect("HIBACHI_PRIVATE_KEY not set");
    println!("The value of the GLOBAL_CONSTANT is = {}", GLOBAL_CONSTANT);
}
