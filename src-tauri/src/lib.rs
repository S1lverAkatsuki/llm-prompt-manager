mod prompt_manager;
use prompt_manager::PromptManager;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            //  from_app 接收 &AppHandle，所以这里调用 app.handle()
            // 原来还可以用这个
            let manager = PromptManager::from_app(app.handle());

            // 注册到 Tauri 维护的状态机内，Manager 就会在整个应用的生命周期内单例运行
            app.manage(manager);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
