use std::fs::{create_dir_all, read_to_string, write};
use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use short_uuid::ShortUuid;

use tauri::path::BaseDirectory;

#[derive(Serialize, Deserialize, Clone)]
pub struct Prompt {
    pub id: String,
    pub title: String,
    pub tip: String,
    pub content: String,
    pub tags: Vec<String>,
}

impl Default for Prompt {
    fn default() -> Self {
        Self {
            id: ShortUuid::generate().to_string(),
            title: String::from("新的提示词"),
            tip: String::new(),
            content: String::from("你是一个 AI 助手..."),
            tags: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PromptManager {
    pub data_file_path: PathBuf,
    pub prompts: Mutex<Vec<Prompt>>,
}

const DATA_FILE: &str = "prompts.json";

#[allow(unused)]
impl PromptManager {
    pub fn from_app(app: &tauri::AppHandle) -> Self {
        use tauri::Manager;

        let data_file_path = app
            .path()
            .resolve(DATA_FILE, BaseDirectory::AppLocalData)
            .expect("路径解析失败");

        if let Some(parent) = data_file_path.parent() {
            if !parent.exists() {
                create_dir_all(parent).expect("目录创建失败");
            }
        }

        let prompts = if data_file_path.exists() {
            let content = read_to_string(&data_file_path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        };

        Self {
            data_file_path,
            prompts: Mutex::new(prompts),
        }
    }

    pub fn create_prompt(&self) -> Result<Vec<Prompt>, String> {
        let mut prompts = self.prompts.lock().map_err(|e| e.to_string())?;
        let new_prompt = Prompt::default();
        prompts.push(new_prompt.clone());

        self.save()?;
        Ok(prompts.clone())
    }

    pub fn read_prompts(&self) -> Result<Vec<Prompt>, String> {
        let prompts = self.prompts.lock().map_err(|e| e.to_string())?;
        Ok(prompts.clone())
    }

    pub fn update_prompt(&self, new_prompt: Prompt) -> Result<(), String> {
        let mut prompts = self.prompts.lock().map_err(|e| e.to_string())?;
        if let Some(index) = prompts.iter().position(|p| p.id == new_prompt.id) {
            prompts[index] = new_prompt;
            self.save()?;
            Ok(())
        } else {
            Err("未找到对应的提示词 ID".to_string())
        }
    }

    pub fn delete_prompt(&self, deleted_id: String) -> Result<(), String> {
        let mut prompts = self.prompts.lock().map_err(|e| e.to_string())?;

        let initial_len = prompts.len();
        // 必须要记录长度，没删掉也不会有任何错误提示
        prompts.retain(|p| p.id != deleted_id);

        // 少一个代表有删了
        if prompts.len() < initial_len {
            self.save()?;
            Ok(())
        } else {
            Err("未找到要删除的 ID".to_string())
        }
    }

    fn save(&self) -> Result<(), String> {
        let json = serde_json::to_string(&self.prompts).map_err(|e| e.to_string())?;

        write(&self.data_file_path, json).map_err(|e| e.to_string())?;

        Ok(())
    }
}
