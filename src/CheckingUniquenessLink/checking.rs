use std::error::Error;
use std::collections::HashSet;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use sled::Db;

use crate::other::fxhash::fxhash;

static ALL_LINKS: Lazy<Mutex<HashSet<u64>>> = Lazy::new(|| Mutex::new(HashSet::new()));
static COUTER_LINK: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

const DB_NAME: &str = "search_engine_db";
const TREE_UNIQUE_LINKS: &str = "unique_links";

/// Відкриває дерево унікальних посилань
fn open_unique_tree() -> Result<sled::Tree, Box<dyn Error>> {
    let db = sled::open(DB_NAME)?;
    let tree = db.open_tree(TREE_UNIQUE_LINKS)?;
    Ok(tree)
}

pub async fn check_link(link: &str) -> Result<bool, Box<dyn Error>> {
    let hesh_lik = fxhash(&link).await;

    // Створюємо окрему область видимості
    let is_new = {
        let mut all_links = ALL_LINKS.lock().map_err(|_| "Mutex lock error")?;
        if all_links.contains(&hesh_lik) {
            false
        } else {
            all_links.insert(hesh_lik);
            if let Ok(mut count) = COUTER_LINK.lock() {
                *count += 1;
            }
            true // Повертаємо результат з блоку
        }
    }; // ТУТ MutexGuard ГАРАНТОВАНО ПОМЕР

    if is_new {
        // Тільки тепер, коли замок вільний, працюємо з Sled
        let tree = open_unique_tree()?;
        tree.insert(hesh_lik.to_be_bytes(), &[1])?;
        tree.flush_async().await?; // Тепер .await безпечний!
        Ok(true)
    } else {
        Ok(false)
    }
}

// Функція збереження тепер по суті не потрібна для постійного виклику, 
// бо ми зберігаємо по одному в check_link. Але залишимо її для сумісності.
pub async fn save_links() -> Result<(), Box<dyn Error>> {
    let tree = open_unique_tree()?;
    let all_links = ALL_LINKS.lock().map_err(|_| "Mutex lock error")?;
    
    for hesh in all_links.iter() {
        tree.insert(hesh.to_be_bytes(), &[1])?;
    }
    tree.flush_async().await?;
    Ok(())
}

pub async fn load_links() -> Result<(), Box<dyn Error>> {
    if !std::path::Path::new(DB_NAME).exists() {
        return Ok(());
    }

    let tree = open_unique_tree()?;
    let mut all_links = ALL_LINKS.lock().map_err(|_| "Mutex lock error")?;
    
    let mut count = 0;
    for item in tree.iter() {
        let (k, _) = item?;
        let hesh = u64::from_be_bytes(k.as_ref().try_into()?);
        all_links.insert(hesh);
        count += 1;
    }

    if let Ok(mut c_lock) = COUTER_LINK.lock() {
        *c_lock = count;
    }

    println!("Sled: завантажено {} унікальних хешів посилань", count);
    Ok(())
}