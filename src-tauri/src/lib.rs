use std::fs;
use std::path::Path;
use std::process::Command;

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
    std::fs::write(&path, content)
        .map_err(|error| format!("Failed to write file: {}", error))?;

    Ok(())
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
            write_text_file
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}