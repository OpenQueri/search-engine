
use std::hash::Hash;
use fxhash::FxHasher;
use std::hash::Hasher;


pub async fn fxhash(link: &str) -> u64 {
        let mut hasher = FxHasher::default();
        link.hash(&mut hasher);
        hasher.finish()
}