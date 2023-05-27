use dotenv::dotenv;
use std::collections::HashMap;
use clap::Parser;
use serde::{Deserialize, Serialize};
use reqwest::header::CONTENT_TYPE;

#[derive(Parser)]
struct Args {
    currency_in: Option<String>,
    currency_out: Option<String>,
    amount: Option<u32>
}

#[derive(Serialize, Deserialize, Debug)]
struct JSONResponse {
    data: serde_json::Number,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args = Args::parse();
    // if let Some(currency_in) = args.currency_in {
    //     println!("Value: {:}", currency_in);
    // } else {
    //     println!("Empty");
    // }

    let api_key = std::env::var("API_KEY").expect("API_KEY enviroment variable must be set!");
    let params = [("apikey", api_key)];
    let url = reqwest::Url::parse_with_params("https://api.freecurrencyapi.com/v1/currencies", &params)?;
    let client = reqwest::Client::new();
    let response = client.get(url.as_str())
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let a = response.get("data");
    if a.is_some() {
        let b = a.unwrap();
        let c = b.as_object().unwrap();

        println!("All available currencies:");
        for a1 in c.iter() {
            println!("{:}", a1.0.as_str());
        }
    } else {
        println!("{:#?}", a);
    }

    // let res: JSONResponse = serde_json::from_str(&response)?;

    Ok(())
}
