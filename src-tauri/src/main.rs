use std::fs;
use tauri::Manager;

#[tauri::command]
fn read_json(path: String) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_json(path: String, content: String) -> Result<(), String> {
    fs::write(path, content).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_json, write_json])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
