use std::error::Error;

pub mod parsing;
pub mod tantivy;

use crate::parsing::parse::main_parse;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let link = vec!["123123123.com","https://www.nasa.gov"];

    let results: Vec<parsing::parse::SiteWords> = match main_parse(&link).await {
        Ok(res) => Ok(res),
        Err(e) => Err(Box::<dyn Error>::from(e)),
    }?;


    for site in results {
        println!("{:?}", site);
        match site.words{
            Some(words) => println!("{:?}", words),
            None => println!("None words")
        }
        match site.error {
            Some(e) => println!("Status Error {}", e),
            None => println!("Status Ok")
            
        }
    }

    Ok(())
}
