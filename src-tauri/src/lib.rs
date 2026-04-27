use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use zip::ZipArchive;

#[tauri::command]
fn launch_game(path: String) -> Result<(), String> {
    Command::new(path)
        .spawn()
        .map_err(|error| format!("Failed to launch game: {}", error))?;

    Ok(())
}

#[tauri::command]
fn open_folder(path: String) -> Result<(), String> {
    Command::new("explorer")
        .arg(path)
        .spawn()
        .map_err(|error| format!("Failed to open folder: {}", error))?;

    Ok(())
}

#[tauri::command]
fn move_folder(from: String, to: String) -> Result<(), String> {
    let from_path = Path::new(&from);
    let to_path = Path::new(&to);

    if !from_path.exists() {
        return Err(format!("Source folder does not exist: {}", from));
    }

    if to_path.exists() {
        return Err(format!("Target folder already exists: {}", to));
    }

    if let Some(parent) = to_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("Failed to create target parent folder: {}", error))?;
    }

    fs::rename(from_path, to_path)
        .map_err(|error| format!("Failed to move folder: {}", error))?;

    Ok(())
}

#[tauri::command]
fn write_text_file(path: String, content: String) -> Result<(), String> {
    fs::write(&path, content).map_err(|error| format!("Failed to write file: {}", error))?;

    Ok(())
}

fn get_smapi_log_dir() -> Result<PathBuf, String> {
    let appdata = std::env::var("APPDATA")
        .map_err(|_| "Failed to read APPDATA environment variable".to_string())?;

    Ok(PathBuf::from(appdata)
        .join("StardewValley")
        .join("ErrorLogs"))
}

#[tauri::command]
fn get_smapi_log_folder() -> Result<String, String> {
    let log_dir = get_smapi_log_dir()?;

    Ok(log_dir.to_string_lossy().to_string())
}

#[tauri::command]
fn read_latest_smapi_log() -> Result<Vec<String>, String> {
    let log_dir = get_smapi_log_dir()?;

    if !log_dir.exists() {
        return Err(format!(
            "SMAPI log folder does not exist: {}",
            log_dir.to_string_lossy()
        ));
    }

    let mut log_files = Vec::new();

    for entry in fs::read_dir(&log_dir)
        .map_err(|error| format!("Failed to read SMAPI log folder: {}", error))?
    {
        let entry = entry.map_err(|error| format!("Failed to read log entry: {}", error))?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let modified = entry
            .metadata()
            .and_then(|metadata| metadata.modified())
            .map_err(|error| format!("Failed to read log metadata: {}", error))?;

        log_files.push((path, modified));
    }

    let Some((latest_path, _)) = log_files.into_iter().max_by_key(|(_, modified)| *modified) else {
        return Err("No SMAPI log files found.".to_string());
    };

    let file_name = latest_path
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown log file".to_string());

    let content = fs::read_to_string(&latest_path)
        .map_err(|error| format!("Failed to read SMAPI log file: {}", error))?;

    Ok(vec![file_name, content])
}

#[derive(Serialize)]
struct ZipModPreview {
    name: String,
    author: String,
    version: String,
    description: String,
    unique_id: String,
    manifest_path: String,
    suggested_folder: String,
}

#[tauri::command]
fn preview_zip_mods(zip_path: String) -> Result<Vec<ZipModPreview>, String> {
    let file =
        File::open(&zip_path).map_err(|error| format!("Failed to open zip file: {}", error))?;

    let mut archive =
        ZipArchive::new(file).map_err(|error| format!("Failed to read zip archive: {}", error))?;

    let mut previews = Vec::new();

    for index in 0..archive.len() {
        let mut file = archive
            .by_index(index)
            .map_err(|error| format!("Failed to read zip entry: {}", error))?;

        if file.is_dir() {
            continue;
        }

        let entry_name = file.name().replace("/", "\\");

        if entry_name.contains("__MACOSX") {
            continue;
        }

        if !entry_name.to_lowercase().ends_with("manifest.json") {
            continue;
        }

        let mut manifest_text = String::new();

        file.read_to_string(&mut manifest_text)
            .map_err(|error| format!("Failed to read manifest.json: {}", error))?;

        let manifest: serde_json::Value = json5::from_str(&manifest_text).map_err(|error| {
            format!(
                "Failed to parse manifest.json at {}: {}",
                entry_name, error
            )
        })?;

        let name = get_json_string(&manifest, "Name")
            .unwrap_or_else(|| get_folder_from_manifest_path(&entry_name));

        let author = get_json_string(&manifest, "Author").unwrap_or_default();
        let version = get_json_string(&manifest, "Version").unwrap_or_default();
        let description = get_json_string(&manifest, "Description").unwrap_or_default();
        let unique_id = get_json_string(&manifest, "UniqueID").unwrap_or_default();
        let suggested_folder = get_folder_from_manifest_path(&entry_name);

        previews.push(ZipModPreview {
            name,
            author,
            version,
            description,
            unique_id,
            manifest_path: entry_name,
            suggested_folder,
        });
    }

    if previews.is_empty() {
        return Err("No manifest.json found in this zip file.".to_string());
    }

    Ok(previews)
}

fn get_json_string(value: &serde_json::Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(|item| item.as_str())
        .map(|item| item.to_string())
}

fn get_folder_from_manifest_path(manifest_path: &str) -> String {
    let parts: Vec<&str> = manifest_path
        .split('\\')
        .filter(|part| !part.is_empty())
        .collect();

    if parts.len() >= 2 {
        return parts[parts.len() - 2].to_string();
    }

    "UnknownMod".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            launch_game,
            open_folder,
            move_folder,
            write_text_file,
            get_smapi_log_folder,
            read_latest_smapi_log,
            preview_zip_mods
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}