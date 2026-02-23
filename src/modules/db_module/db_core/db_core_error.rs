use thiserror::{
    Error,
};

#[derive(Debug, Error)]
pub enum DBCoreError {
    #[error("RuSQlite Error: {0}")]
    RuSQlitError(#[from] rusqlite::Error),
} 
