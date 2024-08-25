use crate::optimizer::UserProfile;
use chromadb::v1::ChromaClient;
use chromadb::v1::collection::{ChromaCollection, CollectionEntries, QueryOptions};
use std::error::Error;

pub struct VectorCache {
    collection: ChromaCollection,
}

impl VectorCache {
    pub async fn new(collection_name: &str) -> Result<Self, Box<dyn Error>> {
        let client: ChromaClient = ChromaClient::new(Default::default());
        let collection = client.get_or_create_collection(collection_name, None)?;
        Ok(VectorCache { collection })
    }

    pub async fn add(&self, id: &str, embedding: Vec<f32>, profile: &UserProfile) -> Result<(), Box<dyn Error>> {
        let metadata = serde_json::to_value(profile)?;
        let collection_entries = CollectionEntries {
            ids: vec![id],
            embeddings: Some(vec![embedding]),
            metadatas: Some(vec![metadata.as_object().unwrap().clone()]),
            documents: None,
        };
        self.collection.upsert(collection_entries, None)?;
        Ok(())
    }

    pub async fn search(&self, query_embedding: Vec<f32>, n_results: u32) -> Result<Vec<(f32, UserProfile)>, Box<dyn Error>> {
        let query = QueryOptions {
            query_texts: None,
            query_embeddings: Some(vec![query_embedding]),
            where_metadata: None,
            where_document: None,
            n_results: Some(n_results as usize),
            include: None,
        };
        let query_result = self.collection.query(query, None)?;

        let mut profiles = Vec::new();
        if let Some(distances) = &query_result.distances {
            if let Some(first_distances) = distances.get(0) {
                for (i, distance) in first_distances.iter().enumerate() {
                    if let Some(metadatas) = &query_result.metadatas {
                        if let Some(metadata_vec) = metadatas.get(0) {
                            if let Some(metadata) = metadata_vec.as_ref().and_then(|m| m.get(i)) {
                                let profile: UserProfile = serde_json::from_value(metadata.clone().into())?;
                                let single_distance = distance[0]; // Adjust this as needed
                                profiles.push((single_distance, profile));
                            }
                        }
                    }
                }
            }
        }
        Ok(profiles)
    }
}
