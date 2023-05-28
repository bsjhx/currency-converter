use dotenv::dotenv;
use clap::Parser;
use serde::{Deserialize, Serialize};
use reqwest::header::CONTENT_TYPE;

mod rest_api;

#[derive(Parser)]
struct Args {
    currency_in: Option<String>,
    currency_out: Option<String>,
    amount: Option<u32>
}

#[derive(Serialize, Deserialize, Debug)]
struct Currencies {
    data: serde_json::Value,
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

    let response = rest_api::rest_api_caller::call_currencies().await?;

    if let Some(data) = response.get("data") {
        let c = data.as_object().unwrap();

        println!("All available currencies:");
        for a1 in c.iter() {
            println!("{:}", a1.0.as_str());
        }
    } else {
        println!("Error");
    }

    Ok(())
}
