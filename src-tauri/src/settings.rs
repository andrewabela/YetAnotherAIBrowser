use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use tauri::AppHandle;
use serde::Deserialize;
use std::fs;

static STORE: Lazy<tauri_plugin_store::Store> = Lazy::new(|| {
    AppHandle::main().unwrap().store("store.json").unwrap()
});

fn get_config(param: String) -> Result<String> {
    match STORE.get(&param) {
        Some(value) => Ok(value.to_string()),
        None => Err(anyhow!("Parameter not found")),
    }
}

fn set_config(param: String, value: String) -> Result<()> {
    STORE.set(&param, value)?;
    STORE.save()?;
    Ok(())
}

#[derive(Deserialize)]
struct Config {
    llm_providers: Vec<LlmProvider>,
}

#[derive(Deserialize)]
struct LlmProvider {
    id: String,
    name: String,
}

#[tauri::command]
fn get_all_llm_providers() -> Result<Vec<(String, String)>> {
    let config_path = "/home/andrew/Documents/GitHub/YetAnotherAIBrowser/YetAnotherAIBrowser/src-tauri/src/pre_def_config.json";
    
    // Read the file content
    let data = fs::read_to_string(config_path).map_err(|e| anyhow!("Failed to read config file: {}", e))?;
    
    // Parse the JSON data
    let config: Config = serde_json::from_str(&data).map_err(|e| anyhow!("Failed to parse config JSON: {}", e))?;
    
    // Extract and return the list of (id, name) tuples
    Ok(config.llm_providers.into_iter().map(|provider| (provider.id.clone(), provider.name.clone())).collect())
}


// #[tauri::command]
// fn get