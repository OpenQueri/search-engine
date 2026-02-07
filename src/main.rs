use reqwest;
use scraper::{Html, Selector};
use regex::Regex;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let url = "https://www.nasa.gov";
    let body = reqwest::get(url).await.unwrap().text().await.unwrap();

    let document = Html::parse_document(&body);
    let selector = Selector::parse("h1, h2, h3, p").unwrap();
    
    let mut text_vec = Vec::new();

    for element in document.select(&selector){
        let text = element.text().collect::<String>();
        text_vec.push(text);
    }
    println!("input {:?}", text_vec);
    let twxt = extract_words(text_vec).await;
    println!("HeshSet {:?}", twxt);

}

async fn extract_words(text: Vec<String>) -> HashMap<String, usize>
{
    let re = Regex::new(r"\p{L}+").unwrap();
    let mut words: HashMap<String, usize> = HashMap::new();


    for line in text.iter() {
        for mat in re.find_iter(line) {
            let word = mat.as_str().to_lowercase();
            *words.entry(word).or_insert(0) += 1;
        }
    }
    
    words

}