use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub index_folders: Vec<PathBuf>,
    pub max_file_size_mb: u64,
    pub exclusions: Vec<String>,
    pub ollama_url: String,
    pub embedding_model: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            index_folders: Vec::new(),
            max_file_size_mb: 50,
            exclusions: vec![".git".to_string(), "node_modules".to_string()],
            ollama_url: "http://localhost:11434".to_string(),
            embedding_model: "llama3".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub name: String,
    pub path: String,
    pub snippet: Option<String>,
    pub source: String,
}

