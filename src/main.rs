use clap::Parser;

#[derive(Parser)]
struct Args {
    currency_in: Option<String>,
    currency_out: Option<String>,
    amount: Option<u32>
}


fn main() {
    let args = Args::parse();
    if let Some(currency_in) = args.currency_in {
        println!("Value: {:}", currency_in);
    } else {
        println!("Empty");
    }
}
