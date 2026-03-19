use reqwest::blocking::Client;
use serde::Deserialize;

const RANDOM_QUOTES_URL: &str = "https://api.api-ninjas.com/v2/randomquotes";

#[derive(Debug, Deserialize)]
struct QuoteItem {
    quote: String,
    author: String,
}

pub fn handle_random_quote_command() {
    // 100% unimportant key... dont wowrry
    let key = "lj7adMx1VsObQDX9qcNKn8z0hXmUYfESLeEcaBIW";

    let client = Client::new();
    let response = client
        .get(RANDOM_QUOTES_URL)
        .header("X-Api-Key", key)
        .send()
        .and_then(|res| res.error_for_status());

    let quote = match response {
        Ok(res) => match res.json::<Vec<QuoteItem>>() {
            Ok(mut items) => items.pop(),
            Err(err) => {
                eprintln!("Could not parse quote response: {err}");
                return;
            }
        },
        Err(err) => {
            eprintln!("Could not fetch quote: {err}");
            return;
        }
    };

    if let Some(item) = quote {
        println!("\"{}\" \n– {}", item.quote, item.author);
    } else {
        eprintln!("Quote API returned no data.");
    }
}

pub fn handle_create_quote_command(quote: &str, author: &str) {
    println!("\"{}\" \n– {}", quote, author)
}