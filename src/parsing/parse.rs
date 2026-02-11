
use scraper::{Html, Selector};
use regex::Regex;
use std::{collections::HashMap};
use once_cell::sync::Lazy;
use std::error::Error;
#[derive(Debug)]
pub struct SiteWords{
    pub url: String,
    pub words: Option<Vec<(String, usize)>>,
    pub error: Option<String>,
}


static WORD_REGEX: Lazy<Option<Regex>> = Lazy::new(|| {
    Regex::new(r"\p{L}+").ok()
});

pub async fn main_parse(site_vec: &[&str]) -> Result< Vec<SiteWords> ,Box<dyn Error>>{


    let tasks: Vec<_> = site_vec
    .iter()
    .map(|url| {
            let url = url.to_string();
            async move {
                match parsing(&url).await {
                    Ok(mut site) => {
                        site.url = url;
                        site
                    },
                    Err(e) => SiteWords {
                        url,
                        words: None,
                        error: Some(e.to_string()),
                    }
                }
            }
        })
        .collect();

    let results = futures::future::join_all(tasks).await;

    Ok(results)
}

async fn parsing(link: &str) -> Result<SiteWords, Box<dyn Error>>{

    let body = reqwest::get(link).await?.text().await?;

    let document = Html::parse_document(&body);
    let selector = Selector::parse("h1, h2, h3, p")?;
    
    let mut text_fragments = Vec::new();

    for element in document.select(&selector){
        for text_piece in element.text(){
            text_fragments.push(text_piece);
        }
    }

    match extract_words(&text_fragments).await {
        Ok(words) => { 
            Ok(SiteWords{
                    url: link.to_string(),
                    words: Some(words),
                    error: None
                })
        },
        Err(e) => {
            eprintln!("Error {}", e);
            Ok(SiteWords{
                        url: link.to_string(),
                        words: None,
                        error: Some(format!("{}", e)),
                    })
            },
    }


}

async fn extract_words(text_fragments: &[&str]) -> Result<Vec<(String, usize)>, Box<dyn Error>>
{
    let regex = match WORD_REGEX.as_ref() {
        Some(re) => re,
        None => return Err("WORD_REGEX no compile".into())
    };
    let mut words: HashMap<String, usize> = HashMap::new();

    for &fragment in text_fragments {
        regex
            .find_iter(fragment)
            .map(|mat: regex::Match<'_>| mat.as_str().to_lowercase())
            .for_each(|word| {
                *words.entry(word).or_insert(0) += 1;
            });
    }    
    Ok(words.into_iter().collect())
}