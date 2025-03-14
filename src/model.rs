use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tabled::Tabled;
use dirs;
#[derive(Debug, Serialize, Deserialize, Clone, Tabled)]
pub struct Task {
    #[tabled(rename = "ID")]
    pub id: usize,
    
    #[tabled(rename = "Task")]
    pub name: String,
    
    #[tabled(rename = "Priority")]
    pub priority: String,
    
    #[tabled(rename = "Status")]
    pub status: String,
    
    #[tabled(rename = "Due Date")]
    #[tabled(display_with = "display_option_string")]
    pub due_date: Option<String>,
    
    #[tabled(display_with = "display_vec_string")]
    pub tags: Vec<String>,
    
    #[tabled(rename = "Created")]
    pub created_at: String,
    
    #[tabled(skip)]
    pub completed_at: Option<String>,
}
fn display_option_string(opt: &Option<String>) -> String {
    match opt {
        Some(s) => s.clone(),
        None => "-".to_string(),
    }
}
fn display_vec_string(vec: &Vec<String>) -> String {
    if vec.is_empty() {
        "-".to_string()
    } else {
        vec.join(", ")
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub tasks_dir: String,
    pub user_name: String,
    pub default_priority: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            tasks_dir: get_default_tasks_dir(),
            user_name: "User".to_string(),
            default_priority: "medium".to_string(),
        }
    }
}
pub fn get_default_tasks_dir() -> String {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".rtasks")
        .to_string_lossy()
        .to_string()
}
pub fn load_config() -> AppConfig {
    confy::load("rtask", "config").unwrap_or_default()
}
pub fn save_config(config: &AppConfig) -> Result<(), confy::ConfyError> {
    confy::store("rtask", "config", config)
}
pub fn is_config_exists() -> bool {
    confy::get_configuration_file_path("rtask", "config")
        .map(|path| path.exists())
        .unwrap_or(false)
}
pub fn get_tasks_file() -> PathBuf {
    let config = load_config();
    let path = Path::new(&config.tasks_dir).join("tasks.json");
    path
}