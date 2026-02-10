use std::error::Error;

pub mod parsing;


use crate::parsing::parse::main_parse;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let link = vec!["https://www.nasa.gov"];

    let results: Vec<parsing::parse::SiteWords> = main_parse(&link).await?;


    for site in results {
        println!("{:?}", site);
        println!("✅ {}: {} слов", site.url, site.words.len());
    }

    Ok(())
}
