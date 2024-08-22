mod cache;
mod optimizer;

use optimizer::Optimizer;

fn main() {
    // Example usage
    let mut optimizer = Optimizer::new();
    optimizer.load_embeddings();
    let query = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    let result = optimizer.query(query, 5);
    println!("Best match: {:?}", result);
}
