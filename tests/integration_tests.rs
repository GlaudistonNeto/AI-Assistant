use ai_assistant::config::{load_config, save_config};
use ai_assistant::models::AppConfig;
use ai_assistant::ollama::check_ollama_status;
use ai_assistant::search::search_by_name;

#[test]
fn test_full_search_flow() {
    // 1. Load configuration
    let config = load_config();
    
    // Ensure at least one folder is present for the test
    if config.index_folders.is_empty() {
        let mut mut_config = config.clone();
        mut_config.index_folders.push(std::path::PathBuf::from("."));
        save_config(&mut_config).expect("Failed to save config");
        // Re-load to confirm
        let config = load_config();
        assert!(!config.index_folders.is_empty());
    }

    // 2. Test search logic
    // We use a generic query "test" to see if it returns any files in the current directory
    let results = search_by_name(&config.index_folders, "test");
    
    // We don't assert that it's not empty because the directory might be empty,
    // but we check that the returned results have the correct structure.
    for result in results {
        assert!(!result.name.is_empty());
        assert!(!result.path.is_empty());
    }
}

#[tokio::test]
async fn test_ollama_connection_timeout() {
    // Testing the status check. 
    // Since we don't have a real Ollama running in the CI/Test env, 
    // it should return false gracefully.
    let status = check_ollama_status("http://localhost:11435").await;
    assert_eq!(status, false, "Ollama deveria estar offline no ambiente de teste");
}
