use scraper::{Html, Selector, };
use std::io::Write;
use std::io::stdout;
use std::collections::HashMap;
use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    host: String,
    #[arg(short, long)]
    tuner: Option<u16>,
}
fn extract(body: &str) -> HashMap<String,String>{
    let mut map = HashMap::new();
        let document = Html::parse_document(body);
        let table_selector = Selector::parse("table").unwrap();
        let row_selector = Selector::parse("tr:not(.L)").unwrap();
        let table = document.select(&table_selector).next().unwrap();
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
            map.insert(key.unwrap().to_lowercase().to_owned(), value.unwrap().to_owned()) ;
        }
    map
}
#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let args = Args::parse();
    let mut output_map = HashMap::new() ;
    if let Some(tuner) = args.tuner {
        let url = format!("http://{}/tuners.html?page=tuner{}",args.host ,tuner);
        let res = reqwest::get(url).await?;
        let body = res.text().await?;
        let map = extract(body.as_str());
        let signal = map.get("signal strength").unwrap();
        let qualty = map.get("signal quality").unwrap();
        output_map.insert("text", format!("Signal: {}", signal));
        output_map.insert("alt", format!("Quality: {}", qualty));

    } else {
        let mut maps = Vec::new();
        for i in 0..4 {
            let url = format!("http://{}/tuners.html?page=tuner{}",args.host ,i);
            let res = reqwest::get(url).await?;
            let body = res.text().await?;
            let map = extract(body.as_str());
            maps.push(map); 
        }  
        let signals:Vec<u16> = maps.iter()
                .map(|map|
                    
                   map.get(&format!("signal strength")).unwrap().trim_end_matches('%')
                )
                .filter(|value| value != &"none")
                .map(|s| s.parse::<u16>().unwrap()).collect();
        let avg_signal: u16 =  signals.iter().sum::<u16>() / signals.len() as u16;
        let qualities: Vec<u16>= maps.iter()
            .map(|map| map.get(&format!("signal quality")).unwrap().trim_end_matches('%'))
            .filter(|value| value != &"none")
            .map(|s|s.parse::<u16>().unwrap()).collect();
        let avg_quality = qualities.iter().sum::<u16>() / qualities.len() as u16; 
        output_map.insert("text", format!("Signal: {}", avg_signal));
        output_map.insert("alt", format!("Quality: {}", avg_quality));
    }
    println!("{output_map:?}");
    let _ = stdout().flush();
    Ok(())
}
