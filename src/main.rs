mod optimizer;
mod vector_cache;

use crate::optimizer::{Optimizer, UserProfile, LRUCache};
use crate::vector_cache::VectorCache;
use rand::Rng;
use std::collections::HashMap;

fn generate_random_embedding(dim: usize) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    (0..dim).map(|_| rng.gen()).collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let optimizer = Optimizer::new();
    let vector_cache = VectorCache::new("user_profiles").await?;

    let user_profiles = vec![
        UserProfile { id: 1, name: "Alice Johnson".to_string(), age: 30, interests: vec!["reading".to_string(), "hiking".to_string()].into() },
        UserProfile { id: 2, name: "Bob Smith".to_string(), age: 25, interests: vec!["gaming".to_string(), "cooking".to_string()].into() },
        UserProfile { id: 3, name: "Charlie Brown".to_string(), age: 35, interests: vec!["traveling".to_string(), "photography".to_string()].into() },
        UserProfile { id: 4, name: "David Wilson".to_string(), age: 28, interests: vec!["hiking".to_string(), "photography".to_string()].into() },
    ];

    // Add user profiles to vector cache
    for profile in &user_profiles {
        let embedding = generate_random_embedding(128); // In real scenario, generate embedding from profile data
        vector_cache.add(&profile.id.to_string(), embedding, profile).await?;
    }

    // Simulate a query
    let query_embedding = generate_random_embedding(128);
    let results = vector_cache.search(query_embedding, 2).await?;

    println!("Vector cache search results:");
    for (distance, profile) in results {
        println!("Distance: {:.4}, Profile: {:?}", distance, profile);
    }

    // Use the optimizer's query method
    let user_db: HashMap<u32, UserProfile> = user_profiles.into_iter().map(|profile| (profile.id, profile)).collect();
    let query_interests = vec!["hiking".to_string()];
    let filtered_profiles = optimizer.query(&user_db, &query_interests, 2, 20, 40);
    println!("Filtered profiles: {:?}", filtered_profiles);

    // Use the LRUCache
    let mut lru_cache = LRUCache::new(2);
    lru_cache.put("key1".to_string(), vec![UserProfile { id: 5, name: "Eve Adams".to_string(), age: 22, interests: vec!["music".to_string()].into() }]);
    lru_cache.put("key2".to_string(), vec![UserProfile { id: 6, name: "Frank White".to_string(), age: 29, interests: vec!["sports".to_string()].into() }]);
    if let Some(cached_profiles) = lru_cache.get("key1") {
        println!("Cached profiles for key1: {:?}", cached_profiles);
    }

    // Return Ok to satisfy the Result return type
    Ok(())
}
