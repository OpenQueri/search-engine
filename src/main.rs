use std::error::Error;

pub mod parsing;

use crate::parsing::parse::parsing;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let link = "https://www.nasa.gov";

    let results = parsing(&link).await?;

    

    println!("{:?}", results);

    Ok(())
}
