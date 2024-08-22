use crate::cache::LRUCache;
use ndarray::Array1;
use serde::{Deserialize, Serialize};

pub struct Optimizer {
    cache: LRUCache,
    embeddings: Vec<Embedding>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Embedding {
    id: String,
    vector: Vec<f64>,
}

impl Optimizer {
    pub fn new() -> Self {
        Optimizer {
            cache: LRUCache::new(100),
            embeddings: Vec::new(),
        }
    }

    pub fn load_embeddings(&mut self) {
        // Load embeddings from a file or external source.
        // Placeholder for loading embeddings
        self.embeddings.push(Embedding {
            id: "sample".to_string(),
            vector: vec![0.1, 0.2, 0.3, 0.4, 0.5],
        });
    }

    pub fn query(&mut self, query_vector: Vec<f64>, top_k: usize) -> Vec<(String, f64)> {
        // Simplified cosine similarity calculation
        let mut results: Vec<(String, f64)> = self
            .embeddings
            .iter()
            .map(|embedding| {
                let similarity = cosine_similarity(&embedding.vector, &query_vector);
                (embedding.id.clone(), similarity)
            })
            .collect();

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        results.truncate(top_k);
        results
    }
}

fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let dot_product: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let norm_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    dot_product / (norm_a * norm_b)
}
