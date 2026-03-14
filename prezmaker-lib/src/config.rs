use crate::error::PrezError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::debug;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Config {
    #[serde(default)]
    pub tmdb: TmdbConfig,
    #[serde(default)]
    pub igdb: IgdbConfig,
    #[serde(default)]
    pub preferences: Preferences,
    #[serde(default)]
    pub llm: LlmConfig,
    #[serde(default)]
    pub modules: ModulesConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ModulesConfig {
    #[serde(default)]
    pub c411: C411ModuleConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct C411ModuleConfig {
    #[serde(default)]
    pub enabled: bool,
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct LlmConfig {
    pub provider: Option<String>,
    pub api_key: Option<String>,
    pub groq_api_key: Option<String>,
    pub mistral_api_key: Option<String>,
    pub gemini_api_key: Option<String>,
}

impl LlmConfig {
    /// Returns the API key for the currently selected provider.
    /// Checks provider-specific key first, falls back to generic `api_key`.
    pub fn resolve_api_key(&self) -> Option<&str> {
        let provider = self.provider.as_deref().unwrap_or("");
        let specific = match provider {
            "groq" => self.groq_api_key.as_deref(),
            "mistral" => self.mistral_api_key.as_deref(),
            "gemini" => self.gemini_api_key.as_deref(),
            _ => None,
        };
        specific
            .or(self.api_key.as_deref())
            .filter(|k| !k.is_empty())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TmdbConfig {
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct IgdbConfig {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Preferences {
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_title_color")]
    pub title_color: String,
    #[serde(default)]
    pub auto_clipboard: bool,
    #[serde(default = "default_pseudo")]
    pub pseudo: String,
    /// Default template name per content type (e.g. "film" -> "MonTemplate")
    #[serde(default)]
    pub default_templates: HashMap<String, String>,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            language: default_language(),
            title_color: default_title_color(),
            auto_clipboard: false,
            pseudo: default_pseudo(),
            default_templates: HashMap::new(),
        }
    }
}

fn default_language() -> String {
    "fr-FR".to_string()
}

fn default_title_color() -> String {
    "c0392b".to_string()
}

fn default_pseudo() -> String {
    String::new()
}

impl Config {
    pub fn load(config_path: Option<&str>) -> Result<Self, PrezError> {
        let path = if let Some(p) = config_path {
            PathBuf::from(p)
        } else {
            Self::default_path()
        };

        let mut config = if path.exists() {
            debug!("Loading config from: {}", path.display());
            let content =
                std::fs::read_to_string(&path).map_err(|e| PrezError::Config(e.to_string()))?;
            toml::from_str::<Config>(&content)
                .map_err(|e| PrezError::Config(format!("Invalid config: {}", e)))?
        } else {
            debug!("No config file found, using defaults");
            Config::default()
        };

        // Override with env vars
        config.apply_env_overrides();

        Ok(config)
    }

    pub fn default_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("prezmaker")
            .join("config.toml")
    }

    pub fn save(&self) -> Result<(), PrezError> {
        let path = Self::default_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| PrezError::Config(format!("Cannot create config dir: {}", e)))?;
        }
        let content = toml::to_string_pretty(self)
            .map_err(|e| PrezError::Config(format!("Cannot serialize config: {}", e)))?;
        std::fs::write(&path, content)
            .map_err(|e| PrezError::Config(format!("Cannot write config: {}", e)))?;
        debug!("Config saved to: {}", path.display());
        Ok(())
    }

    fn apply_env_overrides(&mut self) {
        if let Ok(key) = std::env::var("PREZMAKER_TMDB_API_KEY") {
            self.tmdb.api_key = Some(key);
        }
        if let Ok(id) = std::env::var("PREZMAKER_IGDB_CLIENT_ID") {
            self.igdb.client_id = Some(id);
        }
        if let Ok(secret) = std::env::var("PREZMAKER_IGDB_CLIENT_SECRET") {
            self.igdb.client_secret = Some(secret);
        }
    }

    pub fn tmdb_api_key(&self) -> Result<&str, PrezError> {
        self.tmdb
            .api_key
            .as_deref()
            .ok_or_else(|| {
                PrezError::MissingApiKey(
                    "TMDB API key not found. Set it in config.toml or PREZMAKER_TMDB_API_KEY env var".to_string(),
                )
            })
    }

    pub fn igdb_credentials(&self) -> Result<(&str, &str), PrezError> {
        let id = self.igdb.client_id.as_deref().ok_or_else(|| {
            PrezError::MissingApiKey(
                "IGDB client_id not found. Set it in config.toml or PREZMAKER_IGDB_CLIENT_ID env var".to_string(),
            )
        })?;
        let secret = self.igdb.client_secret.as_deref().ok_or_else(|| {
            PrezError::MissingApiKey(
                "IGDB client_secret not found. Set it in config.toml or PREZMAKER_IGDB_CLIENT_SECRET env var".to_string(),
            )
        })?;
        Ok((id, secret))
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.preferences.language, "fr-FR");
        assert_eq!(config.preferences.title_color, "c0392b");
        assert!(!config.preferences.auto_clipboard);
        assert!(config.tmdb.api_key.is_none());
    }

    #[test]
    fn test_parse_toml() {
        let toml_str = r#"
[tmdb]
api_key = "test_key"

[preferences]
language = "en-US"
title_color = "aa0000"
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.tmdb.api_key.unwrap(), "test_key");
        assert_eq!(config.preferences.language, "en-US");
        assert_eq!(config.preferences.title_color, "aa0000");
    }

    #[test]
    fn test_tmdb_api_key_missing() {
        let config = Config::default();
        assert!(config.tmdb_api_key().is_err());
    }

    #[test]
    fn test_tmdb_api_key_present() {
        let mut config = Config::default();
        config.tmdb.api_key = Some("my_key".to_string());
        assert_eq!(config.tmdb_api_key().unwrap(), "my_key");
    }
}
