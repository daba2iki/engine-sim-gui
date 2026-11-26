use std::fs;
use std::path::{Path, PathBuf};

fn candidate_paths(path: &str) -> Vec<PathBuf> {
    let raw = PathBuf::from(path);
    let mut candidates = vec![raw.clone(), PathBuf::from("..").join(raw)];

    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            let exe_raw = PathBuf::from(path);
            candidates.push(exe_dir.join(exe_raw.clone()));
            candidates.push(exe_dir.join("..").join(exe_raw));
        }
    }

    candidates
}

fn resolve_existing(path: &str) -> Option<PathBuf> {
    candidate_paths(path).into_iter().find(|candidate| candidate.exists())
}

fn resolve_target(path: &str) -> PathBuf {
    if let Some(existing) = resolve_existing(path) {
        return existing;
    }

    let parent = Path::new(path).parent().unwrap_or(Path::new("."));
    let parent_candidates = vec![parent.to_path_buf(), PathBuf::from("..").join(parent)];
    if let Some(parent_dir) = parent_candidates.into_iter().find(|dir| dir.is_dir()) {
        return parent_dir.join(Path::new(path).file_name().unwrap_or_else(|| "sample.json".as_ref()));
    }

    PathBuf::from(path)
}

#[tauri::command]
fn read_json(path: String) -> Result<String, String> {
    let target = resolve_existing(&path)
        .ok_or_else(|| format!("File not found: {path}"))?;
    fs::read_to_string(target).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_json(path: String, content: String) -> Result<(), String> {
    let target = resolve_target(&path);
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(target, content).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_json_files(dir: String) -> Result<Vec<String>, String> {
    let dir_candidates = vec![PathBuf::from(&dir), PathBuf::from("..").join(&dir)];
    let root = dir_candidates
        .into_iter()
        .find(|candidate| candidate.is_dir())
        .ok_or_else(|| format!("Directory not found: {dir}"))?;

    let entries = fs::read_dir(root).map_err(|e| e.to_string())?;
    let mut files = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() && path.extension().map(|ext| ext == "json").unwrap_or(false) {
            if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                files.push(name.to_string());
            }
        }
    }

    files.sort();
    Ok(files)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_json, write_json, list_json_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
