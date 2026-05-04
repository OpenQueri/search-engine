use std::error::Error;
use texting_robots::Robot;
use url::Url;

pub mod Parsing;
pub mod CheckingUniquenessLink;
pub mod other;

use Parsing::parse::{parsing, DataSite,TraversalResult};
use CheckingUniquenessLink::checking::{check_link,load_links};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct DataSiteResponse {
    pub title: String,
    pub img: Vec<String>,
    pub meta: Vec<String>,
    pub text: Vec<String>,
}


pub async fn link_scrap(link: &String,depth_limit: u32) -> Result<Vec<DataSiteResponse>, Box<dyn Error>>{

    let mut queue = VecDeque::new();

    let mut response_result: Vec<DataSiteResponse> = Vec::new();


    queue.push_back((link.to_string(), 0));

    while let Some((current_url, mut current_depth)) = queue.pop_front() {
        
        if current_depth >= depth_limit { 
            continue; 
        }
 
        let result = scrap(&current_url, &mut current_depth).await;

        match result {
            Ok(Some(res)) => {
                response_result.push(DataSiteResponse { 
                     title: res.title,
                     img: res.img,
                     meta: res.meta,
                     text: res.text 
                });


                for found_link in res.url { 
                    queue.push_back((found_link, current_depth)); 
                }
            },
            Ok(None) => println!("None content at {}", current_url),
            Err(e) => println!("Error: {}", e),
        }
    }
    Ok(response_result)
}


pub async fn scrap(link: &str, depth: &mut u32) -> Result<Option<DataSite>, Box<dyn Error>>{
    *depth += 1;

    if check_link(&link).await? == true{
        if can_i_crawl(&link).await {
            tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
            let results = parsing(&link).await?;

            match results {
                Some(result) => {
                    match result.traversal_result {
                        TraversalResult::Successfully => {
                            return Ok(Some(result));
                        },
                        TraversalResult::ThisSiteCannotScraper => {
                            return Ok(None);
                        },
                        
                    }
                
                },

                None => {return Ok(None);}
            }
            
        }
        else {
            println!("Cannot scrap site");
            return Ok(None);
        }
    }
    else {
        println!("This site has already created a response and will not create another");
        return Ok(None);
    }

    
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