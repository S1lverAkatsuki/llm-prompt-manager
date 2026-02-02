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
    fn with_init(init_prompt: Prompt) -> Self {
        Self {
            id: ShortUuid::generate().to_string(),
            title: init_prompt.title,
            tip: init_prompt.tip,
            content: init_prompt.content,
            tags: init_prompt.tags,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PromptManager {
    loaded: bool,
    pub data_file_path: PathBuf,
    pub prompts: Vec<Prompt>,
}

const DATA_FILE: &str = "prompts.json";

#[allow(unused)]
impl PromptManager {
    pub fn empty(app: &tauri::AppHandle) -> Self {
        use tauri::Manager;

        let data_file_path = app
            .path()
            .resolve(DATA_FILE, BaseDirectory::AppLocalData)
            .expect("路径解析失败");

        Self {
            loaded: false,
            data_file_path,
            prompts: Vec::new(),
        }
    }

    pub fn create_prompt(&mut self, init_prompt: Prompt) -> Result<Vec<Prompt>, String> {
        if !self.loaded {
            self.loaded_prompt();
        }

        let new_prompt = Prompt::with_init(init_prompt);
        self.prompts.push(new_prompt);

        self.save()?;
        Ok(self.prompts.clone())
    }

    pub fn read_prompts(&mut self) -> Result<Vec<Prompt>, String> {
        if !self.loaded {
            self.loaded_prompt();
        }

        Ok(self.prompts.clone())
    }

    pub fn update_prompt(&mut self, new_prompt: Prompt) -> Result<(), String> {
        if !self.loaded {
            self.loaded_prompt();
        }

        if let Some(index) = self.prompts.iter().position(|p| p.id == new_prompt.id) {
            self.prompts[index] = new_prompt;
            self.save()?;
            Ok(())
        } else {
            Err("未找到对应项的 ID".to_string())
        }
    }

    pub fn delete_prompt(&mut self, deleted_id: String) -> Result<(), String> {
        if !self.loaded {
            self.loaded_prompt();
        }

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

    fn save(&self) -> Result<(), String> {
        let json = serde_json::to_string(&self.prompts).map_err(|e| e.to_string())?;

        write(&self.data_file_path, json).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn loaded_prompt(&mut self) {
        if let Some(parent) = self.data_file_path.parent() {
            if !parent.exists() {
                create_dir_all(parent).expect("目录创建失败");
            }
        }

        self.prompts = if self.data_file_path.exists() {
            let content = read_to_string(&self.data_file_path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        };
        self.loaded = true;
    }
}
