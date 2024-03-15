use scraper::{Html, Selector, };
use std::collections::HashMap;
use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    tuner: u8,
}
#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let args = Args::parse();
    let url = format!("http://10.0.0.27/tuners.html?page=tuner{}", args.tuner);
    let res = reqwest::get(url).await?;
    let body = res.text().await?;
    let document = Html::parse_document(body.as_str());
    let table_selector = Selector::parse("table").unwrap();
    let row_selector = Selector::parse("tr:not(.L)").unwrap();
    let table = document.select(&table_selector).next().unwrap();
    let mut map = HashMap::new();
    for row in table.select(&row_selector) {
        let mut key: Option<&str> = None; 
        let mut value: Option<&str> = None;
        for child in row.child_elements() {
            if key.is_some() {
                value = child.text().last();
                break;
            } else {
                key = child.text().last();
            }
        }
        map.insert(key.unwrap().to_lowercase(), value.unwrap()) ;
    }
    let signal = map.get("signal strength").unwrap();
    let qualty = map.get("signal quality").unwrap();
    let mut output_map = HashMap::new() ;
    output_map.insert("text", format!("Signal: {}", signal));
    output_map.insert("alt", format!("Quality: {}", qualty));
    println!("{output_map:?}");
    Ok(())
}
