mod config;
mod models;
mod ollama;
mod search;

use models::{AppConfig, SearchResult};
use config::{load_config, save_config};
use ollama::check_ollama_status;
use search::search_by_name;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Iniciando AI Assistant...");

    // 1. Carregar Configuração
    let mut config = load_config();
    println!("Configuração carregada: {:?}", config);

    // 2. Verificar Conexão com Ollama
    println!("Verificando conexão com Ollama em {}...", config.ollama_url);
    if check_ollama_status(&config.ollama_url).await {
        println!("Conectado ao Ollama com sucesso!");
    } else {
        println!("Aviso: Ollama não está acessível. A busca semântica não estará disponível.");
    }

    // 3. Simular Seleção de Pastas (Baseado na configuração inicial)
    // No futuro, isso será interativo via interface Tauri.
    if config.index_folders.is_empty() {
        println!("Nenhuma pasta selecionada para indexação. Adicionando pastas padrão para teste...");
        config.index_folders.push(std::path::PathBuf::from("."));
        save_config(&config)?;
    }

    // 4. Executar Busca por Nome
    let query = "projeto"; // Exemplo de consulta
    println!("Executando busca por: '{}'...", query);
    
    let results = search_by_name(&config.index_folders, query);

    if results.is_empty() {
        println!("Nenhum resultado encontrado.");
    } else {
        println!("Resultados encontrados:");
        for (i, result) in results.iter().enumerate() {
            println!("{}. {} -> {}", i + 1, result.name, result.path);
        }
    }

    println!("Fluxo inicial concluído.");
    Ok(())
}
