mod prompt_manager;
use std::sync::{Arc, Mutex};

use prompt_manager::{Prompt, PromptManager};
use tauri::{Manager, State};

#[tauri::command]
fn create(state: State<'_, Arc<Mutex<PromptManager>>>) -> Result<Vec<Prompt>, String> {
    let mut manager = state.lock().map_err(|e| e.to_string())?;
    manager.create_prompt()
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            //  from_app 接收 &AppHandle，所以这里调用 app.handle()
            let manager = PromptManager::empty(app.handle());

            let shared = Arc::new(Mutex::new(manager));

            // 注册到 Tauri 维护的状态机内，Manager 就会在整个应用的生命周期内单例运行
            app.manage(shared.clone());

            std::thread::spawn(move || {
                if let Ok(mut mtxgd) = shared.lock() {
                    let _ = mtxgd.loaded_prompt();
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create, read, update, delete])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
