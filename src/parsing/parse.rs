use scraper::{Html, Selector};
use std::error::Error;

#[derive(Debug)]
enum TraversalResult {
    Successfully,
    ThisSiteCannotScraper    
}

#[derive(Debug)]
pub struct DataSite {
    title: String,
    url: Vec<String>,
    img: Vec<String>,
    meta: Vec<String>,
    text: Vec<String>,
    traversal_result: TraversalResult
}

impl DataSite {
    fn new() -> Self {
        Self { 
            title: String::new(),
            url: Vec::new(),
            img: Vec::new(),
            meta: Vec::new(),
            text: Vec::new(),
            traversal_result: TraversalResult::Successfully,
        }
    }
    
    // Setters for collected data
    fn add_title(&mut self, title: String) { self.title = title; }
    fn add_url(&mut self, url: String) { self.url.push(url); }
    fn add_img(&mut self, img: String) { self.img.push(img); }
    fn add_meta(&mut self, meta: String) { self.meta.push(meta); }
    fn add_text(&mut self, text: String) { self.text.push(text); }
    
    // Update status if scraping is restricted
    fn cannot_scraper(&mut self) {
        self.traversal_result = TraversalResult::ThisSiteCannotScraper;
    }
}

pub async fn parsing(link: &str) -> Result<Option<DataSite>, Box<dyn Error>> {
    // Fetch HTML body from the provided link
    let body = reqwest::get(link).await?.text().await?;

    // Parse document and define selectors for search engine indexing
    let document = Html::parse_document(&body);
    let text_selector = Selector::parse("title, h1, h2, h3, h4, h5, h6, p, img, meta, a")?;

    let mut result = DataSite::new();

    // Iterate through all matched elements
    for element in document.select(&text_selector) {
        let tag_element = element.value().name();

        match tag_element {
            "title" => {
                // Extract main page title
                let title_text = element.text().collect::<Vec<_>>().join("");
                if !title_text.is_empty() {
                    result.add_title(title_text.trim().to_string());
                }
            }
            "a" => {
                // Extract links for crawler queue
                let attrs = element.value();
                if let Some(url) = attrs.attr("href") {
                    let rel = attrs.attr("rel").unwrap_or("");
                    
                    // Filter out empty links, anchors and nofollow attributes
                    if !url.is_empty() && !url.starts_with('#') && !rel.contains("nofollow") {
                        result.add_url(url.to_string());
                    }
                }
            }
            "img" => {
                // Collect image sources
                if let Some(src) = element.value().attr("src") {
                    result.add_img(src.to_string());
                }
            }
            "meta" => {
                // Check robots meta tags for crawling permissions
                let name = element.value().attr("name").unwrap_or("");
                let content = element.value().attr("content").unwrap_or("");

                if name == "robots" && content.contains("nofollow") {
                    result.cannot_scraper();
                    return Ok(Some(result)); // Stop parsing if forbidden
                }
                
                // Collect other metadata
                if let Some(meta_val) = element.value().attr("content") {
                    result.add_meta(meta_val.to_string());
                }
            }
            _ => {
                // Collect clean text from headings and paragraphs
                let text = element.text().collect::<Vec<_>>().join("");
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    result.add_text(trimmed.to_string());
                }
            }
        }
    }

    Ok(Some(result))
}