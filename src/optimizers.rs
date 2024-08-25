use std::collections::{HashMap, VecDeque};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: u32,
    pub name: String,
    pub age: u32,
    pub interests: Option<Vec<String>>, // Change to Option<Vec<String>>
}

pub struct Optimizer;

impl Optimizer {
    pub fn new() -> Self {
        Optimizer
    }

    pub fn query(&self, user_db: &HashMap<u32, UserProfile>, query: &[String], limit: usize, min_age: u32, max_age: u32) -> Vec<UserProfile> {
        user_db.values()
            .filter(|profile| {
                profile.age >= min_age && 
                profile.age <= max_age &&
                profile.interests.as_ref().map_or(false, |interests| interests.iter().any(|interest| query.contains(interest))) // Handle Option
            })
            .take(limit)
            .cloned()
            .collect()
    }
}

pub struct LRUCache {
    capacity: usize,
    map: HashMap<String, Vec<UserProfile>>,
    order: VecDeque<String>,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&Vec<UserProfile>> {
        if let Some(value) = self.map.get(key) {
            self.order.retain(|x| x != key);
            self.order.push_back(key.to_string());
            Some(value)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: String, value: Vec<UserProfile>) {
        if self.map.len() == self.capacity && !self.map.contains_key(&key) {
            if let Some(lru) = self.order.pop_front() {
                self.map.remove(&lru);
            }
        }
        self.map.insert(key.clone(), value);
        self.order.retain(|x| x != &key);
        self.order.push_back(key);
    }
}
