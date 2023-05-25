use clap::Parser;

#[derive(Parser)]
struct Args {
    currency_in: Option<String>,
    currency_out: Option<String>,
    amount: Option<u32>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if let Some(currency_in) = args.currency_in {
        println!("Value: {:}", currency_in);
    } else {
        println!("Empty");
    }
    let params = [("apikey", "")];
    let url = reqwest::Url::parse_with_params("https://api.freecurrencyapi.com/v1/status", &params)?;
    let body = reqwest::get(url.as_str())
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);

    Ok(())
}
