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
fn extract(body: &str) -> HashMap<String, String>{
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
                key = child.text().last().map(|s| s.trim_matches(' '));
            }
        }
        map.insert(key.unwrap().to_owned(), value.unwrap().to_owned()) ;
    }
    map
}
fn format_tuner(tuner: u16, signal: u16, quality:  u16) {}

async fn get_tuner_status(host: &str, tuner: u16) -> HashMap<String, String> {
    let url = format!("http://{}/tuners.html?page=tuner{}", host ,tuner);
    let body = match reqwest::get(url).await {
        Ok(res) => res.text().await.unwrap(),
        Err(_) => return HashMap::new(),
    };
    extract(body.as_str())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let args = Args::parse();
    let mut output_map = HashMap::new() ;
    if let Some(tuner) = args.tuner {
        let map = get_tuner_status(&args.host, tuner).await;
        let signal = map.get("Signal Strength").unwrap();
        let quality = map.get("Signal Quality").unwrap();
        let channel = map.get("Virtual Channel").unwrap();
        output_map.insert("text", format!("Tuner{tuner}: {channel} "));
        output_map.insert("alt", format!("{channel}: {signal} @ {quality}"));
    } else {
        let mut maps = Vec::new();
        for i in 0..4 {
            let map = get_tuner_status(&args.host, i).await;
            maps.push(map); 
        }  
        {
            // calculate average signal strength
            let signals:Vec<u16> = maps.iter()
                .map(|map| map.get(&format!("Signal Strength")).unwrap().trim_end_matches('%'))
                .filter(|value| value != &"none")
                .map(|s| s.parse::<u16>().unwrap()).collect();
            let avg_signal: u16 =  signals.iter().sum::<u16>() / signals.len() as u16;
            output_map.insert("text", format!("Signal: {}", avg_signal));

        }
        {
            // calculate average signal quality 
            let qualities: Vec<u16>= maps.iter()
                .map(|map| map.get(&format!("Signal Quality")).unwrap().trim_end_matches('%'))
                .filter(|value| value != &"none")
                .map(|s|s.parse::<u16>().unwrap()).collect();
            let avg_quality = qualities.iter().sum::<u16>() / qualities.len() as u16; 
            output_map.insert("alt", format!("Quality: {}", avg_quality));
        }
        { 
            let channels: Vec<String> = maps.iter()
                .map(|map| map.get(&format!("Virtual Channel")).unwrap().to_owned())
                .collect();
            output_map.insert("tooltip", format!("{}", channels.join("\n")));
        }
    }
    println!("{output_map:?}");
    let _ = stdout().flush();
    Ok(())
}
