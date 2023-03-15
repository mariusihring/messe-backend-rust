use rand::Rng;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
pub mod generat_data;
pub mod guard;
pub async fn generate_token() -> String {
    let mut rng = rand::thread_rng();
    let random_number: u64 = rng.gen();
    format!("{:016x}", random_number)
}
pub async fn hasher(input: String) -> String {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}
