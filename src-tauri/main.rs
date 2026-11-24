// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[tauri::command]
fn read_config_file(path: &str) -> Result<String, String> {
  // サンプル実装
  let config_content = r#"{
    "name": "Engine Simulator",
    "version": "0.1.0",
    "engine": {
      "type": "internal combustion engine"
    }
  }"#;
  
  Ok(config_content.to_string())
}

#[tauri::command]
fn save_config_file(path: &str, content: &str) -> Result<(), String> {
  // サンプル実装
  println!("Saving to file: {}", path);
  println!("Content: {}", content);
  Ok(())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![read_config_file, save_config_file])
    .setup(|app| {
      // ウィンドウの非表示設定
      #[cfg(target_os = "macos")]
      app.set_transparent(true);
      
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}