use std::collections::HashMap;
use std::collections::VecDeque;

pub struct LRUCache {
    capacity: usize,
    map: HashMap<String, Vec<f64>>,
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

    pub fn get(&mut self, key: &str) -> Option<&Vec<f64>> {
        if let Some(value) = self.map.get(key) {
            self.order.retain(|x| x != key);
            self.order.push_back(key.to_string());
            Some(value)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: String, value: Vec<f64>) {
        if self.map.len() == self.capacity {
            if let Some(lru) = self.order.pop_front() {
                self.map.remove(&lru);
            }
        }
        self.map.insert(key.clone(), value);
        self.order.push_back(key);
    }
}
