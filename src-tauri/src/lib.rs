// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("<div>Hello,<h1>{}</h1></div>", name)
}

/// This command handles all URL links instead of opening them in the browser.
/// It receives the URL as a parameter and you can implement custom logic here.
#[tauri::command]
async fn handle_url_click(url: String) -> Result<String, String> {
    // Log the intercepted URL
    println!("Intercepted URL: {}", url);
    // TODO:
    Ok(format!("Successfully handled URL: {}", url))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, handle_url_click])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
