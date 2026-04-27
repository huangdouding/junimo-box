use std::process::Command;

#[tauri::command]
fn launch_game(path: String) -> Result<(), String> {
    Command::new(path)
        .spawn()
        .map_err(|error| format!("Failed to launch game: {}", error))?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![launch_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}