use ynab_types::ErrorDetail;

#[derive(Debug, thiserror::Error)]
pub enum YnabError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error ({status}): [{id}] {name} - {detail}")]
    Api {
        status: u16,
        id: String,
        name: String,
        detail: String,
    },

    #[error("Not authenticated. Run `ynab auth login` first.")]
    NotAuthenticated,

    #[error("Rate limited (200 requests/hour). Retry after {retry_after_secs}s.")]
    RateLimited { retry_after_secs: u64 },

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("{0}")]
    Other(String),
}

impl YnabError {
    pub fn from_api_error(status: u16, error: ErrorDetail) -> Self {
        if status == 429 {
            return Self::RateLimited {
                retry_after_secs: 60,
            };
        }
        Self::Api {
            status,
            id: error.id,
            name: error.name,
            detail: error.detail,
        }
    }
}
