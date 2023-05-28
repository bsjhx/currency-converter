use reqwest::header::CONTENT_TYPE;
use serde_json::Value;
use anyhow::Result;

pub async fn call_currencies() -> Result<Value> {
    let api_key = std::env::var("API_KEY").expect("API_KEY enviroment variable must be set!");
    let params = [("apikey", api_key)];
    let url = reqwest::Url::parse_with_params("https://api.freecurrencyapi.com/v1/currencies", &params)?;
    let client = reqwest::Client::new();
    let response = client.get(url.as_str())
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(response)
}