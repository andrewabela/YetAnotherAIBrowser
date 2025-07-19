use serde::Deserialize;
use std::fs;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

#[derive(Deserialize)]
struct Config {
    llm_providers: Vec<LlmProvider>,
}

#[derive(Deserialize)]
struct LlmProvider {
    id: String,
    name: String,
    default_endpoint: String,
    default_api_key: String,
    default_model: String,
}

#[tauri::command]
pub fn get_all_llm_providers() -> Result<Vec<(String, String)>, String> {
    println!("get_all_llm_providers called");
    let config_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/pre_def_config.json");
    println!("Config path: {}", config_path);
    
    // Read the file content
    let data = fs::read_to_string(config_path)
        .map_err(|e| {
            let error_msg = format!("Failed to read config file: {}", e);
            println!("{}", error_msg);
            error_msg
        })?;
    
    println!("Config file read successfully");
    
    // Parse the JSON data
    let config: Config = serde_json::from_str(&data)
        .map_err(|e| {
            let error_msg = format!("Failed to parse config JSON: {}", e);
            println!("{}", error_msg);
            error_msg
        })?;
    
    println!("Config parsed successfully, found {} providers", config.llm_providers.len());
    
    // Extract and return the list of (id, name) tuples
    let result: Vec<(String, String)> = config.llm_providers.into_iter()
        .map(|provider| {
            println!("Provider: {} - {}", provider.id, provider.name);
            (provider.id, provider.name)
        })
        .collect();
    
    println!("Returning {} providers", result.len());
    Ok(result)
}

#[tauri::command]
pub fn get_provider_defaults(provider_id: String) -> Result<(String, String, String), String> {
    println!("get_provider_defaults called for: {}", provider_id);
    let config_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/pre_def_config.json");
    
    // Read the file content
    let data = fs::read_to_string(config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;
    
    // Parse the JSON data
    let config: Config = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse config JSON: {}", e))?;
    
    // Find the provider by ID
    for provider in config.llm_providers {
        if provider.id == provider_id {
            println!("Found provider defaults: endpoint={}, key={}, model={}", 
                provider.default_endpoint, 
                if provider.default_api_key.is_empty() { "[empty]" } else { "[set]" }, 
                provider.default_model);
            return Ok((provider.default_endpoint, provider.default_api_key, provider.default_model));
        }
    }
    
    Err(format!("Provider with id '{}' not found", provider_id))
}

#[tauri::command]
pub fn get_current_llm_provider(app: AppHandle) -> Result<String, String> {
    let store = app.store("settings.json").map_err(|e| format!("Failed to access store: {}", e))?;
    
    match store.get("current_llm_provider") {
        Some(value) => Ok(value.as_str().unwrap_or("").to_string()),
        None => Ok(String::new()), // Return empty string if not set
    }
}

#[tauri::command]
pub fn set_current_llm_provider(app: AppHandle, provider: String) -> Result<(), String> {
    let store = app.store("settings.json").map_err(|e| format!("Failed to access store: {}", e))?;
    
    store.set("current_llm_provider", serde_json::Value::String(provider));
    store.save().map_err(|e| format!("Failed to save store: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn get_llm_endpoint(app: AppHandle) -> Result<String, String> {
    let store = app.store("settings.json").map_err(|e| format!("Failed to access store: {}", e))?;
    
    match store.get("llm_endpoint") {
        Some(value) => Ok(value.as_str().unwrap_or("").to_string()),
        None => Ok(String::new()), // Return empty string if not set
    }
}

#[tauri::command]
pub fn set_llm_endpoint(app: AppHandle, endpoint: String) -> Result<(), String> {
    let store = app.store("settings.json").map_err(|e| format!("Failed to access store: {}", e))?;
    
    store.set("llm_endpoint", serde_json::Value::String(endpoint));
    store.save().map_err(|e| format!("Failed to save store: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn get_llm_key(app: AppHandle) -> Result<String, String> {
    let store = app.store("settings.json").map_err(|e| format!("Failed to access store: {}", e))?;
    
    match store.get("llm_key") {
        Some(value) => Ok(value.as_str().unwrap_or("").to_string()),
        None => Ok(String::new()), // Return empty string if not set
    }
}

#[tauri::command]
pub fn set_llm_key(app: AppHandle, key: String) -> Result<(), String> {
    let store = app.store("settings.json").map_err(|e| format!("Failed to access store: {}", e))?;
    
    store.set("llm_key", serde_json::Value::String(key));
    store.save().map_err(|e| format!("Failed to save store: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn get_llm_model(app: AppHandle) -> Result<String, String> {
    let store = app.store("settings.json").map_err(|e| format!("Failed to access store: {}", e))?;
    
    match store.get("llm_model") {
        Some(value) => Ok(value.as_str().unwrap_or("").to_string()),
        None => Ok(String::new()), // Return empty string if not set
    }
}

#[tauri::command]
pub fn set_llm_model(app: AppHandle, model: String) -> Result<(), String> {
    let store = app.store("settings.json").map_err(|e| format!("Failed to access store: {}", e))?;
    
    store.set("llm_model", serde_json::Value::String(model));
    store.save().map_err(|e| format!("Failed to save store: {}", e))?;
    Ok(())
}


// #[tauri::command]
// fn get