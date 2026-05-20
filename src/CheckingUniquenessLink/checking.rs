use std::error::Error;
use std::collections::HashSet;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use sled::Db;
use crate::other::fxhash::fxhash;

static DB: Lazy<Db> = Lazy::new(|| {
    sled::open(DB_NAME).expect("Помилка при створенні бази даних")
});

static ALL_LINKS: Lazy<Mutex<HashSet<u64>>> = Lazy::new(|| Mutex::new(HashSet::new()));
static COUNTER_LINK: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

const DB_NAME: &str = "search_engine_db";
const TREE_UNIQUE_LINKS: &str = "unique_links";

fn get_unique_tree() -> sled::Tree {
    DB.open_tree(TREE_UNIQUE_LINKS).expect("Помилка відкриття дерева")
}

pub async fn check_link(link: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let hesh_lik = fxhash(link).await;

    let is_new = {
        let mut all_links = ALL_LINKS.lock().map_err(|_| "Mutex lock error")?;
        if all_links.contains(&hesh_lik) {
            false
        } else {
            all_links.insert(hesh_lik);
            if let Ok(mut count) = COUNTER_LINK.lock() {
                *count += 1;
            }
            true 
        }
    }; 

    if is_new {
        let tree = get_unique_tree();
        tree.insert(hesh_lik.to_be_bytes(), &[1])?;
        tree.flush_async().await?;
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn load_links() -> Result<(), Box<dyn Error + Send + Sync>> {
    let tree = get_unique_tree();
    let mut all_links = ALL_LINKS.lock().map_err(|_| "Mutex lock error")?;
    let mut count = 0;

    for item in tree.iter() {
        let (k, _) = item?;
        let hesh = u64::from_be_bytes(k.as_ref().try_into()?);
        all_links.insert(hesh);
        count += 1;
    }

    if let Ok(mut c_lock) = COUNTER_LINK.lock() {
        *c_lock = count;
    }

    println!("Sled: завантажено {} унікальних хешів посилань", count);
    Ok(())
}

pub async fn save_links() -> Result<(), Box<dyn Error + Send + Sync>> {
    let tree = get_unique_tree();
    let all_links = ALL_LINKS.lock().map_err(|_| "Mutex lock error")?;
    
    for hesh in all_links.iter() {
        tree.insert(hesh.to_be_bytes(), &[1])?;
    }
    tree.flush_async().await?;
    Ok(())
}