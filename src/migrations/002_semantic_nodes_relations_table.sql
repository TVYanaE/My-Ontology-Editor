CREATE TABLE IF NOT EXISTS semantic_nodes_relations (
    relations_id TEXT PRIMARY KEY,
    src_node_id TEXT NOT NULL,
    dst_node_id TEXT NOT NULL,
    FOREIGN KEY (src_node_id) REFERENCES semantic_nodes(node_id),
    FOREIGN KEY (dst_node_id) REFERENCES semantic_nodes(node_id)
);
