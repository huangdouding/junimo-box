use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter};
use zip::ZipArchive;

const JUNIMO_BOX_SOURCE_CONFIG_URL: &str =
    "https://cdn.jsdelivr.net/gh/huangdouding/junimo-box@main/public/sources.json";

const FALLBACK_SMAPI_VERSION: &str = "4.5.2";
const FALLBACK_SMAPI_DOWNLOAD_URL: &str =
    "https://github.com/Pathoschild/SMAPI/releases/download/4.5.2/SMAPI-4.5.2-installer.zip";

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

    fs::rename(from_path, to_path).map_err(|error| format!("Failed to move folder: {}", error))?;

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

#[derive(Serialize, Clone)]
struct ZipModDependency {
    unique_id: String,
    is_required: bool,
    is_installed: bool,
}

#[derive(Serialize, Clone)]
struct ZipModPreview {
    name: String,
    author: String,
    version: String,
    description: String,
    unique_id: String,
    manifest_path: String,
    suggested_folder: String,
    entry_dll: String,
    mod_type: String,
    dependencies: Vec<ZipModDependency>,
    content_pack_for: Option<ZipModDependency>,
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

        let entry_name = normalize_zip_path(file.name());

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
        let entry_dll = get_json_string(&manifest, "EntryDll").unwrap_or_default();
        let suggested_folder = get_folder_from_manifest_path(&entry_name);

        let dependencies = normalize_zip_dependencies(manifest.get("Dependencies"));
        let content_pack_for = normalize_zip_content_pack_for(manifest.get("ContentPackFor"));

        let mod_type = detect_mod_type(&unique_id, &entry_dll, &content_pack_for);

        previews.push(ZipModPreview {
            name,
            author,
            version,
            description,
            unique_id,
            manifest_path: entry_name,
            suggested_folder,
            entry_dll,
            mod_type,
            dependencies,
            content_pack_for,
        });
    }

    if previews.is_empty() {
        return Err("No manifest.json found in this zip file.".to_string());
    }

    Ok(previews)
}

#[tauri::command]
fn install_zip_mods(
    zip_path: String,
    game_path: String,
    conflict_mode: Option<String>,
) -> Result<Vec<ZipModPreview>, String> {
    let previews = preview_zip_mods(zip_path.clone())?;
    let conflict_mode = normalize_install_conflict_mode(conflict_mode);

    let root_manifest_count = previews
        .iter()
        .filter(|preview| get_root_prefix_from_manifest_path(&preview.manifest_path).is_empty())
        .count();

    if root_manifest_count > 0 && previews.len() > 1 {
        return Err(
            "This zip contains a root manifest.json and multiple mods. Junimo Box cannot safely install it yet."
                .to_string(),
        );
    }

    let game_dir = Path::new(&game_path);
    let mods_dir = game_dir.join("Mods");

    if !mods_dir.exists() {
        fs::create_dir_all(&mods_dir)
            .map_err(|error| format!("Failed to create Mods folder: {}", error))?;
    }

    validate_zip_install_targets(&mods_dir, &previews, &conflict_mode)?;

    let temp_root = create_install_temp_dir(game_dir)?;

    let install_result = extract_zip_to_temp_and_move(&zip_path, &mods_dir, &temp_root, &previews, &conflict_mode);

    if let Err(error) = fs::remove_dir_all(&temp_root) {
        eprintln!(
            "Failed to clean temporary install folder {}: {}",
            temp_root.to_string_lossy(),
            error
        );
    }

    install_result?;

    Ok(previews)
}

fn validate_zip_install_targets(
    mods_dir: &Path,
    previews: &[ZipModPreview],
    conflict_mode: &str,
) -> Result<(), String> {
    let mut seen_folders = std::collections::HashSet::new();

    for preview in previews {
        if !seen_folders.insert(preview.suggested_folder.clone()) {
            return Err(format!(
                "Duplicate target folder in zip: {}",
                preview.suggested_folder
            ));
        }

        let target_folder = safe_join(mods_dir, &preview.suggested_folder)?;

        if target_folder.exists() && conflict_mode == "cancel" {
            return Err(format!(
                "Target Mod folder already exists: {}",
                target_folder.to_string_lossy()
            ));
        }
    }

    Ok(())
}

fn normalize_install_conflict_mode(conflict_mode: Option<String>) -> String {
    match conflict_mode
        .unwrap_or_else(|| "cancel".to_string())
        .trim()
        .to_lowercase()
        .as_str()
    {
        "skip" => "skip".to_string(),
        "replace" | "update" => "replace".to_string(),
        _ => "cancel".to_string(),
    }
}

fn create_install_temp_dir(game_dir: &Path) -> Result<PathBuf, String> {
    let timestamp = current_timestamp()?;

    let temp_dir = game_dir
        .join("Junimo Box Temp")
        .join(format!("zip-install-{}", timestamp));

    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)
            .map_err(|error| format!("Failed to clear temp folder: {}", error))?;
    }

    fs::create_dir_all(&temp_dir)
        .map_err(|error| format!("Failed to create temp folder: {}", error))?;

    Ok(temp_dir)
}

fn extract_zip_to_temp_and_move(
    zip_path: &str,
    mods_dir: &Path,
    temp_root: &Path,
    previews: &[ZipModPreview],
    conflict_mode: &str,
) -> Result<(), String> {
    let file =
        File::open(zip_path).map_err(|error| format!("Failed to open zip file: {}", error))?;

    let mut archive =
        ZipArchive::new(file).map_err(|error| format!("Failed to read zip archive: {}", error))?;

    for index in 0..archive.len() {
        let mut zip_file = archive
            .by_index(index)
            .map_err(|error| format!("Failed to read zip entry: {}", error))?;

        if zip_file.is_dir() {
            continue;
        }

        let entry_name = normalize_zip_path(zip_file.name());

        if entry_name.contains("__MACOSX") {
            continue;
        }

        let Some((preview, relative_path)) = find_matching_preview(&entry_name, previews) else {
            continue;
        };

        if relative_path.is_empty() {
            continue;
        }

        let temp_mod_folder = safe_join(temp_root, &preview.suggested_folder)?;
        let temp_file_path = safe_join(&temp_mod_folder, &relative_path)?;

        if let Some(parent) = temp_file_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("Failed to create temp target folder: {}", error))?;
        }

        let mut output_file = File::create(&temp_file_path)
            .map_err(|error| format!("Failed to create temp target file: {}", error))?;

        std::io::copy(&mut zip_file, &mut output_file)
            .map_err(|error| format!("Failed to extract zip file: {}", error))?;
    }

    move_extracted_mods_with_conflict_mode(mods_dir, temp_root, previews, conflict_mode)
}

fn move_extracted_mods_with_conflict_mode(
    mods_dir: &Path,
    temp_root: &Path,
    previews: &[ZipModPreview],
    conflict_mode: &str,
) -> Result<(), String> {
    let backup_root = temp_root.join("__junimo_replace_backup");
    let mut backed_up_folders: Vec<(PathBuf, PathBuf)> = Vec::new();
    let mut moved_new_folders: Vec<PathBuf> = Vec::new();

    let move_result = (|| -> Result<(), String> {
        for preview in previews {
            let temp_mod_folder = safe_join(temp_root, &preview.suggested_folder)?;
            let final_mod_folder = safe_join(mods_dir, &preview.suggested_folder)?;

            if !temp_mod_folder.exists() {
                return Err(format!(
                    "Extracted mod folder was not found in temp folder: {}",
                    temp_mod_folder.to_string_lossy()
                ));
            }

            if final_mod_folder.exists() {
                match conflict_mode {
                    "skip" => {
                        continue;
                    }
                    "replace" => {
                        let backup_folder = safe_join(&backup_root, &preview.suggested_folder)?;

                        if let Some(parent) = backup_folder.parent() {
                            fs::create_dir_all(parent).map_err(|error| {
                                format!("Failed to create replace backup folder: {}", error)
                            })?;
                        }

                        fs::rename(&final_mod_folder, &backup_folder).map_err(|error| {
                            format!(
                                "Failed to backup existing Mod folder before replace: {}",
                                error
                            )
                        })?;

                        backed_up_folders.push((final_mod_folder.clone(), backup_folder));
                    }
                    _ => {
                        return Err(format!(
                            "Target Mod folder already exists: {}",
                            final_mod_folder.to_string_lossy()
                        ));
                    }
                }
            }

            fs::rename(&temp_mod_folder, &final_mod_folder).map_err(|error| {
                format!(
                    "Failed to move installed mod into Mods folder: {}",
                    error
                )
            })?;

            moved_new_folders.push(final_mod_folder);
        }

        Ok(())
    })();

    if let Err(error) = move_result {
        for folder in moved_new_folders.iter().rev() {
            if folder.exists() {
                let _ = fs::remove_dir_all(folder);
            }
        }

        for (original_folder, backup_folder) in backed_up_folders.iter().rev() {
            if backup_folder.exists() && !original_folder.exists() {
                if let Some(parent) = original_folder.parent() {
                    let _ = fs::create_dir_all(parent);
                }

                let _ = fs::rename(backup_folder, original_folder);
            }
        }

        return Err(format!(
            "Install failed and Junimo Box attempted to restore existing Mods: {}",
            error
        ));
    }

    if backup_root.exists() {
        fs::remove_dir_all(&backup_root).map_err(|error| {
            format!("Failed to clean replace backup folder: {}", error)
        })?;
    }

    Ok(())
}

#[derive(Deserialize)]
struct RemoteSourceConfig {
    smapi: RemoteSmapiSource,
}

#[derive(Deserialize, Clone)]
struct RemoteSmapiSource {
    version: String,
    #[serde(rename = "downloadUrl")]
    download_url: String,
    #[serde(rename = "fallbackUrls", default)]
    fallback_urls: Vec<String>,
}

#[derive(Clone)]
struct SmapiDownloadSource {
    version: String,
    download_url: String,
    fallback_urls: Vec<String>,
}

#[derive(Serialize)]
struct SmapiInstallResult {
    version: String,
    download_url: String,
    zip_path: String,
    installer_path: String,
}

#[derive(Serialize)]
struct UrlZipDownloadResult {
    download_url: String,
    zip_path: String,
    file_name: String,
    file_size: u64,
    download_id: String,
}

#[derive(Serialize, Clone)]
struct SmapiInstallStagePayload {
    stage: String,
    message: String,
    version: Option<String>,
    downloaded_bytes: Option<u64>,
}

#[derive(Serialize, Clone)]
struct DownloadProgressPayload {
    download_id: String,
    file_name: String,
    stage: String,
    downloaded_bytes: u64,
    total_bytes: u64,
    speed_bytes_per_sec: u64,
    message: String,
    zip_path: Option<String>,
}

type CancelledFlag = Arc<AtomicBool>;
type CancellationMap = Arc<Mutex<HashMap<String, CancelledFlag>>>;

fn cancellation_map() -> &'static CancellationMap {
    static MAP: std::sync::OnceLock<CancellationMap> = std::sync::OnceLock::new();
    MAP.get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
}

fn emit_download_progress(app: &AppHandle, payload: &DownloadProgressPayload) {
    let _ = app.emit("download-progress", payload);
}

#[tauri::command]
async fn download_zip_from_url(
    app: AppHandle,
    url: String,
    game_path: String,
    download_id: String,
) -> Result<UrlZipDownloadResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        download_zip_from_url_blocking(app, url, game_path, download_id)
    })
    .await
    .map_err(|error| format!("ZIP download task failed: {}", error))?
}

fn download_zip_from_url_blocking(
    app: AppHandle,
    url: String,
    game_path: String,
    download_id: String,
) -> Result<UrlZipDownloadResult, String> {
    let trimmed_url = url.trim().to_string();

    if !is_http_url(&trimmed_url) {
        return Err("请输入有效的 http 或 https ZIP 下载链接。".to_string());
    }

    let game_dir = Path::new(&game_path);

    if !game_dir.exists() {
        return Err(format!("Game folder does not exist: {}", game_path));
    }

    let client = Client::builder()
        .user_agent("Junimo Box")
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(300))
        .build()
        .map_err(|error| format!("Failed to create HTTP client: {}", error))?;

    let downloads_dir = game_dir.join("Junimo Box Downloads").join("Mods");
    fs::create_dir_all(&downloads_dir)
        .map_err(|error| format!("Failed to create Mod download folder: {}", error))?;

    let file_name = infer_zip_file_name_from_url(&trimmed_url);
    let target_path = unique_download_path(&downloads_dir, &file_name);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    {
        let mut map = cancellation_map()
            .lock()
            .map_err(|e| e.to_string())?;
        map.insert(download_id.clone(), Arc::clone(&cancel_flag));
    }

    let result = download_generic_zip_file_with_progress(
        &client,
        &trimmed_url,
        &target_path,
        &app,
        &download_id,
        &file_name,
        &cancel_flag,
        0,
    );

    if cancel_flag.load(Ordering::Relaxed) {
        let _ = fs::remove_file(&target_path);
        {
            let mut map = cancellation_map()
                .lock()
                .map_err(|e| e.to_string())?;
            map.remove(&download_id);
        }
        return Err("下载已取消".to_string());
    }

    result?;

    let metadata = fs::metadata(&target_path)
        .map_err(|error| format!("Failed to read downloaded ZIP metadata: {}", error))?;
    let file_size = metadata.len();

    if file_size == 0 {
        let _ = fs::remove_file(&target_path);
        return Err("下载到的 ZIP 文件为空，请检查下载链接或网络连接。".to_string());
    }

    if file_size < 1024 {
        let _ = fs::remove_file(&target_path);
        return Err(format!(
            "下载到的 ZIP 文件过小：{}，下载可能失败。",
            format_bytes(file_size)
        ));
    }

    {
        let mut map = cancellation_map()
            .lock()
            .map_err(|e| e.to_string())?;
        map.remove(&download_id);
    }

    Ok(UrlZipDownloadResult {
        download_url: trimmed_url,
        zip_path: target_path.to_string_lossy().to_string(),
        file_name: target_path
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| file_name),
        file_size,
        download_id,
    })
}

#[tauri::command]
async fn install_latest_smapi(app: AppHandle, game_path: String) -> Result<SmapiInstallResult, String> {
    tauri::async_runtime::spawn_blocking(move || install_latest_smapi_blocking(app, game_path))
        .await
        .map_err(|error| format!("SMAPI install task failed: {}", error))?
}

fn install_latest_smapi_blocking(app: AppHandle, game_path: String) -> Result<SmapiInstallResult, String> {
    let game_dir = Path::new(&game_path);

    if !game_dir.exists() {
        return Err(format!("Game folder does not exist: {}", game_path));
    }

    emit_smapi_stage(&app, "source", "正在读取 SMAPI 下载源...", None, None);

    let client = Client::builder()
        .user_agent("Junimo Box")
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(300))
        .build()
        .map_err(|error| format!("Failed to create HTTP client: {}", error))?;

    let source = get_smapi_download_source(&client);
    emit_smapi_stage(
        &app,
        "download",
        &format!("正在下载 SMAPI {}...", source.version),
        Some(&source.version),
        Some(0),
    );

    let downloads_dir = game_dir.join("Junimo Box Downloads").join("SMAPI");
    fs::create_dir_all(&downloads_dir)
        .map_err(|error| format!("Failed to create SMAPI download folder: {}", error))?;

    let zip_file_name = format!(
        "SMAPI-{}-installer.zip",
        sanitize_file_name(&source.version)
    );
    let zip_path = downloads_dir.join(zip_file_name);

    let downloaded_url = download_smapi_zip(&app, &client, &source, &zip_path)?;

    emit_smapi_stage(
        &app,
        "extract",
        "正在解压 SMAPI 安装包...",
        Some(&source.version),
        None,
    );

    let temp_root = game_dir
        .join("Junimo Box Temp")
        .join(format!("smapi-install-{}", current_timestamp()?));

    if temp_root.exists() {
        fs::remove_dir_all(&temp_root)
            .map_err(|error| format!("Failed to clear SMAPI temp folder: {}", error))?;
    }

    fs::create_dir_all(&temp_root)
        .map_err(|error| format!("Failed to create SMAPI temp folder: {}", error))?;

    extract_zip_archive(&zip_path, &temp_root)?;

    let installer_path = find_file_by_name(&temp_root, "install on Windows.bat")
        .ok_or_else(|| "Failed to find install on Windows.bat in SMAPI installer.".to_string())?;

    emit_smapi_stage(
        &app,
        "open",
        "正在打开 SMAPI 官方 Windows 安装器...",
        Some(&source.version),
        None,
    );

    let installer_folder = installer_path
        .parent()
        .ok_or_else(|| "Failed to resolve SMAPI installer folder.".to_string())?;

    Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg("")
        .arg(installer_path.to_string_lossy().to_string())
        .current_dir(installer_folder)
        .spawn()
        .map_err(|error| format!("Failed to launch SMAPI installer: {}", error))?;

    emit_smapi_stage(
        &app,
        "done",
        "SMAPI 官方安装器已打开，请按照安装器提示完成安装。",
        Some(&source.version),
        None,
    );

    Ok(SmapiInstallResult {
        version: source.version,
        download_url: downloaded_url,
        zip_path: zip_path.to_string_lossy().to_string(),
        installer_path: installer_path.to_string_lossy().to_string(),
    })
}

fn emit_smapi_stage(
    app: &AppHandle,
    stage: &str,
    message: &str,
    version: Option<&str>,
    downloaded_bytes: Option<u64>,
) {
    let _ = app.emit(
        "smapi-install-stage",
        SmapiInstallStagePayload {
            stage: stage.to_string(),
            message: message.to_string(),
            version: version.map(|value| value.to_string()),
            downloaded_bytes,
        },
    );
}

fn get_smapi_download_source(client: &Client) -> SmapiDownloadSource {
    let remote_result = client
        .get(JUNIMO_BOX_SOURCE_CONFIG_URL)
        .header(USER_AGENT, "Junimo Box")
        .header(ACCEPT, "application/json")
        .send()
        .and_then(|response| response.error_for_status())
        .and_then(|response| response.json::<RemoteSourceConfig>());

    match remote_result {
        Ok(config) => SmapiDownloadSource {
            version: config.smapi.version,
            download_url: config.smapi.download_url,
            fallback_urls: config.smapi.fallback_urls,
        },
        Err(error) => {
            eprintln!(
                "Failed to read Junimo Box source config, using fallback SMAPI source: {}",
                error
            );

            SmapiDownloadSource {
                version: FALLBACK_SMAPI_VERSION.to_string(),
                download_url: FALLBACK_SMAPI_DOWNLOAD_URL.to_string(),
                fallback_urls: Vec::new(),
            }
        }
    }
}

fn download_smapi_zip(
    app: &AppHandle,
    client: &Client,
    source: &SmapiDownloadSource,
    zip_path: &Path,
) -> Result<String, String> {
    let mut urls = Vec::new();
    urls.push(source.download_url.clone());
    urls.extend(source.fallback_urls.clone());

    let mut errors = Vec::new();

    for url in urls {
        match download_file(app, client, &url, zip_path, &source.version) {
            Ok(()) => return Ok(url),
            Err(error) => {
                let _ = fs::remove_file(zip_path);
                errors.push(format!("{} -> {}", url, error));
            }
        }
    }

    Err(format!(
        "Failed to download SMAPI installer from all sources: {}",
        errors.join(" | ")
    ))
}

fn download_file(
    app: &AppHandle,
    client: &Client,
    url: &str,
    target_path: &Path,
    version: &str,
) -> Result<(), String> {
    match download_file_parallel(app, client, url, target_path, version) {
        Ok(()) => Ok(()),
        Err(parallel_error) => {
            eprintln!(
                "Parallel SMAPI download failed, falling back to single connection: {}",
                parallel_error
            );

            emit_smapi_stage(
                app,
                "download",
                &format!(
                    "多线程下载不可用，正在使用普通方式下载 SMAPI {}...",
                    version
                ),
                Some(version),
                Some(0),
            );

            download_file_single(app, client, url, target_path, version).map_err(|single_error| {
                format!(
                    "多线程下载失败：{}；普通下载也失败：{}",
                    parallel_error, single_error
                )
            })
        }
    }
}

fn download_file_parallel(
    app: &AppHandle,
    client: &Client,
    url: &str,
    target_path: &Path,
    version: &str,
) -> Result<(), String> {
    const THREAD_COUNT: u64 = 8;
    const MIN_PARALLEL_SIZE: u64 = 2 * 1024 * 1024;
    const REPORT_STEP: u64 = 1024 * 1024;

    let file_size = get_remote_file_size(client, url)?;

    if file_size < MIN_PARALLEL_SIZE {
        return Err(format!(
            "file is too small for parallel download: {}",
            format_bytes(file_size)
        ));
    }

    let temp_path = target_path.with_extension("zip.download");
    clear_download_artifacts(target_path, THREAD_COUNT)?;

    emit_smapi_stage(
        app,
        "download",
        &format!(
            "正在多线程下载 SMAPI {}... 0 / {}",
            version,
            format_bytes(file_size)
        ),
        Some(version),
        Some(0),
    );

    let downloaded_total = Arc::new(AtomicU64::new(0));
    let next_report = Arc::new(AtomicU64::new(REPORT_STEP));
    let mut handles = Vec::new();
    let chunk_size = (file_size + THREAD_COUNT - 1) / THREAD_COUNT;

    for index in 0..THREAD_COUNT {
        let start = index * chunk_size;

        if start >= file_size {
            continue;
        }

        let end = ((index + 1) * chunk_size - 1).min(file_size - 1);
        let expected_size = end - start + 1;
        let part_path = part_path_for(target_path, index);
        let url = url.to_string();
        let version = version.to_string();
        let client = client.clone();
        let app = app.clone();
        let downloaded_total = Arc::clone(&downloaded_total);
        let next_report = Arc::clone(&next_report);

        let handle = thread::spawn(move || -> Result<(), String> {
            let range_header = format!("bytes={}-{}", start, end);

            let mut response = client
                .get(&url)
                .header(USER_AGENT, "Junimo Box")
                .header("Range", range_header)
                .send()
                .map_err(|error| format!("part {} request failed: {}", index, error))?;

            if response.status().as_u16() != 206 {
                return Err(format!(
                    "server did not return partial content for part {}: {}",
                    index,
                    response.status()
                ));
            }

            let mut output_file = File::create(&part_path)
                .map_err(|error| format!("failed to create part {} file: {}", index, error))?;

            let mut part_size = 0_u64;
            let mut buffer = [0_u8; 128 * 1024];

            loop {
                let bytes_read = response
                    .read(&mut buffer)
                    .map_err(|error| format!("failed to read part {}: {}", index, error))?;

                if bytes_read == 0 {
                    break;
                }

                output_file
                    .write_all(&buffer[..bytes_read])
                    .map_err(|error| format!("failed to write part {}: {}", index, error))?;

                part_size += bytes_read as u64;
                let total = downloaded_total.fetch_add(bytes_read as u64, Ordering::Relaxed)
                    + bytes_read as u64;

                let mut report_at = next_report.load(Ordering::Relaxed);
                while total >= report_at {
                    match next_report.compare_exchange(
                        report_at,
                        report_at + REPORT_STEP,
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                    ) {
                        Ok(_) => {
                            emit_smapi_stage(
                                &app,
                                "download",
                                &format!(
                                    "正在多线程下载 SMAPI {}... {} / {}",
                                    version,
                                    format_bytes(total),
                                    format_bytes(file_size)
                                ),
                                Some(&version),
                                Some(total),
                            );
                            break;
                        }
                        Err(current) => report_at = current,
                    }
                }
            }

            output_file
                .flush()
                .map_err(|error| format!("failed to flush part {}: {}", index, error))?;

            if part_size != expected_size {
                return Err(format!(
                    "part {} size mismatch: expected {}, got {}",
                    index,
                    format_bytes(expected_size),
                    format_bytes(part_size)
                ));
            }

            Ok(())
        });

        handles.push(handle);
    }

    for handle in handles {
        handle
            .join()
            .map_err(|_| "parallel download thread panicked".to_string())??;
    }

    merge_download_parts(target_path, &temp_path, THREAD_COUNT, file_size)?;

    if target_path.exists() {
        fs::remove_file(target_path)
            .map_err(|error| format!("Failed to replace old download file: {}", error))?;
    }

    fs::rename(&temp_path, target_path)
        .map_err(|error| format!("Failed to finalize downloaded file: {}", error))?;

    emit_smapi_stage(
        app,
        "downloaded",
        &format!("SMAPI {} 下载完成：{}", version, format_bytes(file_size)),
        Some(version),
        Some(file_size),
    );

    Ok(())
}

fn download_file_single(
    app: &AppHandle,
    client: &Client,
    url: &str,
    target_path: &Path,
    version: &str,
) -> Result<(), String> {
    let temp_path = target_path.with_extension("zip.download");

    if temp_path.exists() {
        fs::remove_file(&temp_path)
            .map_err(|error| format!("Failed to clear old temp download file: {}", error))?;
    }

    let mut response = client
        .get(url)
        .header(USER_AGENT, "Junimo Box")
        .send()
        .map_err(|error| format!("Request failed. Please check your network connection: {}", error))?
        .error_for_status()
        .map_err(|error| format!("Download failed: {}", error))?;

    let mut output_file = File::create(&temp_path)
        .map_err(|error| format!("Failed to create temp download file: {}", error))?;

    let mut downloaded_size = 0_u64;
    let mut next_report_size = 0_u64;
    let mut buffer = [0_u8; 64 * 1024];

    loop {
        let bytes_read = response
            .read(&mut buffer)
            .map_err(|error| format!("Failed to read downloaded data: {}", error))?;

        if bytes_read == 0 {
            break;
        }

        output_file
            .write_all(&buffer[..bytes_read])
            .map_err(|error| format!("Failed to save downloaded data: {}", error))?;

        downloaded_size += bytes_read as u64;

        if downloaded_size >= next_report_size {
            emit_smapi_stage(
                app,
                "download",
                &format!(
                    "正在下载 SMAPI {}... 已下载 {}",
                    version,
                    format_bytes(downloaded_size)
                ),
                Some(version),
                Some(downloaded_size),
            );

            next_report_size = downloaded_size + 1024 * 1024;
        }
    }

    output_file
        .flush()
        .map_err(|error| format!("Failed to flush downloaded file: {}", error))?;

    if downloaded_size == 0 {
        let _ = fs::remove_file(&temp_path);
        return Err(
            "Downloaded SMAPI installer is empty. Please check your network connection to GitHub release assets."
                .to_string(),
        );
    }

    if downloaded_size < 1024 {
        let _ = fs::remove_file(&temp_path);
        return Err(format!(
            "Downloaded SMAPI installer is too small: {}. The download may have failed.",
            format_bytes(downloaded_size)
        ));
    }

    if target_path.exists() {
        fs::remove_file(target_path)
            .map_err(|error| format!("Failed to replace old download file: {}", error))?;
    }

    fs::rename(&temp_path, target_path)
        .map_err(|error| format!("Failed to finalize downloaded file: {}", error))?;

    emit_smapi_stage(
        app,
        "downloaded",
        &format!("SMAPI {} 下载完成：{}", version, format_bytes(downloaded_size)),
        Some(version),
        Some(downloaded_size),
    );

    Ok(())
}

fn get_remote_file_size(client: &Client, url: &str) -> Result<u64, String> {
    let response = client
        .head(url)
        .header(USER_AGENT, "Junimo Box")
        .send()
        .map_err(|error| format!("failed to request file metadata: {}", error))?
        .error_for_status()
        .map_err(|error| format!("failed to request file metadata: {}", error))?;

    let content_length = response
        .headers()
        .get("content-length")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
        .ok_or_else(|| "server did not provide content length".to_string())?;

    if content_length == 0 {
        return Err("remote file size is 0".to_string());
    }

    Ok(content_length)
}

fn part_path_for(target_path: &Path, index: u64) -> PathBuf {
    target_path.with_extension(format!("zip.part{}", index))
}

fn clear_download_artifacts(target_path: &Path, thread_count: u64) -> Result<(), String> {
    let temp_path = target_path.with_extension("zip.download");

    if temp_path.exists() {
        fs::remove_file(&temp_path)
            .map_err(|error| format!("Failed to clear old temp download file: {}", error))?;
    }

    for index in 0..thread_count {
        let part_path = part_path_for(target_path, index);

        if part_path.exists() {
            fs::remove_file(&part_path)
                .map_err(|error| format!("Failed to clear old part file: {}", error))?;
        }
    }

    Ok(())
}

fn merge_download_parts(
    target_path: &Path,
    temp_path: &Path,
    thread_count: u64,
    expected_size: u64,
) -> Result<(), String> {
    let mut output_file = File::create(temp_path)
        .map_err(|error| format!("Failed to create merged download file: {}", error))?;

    let mut merged_size = 0_u64;

    for index in 0..thread_count {
        let part_path = part_path_for(target_path, index);

        if !part_path.exists() {
            continue;
        }

        let mut part_file = File::open(&part_path)
            .map_err(|error| format!("Failed to open part file {}: {}", index, error))?;

        let copied = std::io::copy(&mut part_file, &mut output_file)
            .map_err(|error| format!("Failed to merge part file {}: {}", index, error))?;

        merged_size += copied;

        fs::remove_file(&part_path)
            .map_err(|error| format!("Failed to remove part file {}: {}", index, error))?;
    }

    output_file
        .flush()
        .map_err(|error| format!("Failed to flush merged download file: {}", error))?;

    if merged_size != expected_size {
        let _ = fs::remove_file(temp_path);
        return Err(format!(
            "merged file size mismatch: expected {}, got {}",
            format_bytes(expected_size),
            format_bytes(merged_size)
        ));
    }

    Ok(())
}

fn format_bytes(bytes: u64) -> String {
    let mib = bytes as f64 / 1024.0 / 1024.0;

    if mib >= 1.0 {
        format!("{:.1} MB", mib)
    } else {
        let kib = bytes as f64 / 1024.0;
        format!("{:.1} KB", kib)
    }
}


fn is_http_url(url: &str) -> bool {
    let lower = url.to_lowercase();
    lower.starts_with("https://") || lower.starts_with("http://")
}

fn infer_zip_file_name_from_url(url: &str) -> String {
    let without_query = url.split('?').next().unwrap_or(url);
    let without_fragment = without_query.split('#').next().unwrap_or(without_query);

    let raw_name = without_fragment
        .rsplit('/')
        .next()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("downloaded-mod.zip");

    let mut file_name = sanitize_file_name(raw_name);

    if !file_name.to_lowercase().ends_with(".zip") {
        file_name = format!("{}.zip", file_name.trim_end_matches('.'));
    }

    if file_name == ".zip" || file_name.trim().is_empty() {
        file_name = format!("downloaded-mod-{}.zip", current_timestamp().unwrap_or(0));
    }

    file_name
}

fn unique_download_path(folder: &Path, file_name: &str) -> PathBuf {
    let mut candidate = folder.join(file_name);

    if !candidate.exists() {
        return candidate;
    }

    let stem = Path::new(file_name)
        .file_stem()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| "downloaded-mod".to_string());
    let extension = Path::new(file_name)
        .extension()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| "zip".to_string());

    for index in 1..1000 {
        candidate = folder.join(format!("{}-{}.{}", stem, index, extension));

        if !candidate.exists() {
            return candidate;
        }
    }

    folder.join(format!("{}-{}.{}", stem, current_timestamp().unwrap_or(0), extension))
}

fn download_generic_zip_file_with_progress(
    client: &Client,
    url: &str,
    target_path: &Path,
    app: &AppHandle,
    download_id: &str,
    file_name: &str,
    cancel_flag: &Arc<AtomicBool>,
    total_bytes: u64,
) -> Result<(), String> {
    let file_size = if total_bytes > 0 {
        total_bytes
    } else {
        get_remote_file_size(client, url).unwrap_or(0)
    };

    let result = match download_generic_zip_file_parallel(
        client,
        url,
        target_path,
        Some((app, download_id, file_name, file_size, cancel_flag)),
    ) {
        Ok(()) => Ok(()),
        Err(parallel_error) => {
            if cancel_flag.load(Ordering::Relaxed) {
                let _ = fs::remove_file(target_path);
                return Err("下载已取消".to_string());
            }

            eprintln!(
                "Parallel ZIP download failed, falling back to single connection: {}",
                parallel_error
            );

            download_generic_zip_file_single(
                client,
                url,
                target_path,
                Some((app, download_id, file_name, file_size, cancel_flag)),
            )
            .map_err(|single_error| {
                format!(
                    "多线程下载失败：{}；普通下载也失败：{}",
                    parallel_error, single_error
                )
            })
        }
    };

    let final_file_size = fs::metadata(target_path).map(|m| m.len()).unwrap_or(0);

    match &result {
        Ok(()) => {
            emit_download_progress(
                app,
                &DownloadProgressPayload {
                    download_id: download_id.to_string(),
                    file_name: file_name.to_string(),
                    stage: "completed".to_string(),
                    downloaded_bytes: final_file_size,
                    total_bytes: file_size,
                    speed_bytes_per_sec: 0,
                    message: "下载完成".to_string(),
                    zip_path: Some(target_path.to_string_lossy().to_string()),
                },
            );
            cancellation_map()
                .lock()
                .map_err(|e| e.to_string())?
                .remove(download_id);
        }
        Err(error_msg) => {
            if cancel_flag.load(Ordering::Relaxed) {
                cancellation_map()
                    .lock()
                    .map_err(|e| e.to_string())?
                    .remove(download_id);
                return Err("下载已取消".to_string());
            }

            emit_download_progress(
                app,
                &DownloadProgressPayload {
                    download_id: download_id.to_string(),
                    file_name: file_name.to_string(),
                    stage: "failed".to_string(),
                    downloaded_bytes: 0,
                    total_bytes: file_size,
                    speed_bytes_per_sec: 0,
                    message: error_msg.clone(),
                    zip_path: None,
                },
            );
            cancellation_map()
                .lock()
                .map_err(|e| e.to_string())?
                .remove(download_id);
        }
    }

    result
}

fn download_generic_zip_file_parallel(
    client: &Client,
    url: &str,
    target_path: &Path,
    progress: Option<(&AppHandle, &str, &str, u64, &Arc<AtomicBool>)>,
) -> Result<(), String> {
    const THREAD_COUNT: u64 = 8;
    const MIN_PARALLEL_SIZE: u64 = 2 * 1024 * 1024;

    let file_size = get_remote_file_size(client, url)?;

    if file_size < MIN_PARALLEL_SIZE {
        return Err(format!(
            "file is too small for parallel download: {}",
            format_bytes(file_size)
        ));
    }

    let temp_path = target_path.with_extension("zip.download");
    clear_download_artifacts(target_path, THREAD_COUNT)?;

    if let Some((app, download_id, file_name, total_size, _)) = &progress {
        if app
            .emit(
                "download-progress",
                DownloadProgressPayload {
                    download_id: download_id.to_string(),
                    file_name: file_name.to_string(),
                    stage: "downloading".to_string(),
                    downloaded_bytes: 0,
                    total_bytes: *total_size,
                    speed_bytes_per_sec: 0,
                    message: "正在多线程下载...".to_string(),
                    zip_path: None,
                },
            )
            .is_err()
        {
            // ignore emit errors
        }
    }

    let downloaded_total = Arc::new(AtomicU64::new(0));
    let next_report = Arc::new(AtomicU64::new(1024 * 1024));
    let mut handles = Vec::new();
    let chunk_size = (file_size + THREAD_COUNT - 1) / THREAD_COUNT;

    for index in 0..THREAD_COUNT {
        let start = index * chunk_size;

        if start >= file_size {
            continue;
        }

        let end = ((index + 1) * chunk_size - 1).min(file_size - 1);
        let expected_size = end - start + 1;
        let part_path = part_path_for(target_path, index);
        let url = url.to_string();
        let client = client.clone();
        let cancel_flag = progress.as_ref().map(|(_, _, _, _, cf)| Arc::clone(*cf));
        let downloaded_total = Arc::clone(&downloaded_total);
        let next_report = Arc::clone(&next_report);
        let progress_owned = progress.as_ref().map(|(a, did, fn_, ts, _)| {
            (AppHandle::clone(a), did.to_string(), fn_.to_string(), *ts)
        });

        let handle = thread::spawn(move || -> Result<(), String> {
            let range_header = format!("bytes={}-{}", start, end);

            let mut response = client
                .get(&url)
                .header(USER_AGENT, "Junimo Box")
                .header("Range", range_header)
                .send()
                .map_err(|error| format!("part {} request failed: {}", index, error))?;

            if response.status().as_u16() != 206 {
                return Err(format!(
                    "server did not return partial content for part {}: {}",
                    index,
                    response.status()
                ));
            }

            let mut output_file = File::create(&part_path)
                .map_err(|error| format!("failed to create part {} file: {}", index, error))?;

            let mut part_size = 0_u64;
            let mut buffer = [0_u8; 128 * 1024];

            loop {
                if let Some(ref cf) = cancel_flag {
                    if cf.load(Ordering::Relaxed) {
                        return Err("下载已取消".to_string());
                    }
                }

                let bytes_read = response
                    .read(&mut buffer)
                    .map_err(|error| format!("failed to read part {}: {}", index, error))?;

                if bytes_read == 0 {
                    break;
                }

                output_file
                    .write_all(&buffer[..bytes_read])
                    .map_err(|error| format!("failed to write part {}: {}", index, error))?;

                part_size += bytes_read as u64;
                let total = downloaded_total.fetch_add(bytes_read as u64, Ordering::Relaxed)
                    + bytes_read as u64;

                if let Some((ref app, ref did, ref fn_, ts)) = &progress_owned {
                    let mut report_at = next_report.load(Ordering::Relaxed);
                    while total >= report_at {
                        match next_report.compare_exchange(
                            report_at,
                            report_at + 1024 * 1024,
                            Ordering::Relaxed,
                            Ordering::Relaxed,
                        ) {
                            Ok(_) => {
                                let _ = app.emit(
                                    "download-progress",
                                    DownloadProgressPayload {
                                        download_id: did.to_string(),
                                        file_name: fn_.to_string(),
                                        stage: "downloading".to_string(),
                                        downloaded_bytes: total,
                                        total_bytes: *ts,
                                        speed_bytes_per_sec: 0,
                                        message: format!(
                                            "下载中：{} / {}",
                                            format_bytes(total),
                                            format_bytes(*ts)
                                        ),
                                        zip_path: None,
                                    },
                                );
                                break;
                            }
                            Err(current) => report_at = current,
                        }
                    }
                }
            }

            output_file
                .flush()
                .map_err(|error| format!("failed to flush part {}: {}", index, error))?;

            if part_size != expected_size {
                return Err(format!(
                    "part {} size mismatch: expected {}, got {}",
                    index,
                    format_bytes(expected_size),
                    format_bytes(part_size)
                ));
            }

            Ok(())
        });

        handles.push(handle);
    }

    for handle in handles {
        if let Some((_, _, _, _, cancel_flag)) = &progress {
            if cancel_flag.load(Ordering::Relaxed) {
                return Err("下载已取消".to_string());
            }
        }

        handle
            .join()
            .map_err(|_| "parallel download thread panicked".to_string())??;
    }

    if let Some((_, _, _, _, cancel_flag)) = &progress {
        if cancel_flag.load(Ordering::Relaxed) {
            return Err("下载已取消".to_string());
        }
    }

    merge_download_parts(target_path, &temp_path, THREAD_COUNT, file_size)?;

    if target_path.exists() {
        fs::remove_file(target_path)
            .map_err(|error| format!("Failed to replace old download file: {}", error))?;
    }

    fs::rename(&temp_path, target_path)
        .map_err(|error| format!("Failed to finalize downloaded file: {}", error))?;

    if let Some((_, _, _, _, cancel_flag)) = &progress {
        if cancel_flag.load(Ordering::Relaxed) {
            return Err("下载已取消".to_string());
        }
    }

    Ok(())
}

fn download_generic_zip_file_single(
    client: &Client,
    url: &str,
    target_path: &Path,
    progress: Option<(&AppHandle, &str, &str, u64, &Arc<AtomicBool>)>,
) -> Result<(), String> {
    let temp_path = target_path.with_extension("zip.download");

    if temp_path.exists() {
        fs::remove_file(&temp_path)
            .map_err(|error| format!("Failed to clear old temp download file: {}", error))?;
    }

    let mut response = client
        .get(url)
        .header(USER_AGENT, "Junimo Box")
        .send()
        .map_err(|error| format!("Request failed. Please check your network connection: {}", error))?
        .error_for_status()
        .map_err(|error| format!("Download failed: {}", error))?;

    let mut output_file = File::create(&temp_path)
        .map_err(|error| format!("Failed to create temp download file: {}", error))?;

    let mut downloaded_size = 0_u64;
    let mut next_report_size = 0_u64;
    let mut buffer = [0_u8; 128 * 1024];
    let mut last_emit = std::time::Instant::now();

    loop {
        if let Some((_, _, _, _, cancel_flag)) = &progress {
            if cancel_flag.load(Ordering::Relaxed) {
                let _ = fs::remove_file(&temp_path);
                return Err("下载已取消".to_string());
            }
        }

        let bytes_read = response
            .read(&mut buffer)
            .map_err(|error| format!("Failed to read downloaded data: {}", error))?;

        if bytes_read == 0 {
            break;
        }

        output_file
            .write_all(&buffer[..bytes_read])
            .map_err(|error| format!("Failed to save downloaded data: {}", error))?;

        downloaded_size += bytes_read as u64;

        if let Some((app, download_id, file_name, total_size, _)) = &progress {
            if downloaded_size >= next_report_size || last_emit.elapsed().as_secs() >= 2 {
                let _ = app.emit(
                    "download-progress",
                    DownloadProgressPayload {
                        download_id: download_id.to_string(),
                        file_name: file_name.to_string(),
                        stage: "downloading".to_string(),
                        downloaded_bytes: downloaded_size,
                        total_bytes: *total_size,
                        speed_bytes_per_sec: 0,
                        message: format!(
                            "下载中：{} / {}",
                            format_bytes(downloaded_size),
                            format_bytes(*total_size)
                        ),
                        zip_path: None,
                    },
                );
                next_report_size = downloaded_size + 1024 * 1024;
                last_emit = std::time::Instant::now();
            }
        }
    }

    if let Some((_, _, _, _, cancel_flag)) = &progress {
        if cancel_flag.load(Ordering::Relaxed) {
            let _ = fs::remove_file(&temp_path);
            return Err("下载已取消".to_string());
        }
    }

    output_file
        .flush()
        .map_err(|error| format!("Failed to flush downloaded file: {}", error))?;

    if downloaded_size == 0 {
        let _ = fs::remove_file(&temp_path);
        return Err("下载到的 ZIP 文件为空，请检查下载链接或网络连接。".to_string());
    }

    if downloaded_size < 1024 {
        let _ = fs::remove_file(&temp_path);
        return Err(format!(
            "下载到的 ZIP 文件过小：{}，下载可能失败。",
            format_bytes(downloaded_size)
        ));
    }

    if target_path.exists() {
        fs::remove_file(target_path)
            .map_err(|error| format!("Failed to replace old download file: {}", error))?;
    }

    fs::rename(&temp_path, target_path)
        .map_err(|error| format!("Failed to finalize downloaded file: {}", error))?;

    Ok(())
}

fn extract_zip_archive(zip_path: &Path, target_dir: &Path) -> Result<(), String> {
    let file =
        File::open(zip_path).map_err(|error| format!("Failed to open zip archive: {}", error))?;

    let mut archive =
        ZipArchive::new(file).map_err(|error| format!("Failed to read zip archive: {}", error))?;

    for index in 0..archive.len() {
        let mut zip_file = archive
            .by_index(index)
            .map_err(|error| format!("Failed to read zip entry: {}", error))?;

        if zip_file.is_dir() {
            continue;
        }

        let entry_name = normalize_zip_path(zip_file.name());

        if entry_name.contains("__MACOSX") {
            continue;
        }

        let output_path = safe_join(target_dir, &entry_name)?;

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("Failed to create extracted folder: {}", error))?;
        }

        let mut output_file = File::create(&output_path)
            .map_err(|error| format!("Failed to create extracted file: {}", error))?;

        std::io::copy(&mut zip_file, &mut output_file)
            .map_err(|error| format!("Failed to extract zip file: {}", error))?;
    }

    Ok(())
}

fn find_file_by_name(root: &Path, target_name: &str) -> Option<PathBuf> {
    let entries = fs::read_dir(root).ok()?;

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_file() {
            let file_name = path.file_name()?.to_string_lossy();

            if file_name.eq_ignore_ascii_case(target_name) {
                return Some(path);
            }
        }

        if path.is_dir() {
            if let Some(found) = find_file_by_name(&path, target_name) {
                return Some(found);
            }
        }
    }

    None
}

fn current_timestamp() -> Result<u64, String> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("Failed to create timestamp: {}", error))?
        .as_secs())
}

fn sanitize_file_name(file_name: &str) -> String {
    file_name
        .chars()
        .map(|character| match character {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            _ => character,
        })
        .collect()
}

fn get_json_string(value: &serde_json::Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(|item| item.as_str())
        .map(|item| item.to_string())
}

fn normalize_zip_dependencies(raw_dependencies: Option<&serde_json::Value>) -> Vec<ZipModDependency> {
    let Some(raw_dependencies) = raw_dependencies else {
        return Vec::new();
    };

    let Some(items) = raw_dependencies.as_array() else {
        return Vec::new();
    };

    items
        .iter()
        .filter_map(|item| {
            let unique_id = get_json_string(item, "UniqueID")?;

            let is_required = item
                .get("IsRequired")
                .and_then(|value| value.as_bool())
                .unwrap_or(true);

            Some(ZipModDependency {
                unique_id,
                is_required,
                is_installed: false,
            })
        })
        .collect()
}

fn normalize_zip_content_pack_for(
    raw_content_pack_for: Option<&serde_json::Value>,
) -> Option<ZipModDependency> {
    let raw_content_pack_for = raw_content_pack_for?;

    let unique_id = get_json_string(raw_content_pack_for, "UniqueID")?;

    Some(ZipModDependency {
        unique_id,
        is_required: true,
        is_installed: false,
    })
}

fn detect_mod_type(
    unique_id: &str,
    entry_dll: &str,
    content_pack_for: &Option<ZipModDependency>,
) -> String {
    if unique_id == "spacechase0.GenericModConfigMenu" {
        return "GMCM".to_string();
    }

    if let Some(content_pack_for) = content_pack_for {
        return match content_pack_for.unique_id.as_str() {
            "Pathoschild.ContentPatcher" => "Content Patcher 内容包".to_string(),
            "PeacefulEnd.FashionSense" => "Fashion Sense 内容包".to_string(),
            "spacechase0.JsonAssets" => "Json Assets 内容包".to_string(),
            "FlashShifter.StardewValleyExpandedCP" => "SVE 内容包".to_string(),
            _ => "内容包".to_string(),
        };
    }

    if !entry_dll.trim().is_empty() {
        return "SMAPI 插件".to_string();
    }

    "未知类型".to_string()
}

fn get_folder_from_manifest_path(manifest_path: &str) -> String {
    let normalized = normalize_zip_path(manifest_path);

    let parts: Vec<&str> = normalized
        .split('/')
        .filter(|part| !part.is_empty())
        .collect();

    if parts.len() >= 2 {
        return parts[parts.len() - 2].to_string();
    }

    "UnknownMod".to_string()
}

fn normalize_zip_path(path: &str) -> String {
    path.replace('\\', "/")
}

fn get_root_prefix_from_manifest_path(manifest_path: &str) -> String {
    let normalized = normalize_zip_path(manifest_path);

    match normalized.rfind('/') {
        Some(index) => normalized[..index + 1].to_string(),
        None => String::new(),
    }
}

fn find_matching_preview<'a>(
    entry_name: &str,
    previews: &'a [ZipModPreview],
) -> Option<(&'a ZipModPreview, String)> {
    for preview in previews {
        let prefix = get_root_prefix_from_manifest_path(&preview.manifest_path);

        if prefix.is_empty() {
            return Some((preview, entry_name.to_string()));
        }

        if entry_name.starts_with(&prefix) {
            let relative_path = entry_name[prefix.len()..].to_string();

            if !relative_path.is_empty() {
                return Some((preview, relative_path));
            }
        }
    }

    None
}

fn safe_join(base: &Path, relative_path: &str) -> Result<PathBuf, String> {
    let mut result = base.to_path_buf();
    let normalized = normalize_zip_path(relative_path);

    for part in normalized.split('/') {
        if part.is_empty() || part == "." {
            continue;
        }

        if part == ".." || part.contains(':') {
            return Err(format!("Unsafe zip path: {}", relative_path));
        }

        result.push(part);
    }

    Ok(result)
}




#[derive(Serialize)]
struct NexusUserInfo {
    name: String,
    user_id: u64,
    is_premium: bool,
}

#[tauri::command]
async fn test_nexus_api_key(api_key: String) -> Result<NexusUserInfo, String> {
    tauri::async_runtime::spawn_blocking(move || test_nexus_api_key_blocking(api_key))
        .await
        .map_err(|error| format!("Nexus API 测试任务失败：{}", error))?
}

fn test_nexus_api_key_blocking(api_key: String) -> Result<NexusUserInfo, String> {
    let trimmed_key = api_key.trim().to_string();

    if trimmed_key.is_empty() {
        return Err("请先填写 Nexus Personal API Key。".to_string());
    }

    let client = Client::builder()
        .user_agent("Junimo Box")
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(60))
        .build()
        .map_err(|error| format!("Failed to create HTTP client: {}", error))?;

    let response = client
        .get("https://api.nexusmods.com/v1/users/validate.json")
        .header(USER_AGENT, "Junimo Box")
        .header("apikey", trimmed_key)
        .send()
        .map_err(|error| format!("无法连接 Nexus Mods API：{}", error))?;

    let status = response.status();
    let body = response
        .text()
        .map_err(|error| format!("无法读取 Nexus Mods API 响应：{}", error))?;

    if !status.is_success() {
        if status.as_u16() == 401 {
            return Err("Nexus API Key 无效或已失效，请在 Nexus 账号设置里重新生成 Personal API Key。".to_string());
        }

        return Err(format!(
            "Nexus Mods API 返回错误：{}。响应：{}",
            status,
            body.chars().take(500).collect::<String>()
        ));
    }

    let value: serde_json::Value = serde_json::from_str(&body)
        .map_err(|error| format!("无法解析 Nexus Mods API 响应：{}。响应：{}", error, body))?;

    let name = value
        .get("name")
        .or_else(|| value.get("username"))
        .and_then(|item| item.as_str())
        .unwrap_or("Nexus 用户")
        .to_string();

    let user_id = value
        .get("user_id")
        .or_else(|| value.get("member_id"))
        .or_else(|| value.get("id"))
        .and_then(|item| item.as_u64())
        .unwrap_or(0);

    let is_premium = value
        .get("is_premium")
        .or_else(|| value.get("premium"))
        .and_then(|item| item.as_bool())
        .unwrap_or(false);

    Ok(NexusUserInfo {
        name,
        user_id,
        is_premium,
    })
}

#[derive(Clone)]
struct ParsedNxmLink {
    raw: String,
    game_domain: String,
    mod_id: String,
    file_id: String,
    key: String,
    expires: String,
    user_id: String,
}

#[tauri::command]
async fn download_nxm_file(
    app: AppHandle,
    nxm_link: String,
    game_path: String,
    api_key: Option<String>,
    download_id: String,
) -> Result<UrlZipDownloadResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        download_nxm_file_blocking(app, nxm_link, game_path, api_key, download_id)
    })
    .await
    .map_err(|error| format!("NXM download task failed: {}", error))?
}

fn download_nxm_file_blocking(
    app: AppHandle,
    nxm_link: String,
    game_path: String,
    api_key: Option<String>,
    download_id: String,
) -> Result<UrlZipDownloadResult, String> {
    let parsed = parse_nxm_link(&nxm_link)?;

    if parsed.game_domain.to_lowercase() != "stardewvalley" {
        return Err(format!(
            "当前只支持 Stardew Valley 的 NXM 链接，收到的是：{}",
            parsed.game_domain
        ));
    }

    let game_dir = Path::new(&game_path);

    if !game_dir.exists() {
        return Err(format!("Game folder does not exist: {}", game_path));
    }

    let client = Client::builder()
        .user_agent("Junimo Box")
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(300))
        .build()
        .map_err(|error| format!("Failed to create HTTP client: {}", error))?;

    let direct_url = resolve_nxm_download_url(&client, &parsed, api_key.as_deref())?;

    let downloads_dir = game_dir.join("Junimo Box Downloads").join("Nexus");
    fs::create_dir_all(&downloads_dir)
        .map_err(|error| format!("Failed to create Nexus download folder: {}", error))?;

    let file_name = format!(
        "nexus-{}-mod-{}-file-{}.zip",
        sanitize_file_name(&parsed.game_domain),
        sanitize_file_name(&parsed.mod_id),
        sanitize_file_name(&parsed.file_id)
    );
    let target_path = unique_download_path(&downloads_dir, &file_name);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    {
        let mut map = cancellation_map()
            .lock()
            .map_err(|e| e.to_string())?;
        map.insert(download_id.clone(), Arc::clone(&cancel_flag));
    }

    let result = download_generic_zip_file_with_progress(
        &client,
        &direct_url,
        &target_path,
        &app,
        &download_id,
        &file_name,
        &cancel_flag,
        0,
    );

    if cancel_flag.load(Ordering::Relaxed) {
        let _ = fs::remove_file(&target_path);
        {
            let mut map = cancellation_map()
                .lock()
                .map_err(|e| e.to_string())?;
            map.remove(&download_id);
        }
        return Err("下载已取消".to_string());
    }

    result?;

    let metadata = fs::metadata(&target_path)
        .map_err(|error| format!("Failed to read downloaded NXM ZIP metadata: {}", error))?;
    let file_size = metadata.len();

    if file_size == 0 {
        let _ = fs::remove_file(&target_path);
        return Err("下载到的 NXM ZIP 文件为空，请检查 Nexus 下载权限或网络连接。".to_string());
    }

    if file_size < 1024 {
        let _ = fs::remove_file(&target_path);
        return Err(format!(
            "下载到的 NXM ZIP 文件过小：{}，下载可能失败。",
            format_bytes(file_size)
        ));
    }

    {
        let mut map = cancellation_map()
            .lock()
            .map_err(|e| e.to_string())?;
        map.remove(&download_id);
    }

    Ok(UrlZipDownloadResult {
        download_url: parsed.raw,
        zip_path: target_path.to_string_lossy().to_string(),
        file_name: target_path
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or(file_name),
        file_size,
        download_id,
    })
}

fn parse_nxm_link(link: &str) -> Result<ParsedNxmLink, String> {
    let trimmed = link.trim();

    if !trimmed.to_lowercase().starts_with("nxm:") {
        return Err("这不是有效的 nxm:// 链接。".to_string());
    }

    let without_scheme = trimmed
        .trim_start_matches("nxm://")
        .trim_start_matches("NXM://")
        .trim_start_matches("nxm:")
        .trim_start_matches("NXM:")
        .trim_start_matches('/');

    let mut split = without_scheme.splitn(2, '?');
    let path_part = split.next().unwrap_or("");
    let query_part = split.next().unwrap_or("");

    let parts: Vec<&str> = path_part
        .split('/')
        .filter(|part| !part.trim().is_empty())
        .collect();

    let game_domain = parts
        .get(0)
        .map(|value| value.to_string())
        .unwrap_or_default();

    let mods_index = parts
        .iter()
        .position(|part| part.eq_ignore_ascii_case("mods"));
    let files_index = parts
        .iter()
        .position(|part| part.eq_ignore_ascii_case("files"));

    let mod_id = mods_index
        .and_then(|index| parts.get(index + 1))
        .map(|value| value.to_string())
        .unwrap_or_default();

    let file_id = files_index
        .and_then(|index| parts.get(index + 1))
        .map(|value| value.to_string())
        .unwrap_or_default();

    let key = get_query_param(query_part, "key");
    let expires = get_query_param(query_part, "expires");
    let user_id = get_query_param(query_part, "user_id");

    if game_domain.trim().is_empty() || mod_id.trim().is_empty() || file_id.trim().is_empty() {
        return Err("无法从 NXM 链接解析 game / modId / fileId。".to_string());
    }

    if key.trim().is_empty() || expires.trim().is_empty() {
        return Err("NXM 链接缺少 key 或 expires 参数，无法自动下载。请重新从 Nexus 点击 Mod Manager Download。".to_string());
    }

    Ok(ParsedNxmLink {
        raw: trimmed.to_string(),
        game_domain,
        mod_id,
        file_id,
        key,
        expires,
        user_id,
    })
}

fn get_query_param(query: &str, key: &str) -> String {
    for pair in query.split('&') {
        let mut parts = pair.splitn(2, '=');
        let Some(raw_key) = parts.next() else {
            continue;
        };
        let raw_value = parts.next().unwrap_or("");

        if raw_key.eq_ignore_ascii_case(key) {
            return raw_value.to_string();
        }
    }

    String::new()
}

fn resolve_nxm_download_url(
    client: &Client,
    parsed: &ParsedNxmLink,
    api_key: Option<&str>,
) -> Result<String, String> {
    let trimmed_api_key = api_key.unwrap_or("").trim();

    if trimmed_api_key.is_empty() {
        return Err("NXM 自动下载需要 Nexus Personal API Key。请先到 设置 → Nexus Mods 保存并测试 API Key。".to_string());
    }

    let mut url = format!(
        "https://api.nexusmods.com/v1/games/{}/mods/{}/files/{}/download_link.json?key={}&expires={}",
        parsed.game_domain,
        parsed.mod_id,
        parsed.file_id,
        parsed.key,
        parsed.expires
    );

    if !parsed.user_id.trim().is_empty() {
        url.push_str("&user_id=");
        url.push_str(&parsed.user_id);
    }

    let response = client
        .get(&url)
        .header(USER_AGENT, "Junimo Box")
        .header("apikey", trimmed_api_key)
        .send()
        .map_err(|error| format!("无法连接 Nexus NXM 下载接口：{}", error))?;

    let status = response.status();
    let body = response
        .text()
        .map_err(|error| format!("无法读取 Nexus NXM 下载响应：{}", error))?;

    if !status.is_success() {
        let snippet = body.chars().take(500).collect::<String>();

        if status.as_u16() == 401 {
            return Err(format!(
                "Nexus 认证失败：API Key 无效、未保存，或 NXM 请求没有通过认证。请到 设置 → Nexus Mods 重新保存并测试 API Key。响应：{}",
                snippet
            ));
        }

        if status.as_u16() == 403 {
            return Err(format!(
                "Nexus 权限不足：当前账号可能无法通过 API 直接下载此文件。可以打开 Nexus 页面完成下载后，再回到 Junimo Box 选择 ZIP 预览安装。响应：{}",
                snippet
            ));
        }

        return Err(format!(
            "Nexus NXM 下载接口返回错误：{}。响应：{}",
            status,
            snippet
        ));
    }

    let value: serde_json::Value = serde_json::from_str(&body)
        .map_err(|error| format!("无法解析 Nexus NXM 下载响应：{}。响应：{}", error, body))?;

    extract_download_uri_from_nexus_response(&value).ok_or_else(|| {
        format!(
            "Nexus NXM 响应中没有可用下载地址：{}",
            value
        )
    })
}

fn extract_download_uri_from_nexus_response(value: &serde_json::Value) -> Option<String> {
    if let Some(array) = value.as_array() {
        for item in array {
            if let Some(uri) = extract_download_uri_from_nexus_response(item) {
                return Some(uri);
            }
        }
    }

    if let Some(object) = value.as_object() {
        for key in ["URI", "uri", "url", "download_url", "downloadUrl"] {
            if let Some(uri) = object.get(key).and_then(|item| item.as_str()) {
                if uri.starts_with("http://") || uri.starts_with("https://") {
                    return Some(uri.to_string());
                }
            }
        }

        for key in ["data", "links", "results"] {
            if let Some(child) = object.get(key) {
                if let Some(uri) = extract_download_uri_from_nexus_response(child) {
                    return Some(uri);
                }
            }
        }
    }

    None
}

fn state_dir() -> Result<PathBuf, String> {
    #[cfg(target_os = "windows")]
    {
        let base = std::env::var("LOCALAPPDATA")
            .or_else(|_| std::env::var("APPDATA"))
            .map_err(|_| "Failed to read LOCALAPPDATA / APPDATA environment variable".to_string())?;
        let dir = PathBuf::from(base).join("JunimoBox");
        fs::create_dir_all(&dir)
            .map_err(|error| format!("Failed to create Junimo Box state folder: {}", error))?;
        return Ok(dir);
    }

    #[cfg(not(target_os = "windows"))]
    {
        let dir = std::env::temp_dir().join("JunimoBox");
        fs::create_dir_all(&dir)
            .map_err(|error| format!("Failed to create Junimo Box state folder: {}", error))?;
        return Ok(dir);
    }
}

fn lock_file_path() -> Result<PathBuf, String> {
    Ok(state_dir()?.join("junimo-box.lock"))
}

fn pending_nxm_file_path() -> Result<PathBuf, String> {
    Ok(state_dir()?.join("pending-nxm-link.txt"))
}

fn extract_startup_nxm_arg() -> Option<String> {
    std::env::args()
        .find(|arg| arg.to_lowercase().starts_with("nxm://") || arg.to_lowercase().starts_with("nxm:"))
}

#[cfg(target_os = "windows")]
fn is_pid_running(pid: u32) -> bool {
    let output = Command::new("tasklist")
        .args(["/FI", &format!("PID eq {}", pid), "/FO", "CSV", "/NH"])
        .output();

    let Ok(output) = output else {
        return false;
    };

    if !output.status.success() {
        return false;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.contains(&format!("\"{}\"", pid)) || stdout.contains(&pid.to_string())
}

#[cfg(not(target_os = "windows"))]
fn is_pid_running(_pid: u32) -> bool {
    false
}

fn existing_instance_is_running() -> bool {
    let Ok(path) = lock_file_path() else {
        return false;
    };

    let Ok(text) = fs::read_to_string(path) else {
        return false;
    };

    let Ok(pid) = text.trim().parse::<u32>() else {
        return false;
    };

    if pid == std::process::id() {
        return false;
    }

    is_pid_running(pid)
}

fn write_current_instance_lock() {
    if let Ok(path) = lock_file_path() {
        let _ = fs::write(path, std::process::id().to_string());
    }
}

fn write_pending_nxm_link(link: &str) -> Result<(), String> {
    let path = pending_nxm_file_path()?;
    fs::write(path, link)
        .map_err(|error| format!("Failed to write pending NXM link: {}", error))
}

#[tauri::command]
fn read_pending_nxm_link() -> Result<Option<String>, String> {
    let path = pending_nxm_file_path()?;

    if !path.exists() {
        return Ok(None);
    }

    let link = fs::read_to_string(&path)
        .map_err(|error| format!("Failed to read pending NXM link: {}", error))?;

    let _ = fs::remove_file(&path);

    let trimmed = link.trim().to_string();

    if trimmed.is_empty() {
        return Ok(None);
    }

    Ok(Some(trimmed))
}

#[tauri::command]
fn register_nxm_protocol() -> Result<(), String> {
    #[cfg(not(target_os = "windows"))]
    {
        return Err("当前第一版 NXM 协议关联只支持 Windows。".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        let exe_path = std::env::current_exe()
            .map_err(|error| format!("Failed to resolve current executable: {}", error))?;

        let exe = exe_path.to_string_lossy().to_string();
        let command_value = format!("\"{}\" \"%1\"", exe);

        run_reg_command(&[
            "add",
            r"HKCU\Software\Classes\nxm",
            "/ve",
            "/d",
            "URL:NXM Protocol",
            "/f",
        ])?;

        run_reg_command(&[
            "add",
            r"HKCU\Software\Classes\nxm",
            "/v",
            "URL Protocol",
            "/d",
            "",
            "/f",
        ])?;

        run_reg_command(&[
            "add",
            r"HKCU\Software\Classes\nxm\shell\open\command",
            "/ve",
            "/d",
            &command_value,
            "/f",
        ])?;

        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn run_reg_command(args: &[&str]) -> Result<(), String> {
    let output = Command::new("reg")
        .args(args)
        .output()
        .map_err(|error| format!("Failed to run registry command: {}", error))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

        return Err(format!(
            "Registry command failed: {}{}{}",
            stdout,
            if stdout.is_empty() || stderr.is_empty() { "" } else { " | " },
            stderr
        ));
    }

    Ok(())
}

#[tauri::command]
fn read_startup_nxm_link() -> Result<Option<String>, String> {
    Ok(extract_startup_nxm_arg())
}

#[tauri::command]
fn open_url_in_browser(url: String) -> Result<(), String> {
    if !url.to_lowercase().starts_with("http://") && !url.to_lowercase().starts_with("https://") {
        return Err("Only http:// and https:// URLs can be opened.".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .arg("/C")
            .arg("start")
            .arg("")
            .arg(url)
            .spawn()
            .map_err(|error| format!("Failed to open URL in browser: {}", error))?;

        return Ok(());
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(url)
            .spawn()
            .map_err(|error| format!("Failed to open URL in browser: {}", error))?;

        return Ok(());
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        Command::new("xdg-open")
            .arg(url)
            .spawn()
            .map_err(|error| format!("Failed to open URL in browser: {}", error))?;

        return Ok(());
    }
}

#[tauri::command]
fn cancel_download(download_id: String) -> Result<(), String> {
    let map = cancellation_map()
        .lock()
        .map_err(|e| e.to_string())?;

    if let Some(flag) = map.get(&download_id) {
        flag.store(true, Ordering::Relaxed);
    }

    Ok(())
}

#[derive(Serialize, Clone)]
struct DeletedModInfo {
    folder_name: String,
    original_name: String,
    deleted_at: String,
}

fn parse_deleted_folder_name(folder_name: &str) -> (String, String) {
    let bytes = folder_name.as_bytes();
    if bytes.len() < 24 {
        return (folder_name.to_string(), String::new());
    }

    for i in (0..bytes.len().saturating_sub(19)).rev() {
        if bytes[i] == b'-'
            && bytes[i + 1].is_ascii_digit()
            && bytes[i + 2].is_ascii_digit()
            && bytes[i + 3].is_ascii_digit()
            && bytes[i + 4].is_ascii_digit()
            && bytes[i + 5] == b'-'
            && bytes[i + 6].is_ascii_digit()
            && bytes[i + 7].is_ascii_digit()
            && bytes[i + 8] == b'-'
            && bytes[i + 9].is_ascii_digit()
            && bytes[i + 10].is_ascii_digit()
            && bytes[i + 11] == b'T'
        {
            let original = &folder_name[..i];
            if original.is_empty() {
                break;
            }
            let ts = &folder_name[i + 1..];
            return (original.to_string(), ts.to_string());
        }
    }

    (folder_name.to_string(), String::new())
}

fn recycle_bin_path(game_path: &str) -> PathBuf {
    Path::new(game_path).join("Junimo Box Deleted Mods")
}

fn mods_path(game_path: &str) -> PathBuf {
    Path::new(game_path).join("Mods")
}

fn disabled_mods_path(game_path: &str) -> PathBuf {
    Path::new(game_path).join("Disabled Mods")
}

#[tauri::command]
fn list_deleted_mods(game_path: String) -> Result<Vec<DeletedModInfo>, String> {
    let bin = recycle_bin_path(&game_path);
    if !bin.exists() {
        return Ok(Vec::new());
    }

    let mut items = Vec::new();
    let mut entries: Vec<_> = fs::read_dir(&bin)
        .map_err(|error| format!("Failed to read recycle bin: {}", error))?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .collect();

    entries.sort_by(|a, b| {
        let am = a.metadata().and_then(|m| m.created()).ok();
        let bm = b.metadata().and_then(|m| m.created()).ok();
        bm.cmp(&am)
    });

    for entry in entries {
        let folder_name = entry.file_name().to_string_lossy().to_string();
        let (original_name, deleted_at) = parse_deleted_folder_name(&folder_name);
        items.push(DeletedModInfo {
            folder_name,
            original_name,
            deleted_at,
        });
    }

    Ok(items)
}

#[tauri::command]
fn restore_deleted_mod(game_path: String, folder_name: String) -> Result<String, String> {
    let from = recycle_bin_path(&game_path).join(&folder_name);
    if !from.exists() {
        return Err(format!("Deleted mod folder not found: {}", folder_name));
    }

    let (original_name, _) = parse_deleted_folder_name(&folder_name);
    let mods_dir = mods_path(&game_path);

    let mut target = mods_dir.join(&original_name);
    if target.exists() {
        let stem = &original_name;
        for n in 1..100 {
            target = mods_dir.join(format!("{}-恢复-{}", stem, n));
            if !target.exists() {
                break;
            }
        }
    }

    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("Failed to create Mods folder: {}", error))?;
    }

    fs::rename(&from, &target).map_err(|error| {
        format!("Failed to restore deleted mod: {}", error)
    })?;

    Ok(target
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or(original_name))
}

#[tauri::command]
fn permanently_delete_mod(game_path: String, folder_name: String) -> Result<(), String> {
    let target = recycle_bin_path(&game_path).join(&folder_name);
    if !target.exists() {
        return Err(format!("Mod folder not found in recycle bin: {}", folder_name));
    }

    fs::remove_dir_all(&target).map_err(|error| {
        format!("Failed to permanently delete mod: {}", error)
    })?;

    Ok(())
}

#[tauri::command]
fn empty_recycle_bin(game_path: String) -> Result<(), String> {
    let bin = recycle_bin_path(&game_path);
    if !bin.exists() {
        return Ok(());
    }

    fs::remove_dir_all(&bin).map_err(|error| {
        format!("Failed to empty recycle bin: {}", error)
    })?;

    fs::create_dir_all(&bin).map_err(|error| {
        format!("Failed to recreate recycle bin: {}", error)
    })?;

    Ok(())
}

#[tauri::command]
fn export_mods_backup(game_path: String, backup_path: String) -> Result<String, String> {
    let mods_dir = mods_path(&game_path);
    let disabled_dir = disabled_mods_path(&game_path);

    let mut enabled_mods: Vec<String> = Vec::new();
    let mut disabled_mods: Vec<String> = Vec::new();

    if mods_dir.exists() {
        let mut entries: Vec<_> = fs::read_dir(&mods_dir)
            .map_err(|error| format!("Failed to read Mods folder: {}", error))?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().map(|t| t.is_dir()).unwrap_or(false))
            .map(|entry| entry.file_name().to_string_lossy().to_string())
            .collect();
        entries.sort();
        enabled_mods = entries;
    }

    if disabled_dir.exists() {
        let mut entries: Vec<_> = fs::read_dir(&disabled_dir)
            .map_err(|error| format!("Failed to read Disabled Mods folder: {}", error))?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().map(|t| t.is_dir()).unwrap_or(false))
            .map(|entry| entry.file_name().to_string_lossy().to_string())
            .collect();
        entries.sort();
        disabled_mods = entries;
    }

    let now = chrono_now_fallback();
    let backup = serde_json::json!({
        "app": "Junimo Box",
        "type": "mods-backup",
        "version": 1,
        "exportedAt": now,
        "enabledMods": enabled_mods,
        "disabledMods": disabled_mods,
    });

    let content = serde_json::to_string_pretty(&backup)
        .map_err(|error| format!("Failed to serialize backup: {}", error))?;

    let path = Path::new(&backup_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("Failed to create backup parent folder: {}", error))?;
    }

    fs::write(path, content)
        .map_err(|error| format!("Failed to write backup file: {}", error))?;

    let total = backup["enabledMods"].as_array().map(|a| a.len()).unwrap_or(0)
        + backup["disabledMods"].as_array().map(|a| a.len()).unwrap_or(0);

    Ok(format!(
        "备份完成：已启用 {} 个，已禁用 {} 个，共 {} 个 Mod",
        enabled_mods.len(),
        disabled_mods.len(),
        total
    ))
}

fn chrono_now_fallback() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    let secs = now.as_secs();
    const SECS_PER_DAY: u64 = 86400;
    const DAYS_FROM_0000_TO_1970: u64 = 719468;
    let days = secs / SECS_PER_DAY;
    let remaining = secs % SECS_PER_DAY;
    let hours = remaining / 3600;
    let minutes = (remaining % 3600) / 60;
    let seconds = remaining % 60;

    let z = days + DAYS_FROM_0000_TO_1970;
    let era_days = z % 146097;
    let era_years = (era_days - era_days / 1460 + era_days / 36524 - era_days / 146096) / 365;
    let y = era_years + 2000;
    let yday = era_days - (365 * era_years + era_years / 4 - era_years / 100);
    let mp = (5 * yday + 2) / 153;
    let d = yday - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        y, m, d, hours, minutes, seconds
    )
}

#[tauri::command]
fn import_mods_backup(game_path: String, backup_path: String) -> Result<String, String> {
    let content = fs::read_to_string(&backup_path)
        .map_err(|error| format!("Failed to read backup file: {}", error))?;

    let parsed: serde_json::Value = serde_json::from_str(&content)
        .map_err(|error| format!("Failed to parse backup file: {}", error))?;

    if parsed.get("app").and_then(|v| v.as_str()) != Some("Junimo Box") {
        return Err("Invalid backup file: not a Junimo Box backup".to_string());
    }
    if parsed.get("type").and_then(|v| v.as_str()) != Some("mods-backup") {
        return Err("Invalid backup file: not a mods backup".to_string());
    }

    let mods_dir = mods_path(&game_path);
    let disabled_dir = disabled_mods_path(&game_path);

    fs::create_dir_all(&mods_dir)
        .map_err(|error| format!("Failed to ensure Mods folder: {}", error))?;
    fs::create_dir_all(&disabled_dir)
        .map_err(|error| format!("Failed to ensure Disabled Mods folder: {}", error))?;

    let enabled_mods: Vec<String> = parsed["enabledMods"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let disabled_mods: Vec<String> = parsed["disabledMods"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let mut restored_count = 0;
    let mut disabled_count = 0;

    for folder_name in &enabled_mods {
        let from = disabled_dir.join(folder_name);
        let to = mods_dir.join(folder_name);
        if from.exists() && !to.exists() {
            fs::rename(&from, &to).map_err(|error| {
                format!("Failed to restore mod '{}': {}", folder_name, error)
            })?;
            restored_count += 1;
        }
    }

    for folder_name in &disabled_mods {
        let from = mods_dir.join(folder_name);
        let to = disabled_dir.join(folder_name);
        if from.exists() && !to.exists() {
            fs::rename(&from, &to).map_err(|error| {
                format!("Failed to disable mod '{}': {}", folder_name, error)
            })?;
            disabled_count += 1;
        }
    }

    Ok(format!(
        "还原完成：已启用 {} 个，已禁用 {} 个",
        restored_count, disabled_count
    ))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let startup_nxm_link = extract_startup_nxm_arg();

    if existing_instance_is_running() {
        if let Some(link) = startup_nxm_link {
            let _ = write_pending_nxm_link(&link);
        }

        return;
    }

    write_current_instance_lock();

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
            preview_zip_mods,
            install_zip_mods,
            download_zip_from_url,
            install_latest_smapi,
            download_nxm_file,
            test_nexus_api_key,
            register_nxm_protocol,
            read_startup_nxm_link,
            read_pending_nxm_link,
            open_url_in_browser,
            cancel_download,
            list_deleted_mods,
            restore_deleted_mod,
            permanently_delete_mod,
            empty_recycle_bin,
            export_mods_backup,
            import_mods_backup
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
