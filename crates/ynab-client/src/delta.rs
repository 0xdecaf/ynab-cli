use std::collections::HashMap;
use std::path::PathBuf;

use crate::error::YnabError;

/// Persistent cache for YNAB server_knowledge values (delta sync).
///
/// Stores the last known server_knowledge for each plan+resource combo
/// so subsequent requests can fetch only changes.
pub struct DeltaCache {
    path: PathBuf,
    data: HashMap<String, i64>,
}

impl DeltaCache {
    pub fn load() -> Result<Self, YnabError> {
        let path = cache_path()?;
        let data = if path.exists() {
            let content = std::fs::read_to_string(&path)
                .map_err(|e| YnabError::Config(format!("Failed to read delta cache: {e}")))?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            HashMap::new()
        };
        Ok(Self { path, data })
    }

    /// Get the last server_knowledge for a given plan+resource key.
    pub fn get(&self, plan_id: &str, resource: &str) -> Option<i64> {
        let key = format!("{plan_id}:{resource}");
        self.data.get(&key).copied()
    }

    /// Update the server_knowledge for a given plan+resource key and persist.
    pub fn set(&mut self, plan_id: &str, resource: &str, knowledge: i64) -> Result<(), YnabError> {
        let key = format!("{plan_id}:{resource}");
        self.data.insert(key, knowledge);
        self.save()
    }

    fn save(&self) -> Result<(), YnabError> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| YnabError::Config(format!("Failed to create cache dir: {e}")))?;
        }
        let content = serde_json::to_string_pretty(&self.data)?;
        std::fs::write(&self.path, content)
            .map_err(|e| YnabError::Config(format!("Failed to write delta cache: {e}")))?;
        Ok(())
    }
}

fn cache_path() -> Result<PathBuf, YnabError> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| YnabError::Config("Could not determine config directory".into()))?;
    Ok(config_dir.join("ynab").join("delta_cache.json"))
}
