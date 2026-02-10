use scraper::{Html, Selector};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

#[derive(Debug)]
struct SiteWords{
    url: String,
    words: Vec<(String, usize)>
}

static WORD_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\p{L}+").expect("Invalid regex pattern")
});

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let link = vec!["https://www.nasa.gov"];

    let tasks: Vec<_> = link.iter().map(|&url| parse(url)).collect();
    let results = futures::future::try_join_all(tasks).await?;


    for site in results {
        println!("{:?}", site);
        println!("✅ {}: {} слов", site.url, site.words.len());
    }
    
    Ok(())
}

async fn parse(link: &str) -> Result<SiteWords, Box<dyn Error>>{

    let body = reqwest::get(link).await?.text().await?;

    let document = Html::parse_document(&body);
    let selector = Selector::parse("h1, h2, h3, p")?;
    
    let mut text_fragments = Vec::new();

    for element in document.select(&selector){
        for text_piece in element.text(){
            text_fragments.push(text_piece);
        }
    }

    let words = extract_words(&text_fragments).await?;

    Ok(SiteWords{
        url: link.to_string(),
        words: words,
    })

}

async fn extract_words(text_fragments: &[&str]) -> Result<Vec<(String, usize)>, Box<dyn Error>>
{
    let mut words: HashMap<String, usize> = HashMap::new();

    for &fragment in text_fragments {
        WORD_REGEX
            .find_iter(fragment)
            .map(|mat: regex::Match<'_>| mat.as_str().to_lowercase())
            .for_each(|word| {
                *words.entry(word).or_insert(0) += 1;
            });
    }
    let vec: Vec<(String, usize)> = (*words.into_iter().collect::<Vec<(String, usize)>>()).to_vec();
    
    Ok(vec)
}