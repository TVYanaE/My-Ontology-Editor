CREATE TABLE IF NOT EXISTS files (
    file_id TEXT PRIMARY KEY,
    path TEXT NOT NULL,
    parent TEXT NOT NULL,
    FOREIGN KEY (parent) REFERENCES semantic_nodes(node_id) 
); 
