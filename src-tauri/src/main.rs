use std::fs;
use std::path::{Path, PathBuf};

/// A directory is treated as the project root when it contains both a
/// `sample/` folder and `package.json`. Walking upward from the current
/// directory and from the executable directory lets relative paths such as
/// `sample/sample.json` resolve no matter where the app is launched from
/// (dev server, `target\release`, or an installed location).
fn looks_like_root(dir: &Path) -> bool {
    dir.join("sample").is_dir() && dir.join("package.json").exists()
}

/// Pure version (testable): walk up from each base directory (up to `depth`
/// levels) and return the first one that looks like a project root.
fn find_project_root_from(bases: &[PathBuf], depth: usize) -> Option<PathBuf> {
    for base in bases {
        let mut current = Some(base.clone());
        for _ in 0..=depth {
            let dir = current?;
            if looks_like_root(&dir) {
                return Some(dir);
            }
            current = dir.parent().map(|p| p.to_path_buf());
        }
    }
    None
}

fn find_project_root() -> Option<PathBuf> {
    let mut bases: Vec<PathBuf> = Vec::new();
    if let Ok(cwd) = std::env::current_dir() {
        bases.push(cwd);
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            bases.push(dir.to_path_buf());
        }
    }
    find_project_root_from(&bases, 6)
}

/// All plausible locations for a relative path, in priority order.
fn candidate_paths(path: &str) -> Vec<PathBuf> {
    let raw = PathBuf::from(path);
    if raw.is_absolute() {
        return vec![raw];
    }

    let mut candidates: Vec<PathBuf> = Vec::new();
    let push_unique = |candidates: &mut Vec<PathBuf>, p: PathBuf| {
        if !candidates.contains(&p) {
            candidates.push(p);
        }
    };

    if let Ok(cwd) = std::env::current_dir() {
        push_unique(&mut candidates, cwd.join(&raw));
    }
    if let Some(root) = find_project_root() {
        push_unique(&mut candidates, root.join(&raw));
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            push_unique(&mut candidates, dir.join(&raw));
        }
    }
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        push_unique(
            &mut candidates,
            PathBuf::from(local).join("engine-sim-gui").join(&raw),
        );
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

    // No existing file yet: write next to an existing `sample/` dir when we
    // can find one, otherwise create it under the current directory.
    if let Some(root) = find_project_root() {
        return root.join(path);
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
    let root = candidate_paths(&dir)
        .into_iter()
        .find(|candidate| candidate.is_dir())
        .ok_or_else(|| format!("Directory not found: {dir}"))?;

    let entries = fs::read_dir(&root).map_err(|e| e.to_string())?;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_base(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "enginesim-gui-test-{}-{}",
            std::process::id(),
            name
        ));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn finds_root_two_levels_up() {
        let base = temp_base("up");
        let root = base.join("proj");
        let deep = root.join("target").join("release");
        fs::create_dir_all(deep.join("sample").parent().unwrap()).unwrap();
        fs::create_dir_all(root.join("sample")).unwrap();
        fs::write(root.join("package.json"), "{}").unwrap();
        fs::write(root.join("sample").join("sample.json"), "{}").unwrap();

        let found = find_project_root_from(&[deep.clone()], 6).expect("should find root");
        assert_eq!(found, root);
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn prefers_first_base_with_root() {
        let base = temp_base("multi");
        let root_a = base.join("a");
        let root_b = base.join("b");
        for root in [&root_a, &root_b] {
            fs::create_dir_all(root.join("sample")).unwrap();
            fs::write(root.join("package.json"), "{}").unwrap();
        }

        let found = find_project_root_from(&[root_a.clone(), root_b.clone()], 2).unwrap();
        assert_eq!(found, root_a);
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn rejects_dir_without_sample() {
        let base = temp_base("no-sample");
        fs::create_dir_all(base.join("x")).unwrap();
        fs::write(base.join("package.json"), "{}").unwrap();
        assert!(find_project_root_from(&[base.clone()], 3).is_none());
        let _ = fs::remove_dir_all(&base);
    }
}
