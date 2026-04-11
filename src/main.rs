use std::error::Error;
use texting_robots::Robot;
use url::Url;

pub mod parsing;

use crate::parsing::parse::parsing;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let link = "https://www.nasa.gov/";

    
    if can_i_crawl(&link).await {
        let results = parsing(&link).await?;
        println!("{:?}", results);
    }
    else {
        println!("Cannot scrap site")
    }



    Ok(())
}


pub async fn can_i_crawl(link: &str) -> bool {
    let user_agent = "OpenQweryBot";

    let parsed_url = match Url::parse(link) {
        Ok(u) => u,
        Err(_) => return false,
    };

    let robots_url = format!(
        "{}://{}/robots.txt", 
        parsed_url.scheme(), 
        parsed_url.host_str().unwrap_or("")
    );

    if let Ok(response) = reqwest::get(&robots_url).await {
    if let Ok(text) = response.text().await {
        match Robot::new(user_agent, text.as_bytes()) {
            Ok(r) => {
                return r.allowed(parsed_url.path());
            }
            Err(_) => {

                return true; 
            }
        }
    }
}

    true 
}