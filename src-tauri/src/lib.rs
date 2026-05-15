mod prompt_manager;
use std::sync::{Arc, Mutex};

use prompt_manager::{ModelConfig, Prompt, PromptData, PromptManager};
use tauri::{Manager, State};

#[tauri::command]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
fn create(
    state: State<'_, Arc<Mutex<PromptManager>>>,
    draft: PromptData,
) -> Result<Prompt, String> {
    let mut manager = state.lock().map_err(|e| e.to_string())?;
    manager.create_prompt(draft)
}

#[tauri::command]
fn read(state: tauri::State<'_, Arc<Mutex<PromptManager>>>) -> Result<Vec<Prompt>, String> {
    let mut manager = state.lock().map_err(|e| e.to_string())?;
    manager.read_prompts()
}

#[tauri::command]
fn update(state: State<'_, Arc<Mutex<PromptManager>>>, new_prompt: Prompt) -> Result<(), String> {
    let mut manager = state.lock().map_err(|e| e.to_string())?;
    manager.update_prompt(new_prompt)
}

#[tauri::command]
fn delete(state: State<'_, Arc<Mutex<PromptManager>>>, deleted_id: String) -> Result<(), String> {
    let mut manager = state.lock().map_err(|e| e.to_string())?;
    manager.delete_prompt(deleted_id)
}

#[tauri::command]
fn reorder(
    state: State<'_, Arc<Mutex<PromptManager>>>,
    ordered_ids: Vec<String>,
) -> Result<(), String> {
    let mut manager = state.lock().map_err(|e| e.to_string())?;
    manager.reorder_prompts(ordered_ids)
}

#[tauri::command]
fn get_data_path(state: State<'_, Arc<Mutex<PromptManager>>>) -> Result<String, String> {
    let manager = state.lock().map_err(|e| e.to_string())?;
    let path_str = manager
        .get_data_file_path()
        .to_str()
        .ok_or_else(|| String::from("路径可能不是 UTF-8"))?;

    Ok(path_str.to_string())
}

#[tauri::command]
fn get_dark_mode(state: State<'_, Arc<Mutex<PromptManager>>>) -> Result<bool, String> {
    let manager = state.lock().map_err(|e| e.to_string())?;
    Ok(manager.get_dark_mode())
}

#[tauri::command]
fn set_dark_mode(
    state: State<'_, Arc<Mutex<PromptManager>>>,
    new_mode: bool,
) -> Result<(), String> {
    let mut manager = state.lock().map_err(|e| e.to_string())?;
    manager.set_dark_mode(new_mode)
}

#[tauri::command]
fn get_all_tags(state: State<'_, Arc<Mutex<PromptManager>>>) -> Result<Vec<String>, String> {
    let manager = state.lock().map_err(|e| e.to_string())?;
    Ok(manager.get_all_tags())
}

#[tauri::command]
fn add_tag(state: State<'_, Arc<Mutex<PromptManager>>>, tag: String) -> Result<(), String> {
    let mut manager = state.lock().map_err(|e| e.to_string())?;
    manager.add_tag(tag)
}

#[tauri::command]
fn remove_tag(state: State<'_, Arc<Mutex<PromptManager>>>, tag: String) -> Result<(), String> {
    let mut manager = state.lock().map_err(|e| e.to_string())?;
    manager.remove_tag(tag)
}

#[tauri::command]
fn get_ai_enabled(state: State<'_, Arc<Mutex<PromptManager>>>) -> Result<bool, String> {
    let manager = state.lock().map_err(|e| e.to_string())?;
    Ok(manager.get_ai_enabled())
}

#[tauri::command]
fn set_ai_enabled(
    state: State<'_, Arc<Mutex<PromptManager>>>,
    enabled: bool,
) -> Result<(), String> {
    let mut manager = state.lock().map_err(|e| e.to_string())?;
    manager.set_ai_enabled(enabled)
}

#[tauri::command]
fn get_model_config(state: State<'_, Arc<Mutex<PromptManager>>>) -> Result<Option<ModelConfig>, String> {
    let manager = state.lock().map_err(|e| e.to_string())?;
    Ok(manager.get_model_config())
}

#[tauri::command]
fn set_model_config(
    state: State<'_, Arc<Mutex<PromptManager>>>,
    config: ModelConfig,
) -> Result<(), String> {
    let mut manager = state.lock().map_err(|e| e.to_string())?;
    manager.set_model_config(config)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            //  from_app 接收 &AppHandle，所以这里调用 app.handle()
            let manager = PromptManager::from_app_handle(app.handle());

            let shared = Arc::new(Mutex::new(manager));

            // 注册到 Tauri 维护的状态机内，Manager 就会在整个应用的生命周期内单例运行
            app.manage(shared.clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create,
            read,
            update,
            delete,
            reorder,
            get_data_path,
            get_version,
            get_dark_mode,
            set_dark_mode,
            get_all_tags,
            add_tag,
            remove_tag,
            get_ai_enabled,
            set_ai_enabled,
            get_model_config,
            set_model_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
