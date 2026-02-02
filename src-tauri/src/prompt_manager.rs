use std::fs::{create_dir_all, read_to_string, write};
use std::path::PathBuf;

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

#[derive(Serialize, Deserialize, Clone)]
pub struct PromptData {
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

impl Prompt {
    fn from_data(data: PromptData) -> Self {
        Self {
            id: ShortUuid::generate().to_string(),
            title: data.title,
            tip: data.tip,
            content: data.content,
            tags: data.tags,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PromptManager {
    is_in_dark_mode: bool,
    #[serde(skip)]
    // 能在序列化和反序列化时跳过下面这条
    data_file_path: PathBuf,
    pub prompts: Vec<Prompt>,
}

impl Default for PromptManager {
    fn default() -> Self {
        Self {
            is_in_dark_mode: false,
            data_file_path: PathBuf::new(),
            prompts: Vec::new(),
        }
    }
}

const DATA_FILE: &str = "config.json";

impl PromptManager {
    pub fn from_app_handle(app: &tauri::AppHandle) -> Self {
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

        let config = if data_file_path.exists() {
            let content = read_to_string(&data_file_path).unwrap_or_default();

            if content.trim().is_empty() {
                PromptManager::default()
            } else {
                serde_json::from_str::<PromptManager>(&content).unwrap_or_default()
            }
        } else {
            PromptManager::default()
        };

        Self {
            data_file_path,
            ..config
        }
    }

    pub fn create_prompt(&mut self, data: PromptData) -> Result<Prompt, String> {
        let new_prompt = Prompt::from_data(data);
        self.prompts.push(new_prompt.clone());

        self.save()?;
        Ok(new_prompt)
    }

    pub fn read_prompts(&mut self) -> Result<Vec<Prompt>, String> {
        Ok(self.prompts.clone())
    }

    pub fn update_prompt(&mut self, new_prompt: Prompt) -> Result<(), String> {
        if let Some(index) = self.prompts.iter().position(|p| p.id == new_prompt.id) {
            self.prompts[index] = new_prompt;
            self.save()?;
            Ok(())
        } else {
            Err("未找到对应项的 ID".to_string())
        }
    }

    pub fn delete_prompt(&mut self, deleted_id: String) -> Result<(), String> {
        let initial_len = self.prompts.len();
        // 必须要记录长度，没删掉也不会有任何错误提示
        self.prompts.retain(|p| p.id != deleted_id);

        // 少一个代表有删了
        if self.prompts.len() < initial_len {
            self.save()?;
            Ok(())
        } else {
            Err("未找到要删除项的 ID".to_string())
        }
    }

    pub fn set_dark_mode(&mut self, new_mode: bool) -> Result<(), String> {
        self.is_in_dark_mode = new_mode;
        self.save()
    }

    pub fn get_dark_mode(&self) -> bool {
        self.is_in_dark_mode
    }

    pub fn get_data_file_path(&self) -> &PathBuf {
        &self.data_file_path
    }

    fn save(&self) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self).map_err(|e| e.to_string())?;

        write(&self.data_file_path, json).map_err(|e| e.to_string())?;

        Ok(())
    }
}
