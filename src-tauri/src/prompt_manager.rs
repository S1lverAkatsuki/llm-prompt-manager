use std::collections::{HashMap, HashSet};
use std::fs::{create_dir_all, read_to_string, write};
use std::path::PathBuf;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use short_uuid::ShortUuid;

use tauri::path::BaseDirectory;

#[derive(Serialize, Deserialize, Clone)]
pub struct ModelConfig {
    pub api_key: String,
    pub provider: String,
    pub model: String,
    pub base_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Prompt {
    pub id: String,
    pub title: String,
    pub tip: String,
    pub content: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, )]
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
            title: String::new(),
            tip: String::new(),
            content: String::new(),
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
    is_ai_enabled: bool,
    model_config: Option<ModelConfig>,
    #[serde(skip)]
    data_file_path: PathBuf,
    pub prompts: Vec<Prompt>,
    tags: Vec<String>,
    #[serde(skip)]
    pub reqwest_client: Option<Client>,
}

impl Default for PromptManager {
    fn default() -> Self {
        Self {
            is_in_dark_mode: false,
            is_ai_enabled: false,
            model_config: None,
            data_file_path: PathBuf::new(),
            prompts: Vec::new(),
            tags: Vec::new(),
            reqwest_client: None,
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

        let mut app_config = if data_file_path.exists() {
            let content = read_to_string(&data_file_path).unwrap_or_default();

            if content.trim().is_empty() {
                PromptManager::default()
            } else {
                serde_json::from_str::<PromptManager>(&content).unwrap_or_default()
            }
        } else {
            PromptManager::default()
        };

        if app_config.is_ai_enabled {
            app_config.reqwest_client = Some(Client::new())
        }

        Self {
            data_file_path,
            ..app_config
        }
    }

    pub fn create_prompt(&mut self, data: PromptData) -> Result<Prompt, String> {
        let new_prompt = Prompt::from_data(data);
        self.prompts.push(new_prompt.clone());
        self.rebuild_tags();

        self.save()?;
        Ok(new_prompt)
    }

    pub fn read_prompts(&mut self) -> Result<Vec<Prompt>, String> {
        Ok(self.prompts.clone())
    }

    pub fn update_prompt(&mut self, new_prompt: Prompt) -> Result<(), String> {
        if let Some(index) = self.prompts.iter().position(|p| p.id == new_prompt.id) {
            self.prompts[index] = new_prompt;
            self.rebuild_tags();
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
            self.rebuild_tags();
            self.save()?;
            Ok(())
        } else {
            Err("未找到要删除项的 ID".to_string())
        }
    }

    pub fn reorder_prompts(&mut self, ordered_ids: Vec<String>) -> Result<(), String> {
        let mut map: HashMap<String, Prompt> = HashMap::new();
        for p in self.prompts.drain(..) {
            map.insert(p.id.clone(), p);
        }

        let len = map.len();
        let reordered: Vec<Prompt> = ordered_ids.iter().filter_map(|id| map.remove(id)).collect();

        if reordered.len() != len {
            return Err("ordered_ids 与已有项不匹配".to_string());
        }

        self.prompts = reordered;
        self.save()
    }

    pub fn set_dark_mode(&mut self, new_mode: bool) -> Result<(), String> {
        self.is_in_dark_mode = new_mode;
        self.save()
    }

    pub fn get_dark_mode(&self) -> bool {
        self.is_in_dark_mode
    }

    pub fn get_ai_enabled(&self) -> bool {
        self.is_ai_enabled
    }

    pub fn set_ai_enabled(&mut self, enabled: bool) -> Result<(), String> {
        self.is_ai_enabled = enabled;
        self.reqwest_client = if enabled { Some(Client::new()) } else { None };
        self.save()
    }

    pub fn get_model_config(&self) -> Option<ModelConfig> {
        self.model_config.clone()
    }

    pub fn set_model_config(&mut self, config: ModelConfig) -> Result<(), String> {
        self.model_config = Some(config);
        self.reqwest_client = Some(Client::new());
        self.save()
    }

    pub fn clear_ai_config(&mut self) -> Result<(), String> {
        self.is_ai_enabled = false;
        self.model_config = None;
        self.reqwest_client = None;
        self.save()
    }

    pub fn get_data_file_path(&self) -> &PathBuf {
        &self.data_file_path
    }

    fn rebuild_tags(&mut self) {
        let mut tag_set = HashSet::new();
        for prompt in &self.prompts {
            for tag in &prompt.tags {
                tag_set.insert(tag.clone());
            }
        }
        let mut v: Vec<String> = tag_set.into_iter().collect();
        v.sort();
        self.tags = v;
    }

    pub fn get_all_tags(&self) -> Vec<String> {
        self.tags.clone()
    }

    pub fn add_tag(&mut self, tag: String) -> Result<(), String> {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.save()?;
        }
        Ok(())
    }

    pub fn remove_tag(&mut self, tag: String) -> Result<(), String> {
        self.tags.retain(|t| t != &tag);
        self.save()
    }

    fn save(&self) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self).map_err(|e| e.to_string())?;

        write(&self.data_file_path, json).map_err(|e| e.to_string())?;

        Ok(())
    }
}
