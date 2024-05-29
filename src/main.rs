extern crate reqwest;
extern crate dotenv;
extern crate clap;

use reqwest::header::{HeaderMap, UPGRADE_INSECURE_REQUESTS};
use dotenv::dotenv;
use std::env;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The API endpoint version to use for v3
    #[clap(long)]
    endpoint: Option<String>,
    /// Query input
    #[clap(long)]
    query: Option<String>,
    /// The optional v4 endpoint to use v4 API features
    #[clap(long)]
    v4endpoint: Option<String>,
}

async fn get_stock_info_v3(endpoint: &str, symbol: &str) -> Result<String, reqwest::Error> {
    dotenv().ok(); // Load environment variables from .env file

    // Not sure if I can just set this once in a global or something similar
    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    let mut headers = HeaderMap::new();
    headers.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());

    let url = format!(
        "https://financialmodelingprep.com/api/v3/{}/{}?apikey={}",
        endpoint, symbol, api_key
    );

    let res = reqwest::Client::new()
    .get(&url)
    .headers(headers)
    .send()
    .await? // Await to get future responses
    .text()
    .await?;


    Ok(res)
}

async fn get_stock_info_v4(v4endpoint: &str, symbol: &str) -> Result<String, reqwest::Error> {
    dotenv().ok(); // Load environment variables from .env file

    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    let mut headers = HeaderMap::new();
    headers.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());

    let url = format!(
        "https://financialmodelingprep.com/api/v4/{}?symbol={}&apikey={}",
        v4endpoint, symbol, api_key
    );


    let res = reqwest::Client::new()
    .get(&url)
    .headers(headers)
    .send()
    .await? // Await to get future responses
    .text()
    .await?;


    Ok(res)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Cli::parse();

    if let Some(endpoint) = args.endpoint {
        if let Some(query) = args.query {
            match get_stock_info_v3(&endpoint, &query).await {
                Ok(result) => println!("Response: {}", result),
                Err(e) => eprintln!("Error: {}", e),
            }
        } else {
            println!("Query parameter is required for v3 endpoint");
        }
    } else if let Some(v4endpoint) = args.v4endpoint {
        if let Some(query) = args.query {
            match get_stock_info_v4(&v4endpoint, &query).await {
                Ok(result) => println!("Response: {}", result),
                Err(e) => eprintln!("Error: {}", e),
            }
        } else {
            println!("Query parameter is required for v4 endpoint");
        }
    } else {
        println!("No endpoint specified");
    }

    Ok(())
}
