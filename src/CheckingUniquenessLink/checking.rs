
use std::error::Error;
use std::{collections::HashSet};
use once_cell::sync::{Lazy};
use std::sync::{Mutex}; 
use std::fs::File;
use std::io::{BufReader, BufWriter};

use crate::other::fxhash::fxhash;

static ALL_LINKS: Lazy<Mutex<HashSet<u64>>> = Lazy::new(||{Mutex::new(HashSet::new())});

static COUTER_LINK: Lazy<Mutex<u64>> = Lazy::new(||Mutex::new(0));


pub async fn check_link(link: &str) -> Result<bool, Box<dyn Error>>{

    let hesh_lik = fxhash(&link).await;

    save_links().await?;

    let mut all_links: std::sync::MutexGuard<'_, HashSet<u64>> = ALL_LINKS.lock()?;

    if all_links.contains(&hesh_lik){
        return Ok(false);
    }
    else {
        all_links.insert(hesh_lik);
        *COUTER_LINK.lock()? += 1;
        return Ok(true);
    }
    

}


// Save And Load
const DB_PATH: &str = "links.bin";

pub async fn save_links() -> Result<(), Box<dyn Error>> {
    
    let hash_set = ALL_LINKS.lock()?;
        

    let file = BufWriter::new(File::create(DB_PATH)?); 

    bincode::serialize_into(file, &*hash_set)?;
    
    
    Ok(())
}

pub async fn load_links() -> Result<(), Box<dyn Error>> {
    
    if std::path::Path::new(DB_PATH).exists() {

        let file = BufReader::new(File::open(DB_PATH)?);
        

        let loaded_set: HashSet<u64> = match bincode::deserialize_from(file) {
            Ok(set) => set,
            Err(e) => {
                println!("Error parse: {} \n{}",DB_PATH, e);
                return Ok(()); 
            }
        };
        
        let mut all_links = ALL_LINKS.lock()?;
        let count = loaded_set.len();

        *COUTER_LINK.lock()? = count as u64;

        *all_links = loaded_set;
        
    }
    Ok(())
}