use std::path::PathBuf;

use crate::error::YnabError;

/// Configuration file at `~/.config/ynab/config.json`.
///
/// Currently stores `default_plan_id` so commands can omit `--plan-id`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_plan_id: Option<String>,
}

impl Config {
    /// Load config from disk, returning default if file doesn't exist.
    pub fn load() -> Result<Self, YnabError> {
        let path = config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)
            .map_err(|e| YnabError::Config(format!("Failed to read config: {e}")))?;
        serde_json::from_str(&content).map_err(YnabError::from)
    }

    /// Save config to disk.
    pub fn save(&self) -> Result<(), YnabError> {
        let path = config_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| YnabError::Config(format!("Failed to create config dir: {e}")))?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, content)
            .map_err(|e| YnabError::Config(format!("Failed to write config: {e}")))?;
        Ok(())
    }

    /// Get the default plan ID, if set.
    pub fn default_plan_id(&self) -> Option<&str> {
        self.default_plan_id.as_deref()
    }

    /// Set the default plan ID.
    pub fn set_default_plan_id(&mut self, plan_id: &str) {
        self.default_plan_id = Some(plan_id.to_string());
    }
}

fn config_path() -> Result<PathBuf, YnabError> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| YnabError::Config("Could not determine config directory".into()))?;
    Ok(config_dir.join("ynab").join("config.json"))
}
