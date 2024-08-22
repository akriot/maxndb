from sentence_transformers import SentenceTransformer
import json

model = SentenceTransformer('paraphrase-MiniLM-L6-v2')

sentences = [
    "This is an example sentence.",
    "Each sentence is converted",
    "into a vector using BERT."
]

embeddings = model.encode(sentences)

data = [
    {"id": f"sentence_{i}", "vector": emb.tolist()}
    for i, emb in enumerate(embeddings)
]

with open("embeddings.json", "w") as f:
    json.dump(data, f)
