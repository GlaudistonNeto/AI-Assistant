use crate::models::SearchResult;
use walkdir::WalkDir;
use std::path::Path;
use std::fs;

pub fn search_by_name(folders: &[std::path::PathBuf], query: &str) -> Vec<SearchResult> {
    let mut results = Vec::new();
    for folder in folders {
        for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_string_lossy();
                if file_name.to_lowercase().contains(&query.to_lowercase()) {
                    results.push(SearchResult {
                        name: file_name.into_owned(),
                        path: path.to_string_lossy().into_owned(),
                        snippet: None,
                        source: "Metadata".to_string(),
                    });
                }
            }
        }
    }
    results
}
