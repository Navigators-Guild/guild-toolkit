use crate::GuildError;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Top-level guild configuration, loaded from `~/.guild/config.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildConfig {
    pub user: UserConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub name: String,
    pub handle: String,
}

impl GuildConfig {
    /// Returns the default guild directory: `~/.guild/`
    pub fn guild_dir() -> PathBuf {
        dirs_or_home().join(".guild")
    }

    /// Returns the default config file path: `~/.guild/config.toml`
    pub fn default_path() -> PathBuf {
        Self::guild_dir().join("config.toml")
    }

    /// Load config from the default path.
    pub fn load() -> Result<Self, GuildError> {
        Self::load_from(&Self::default_path())
    }

    /// Load config from a specific path.
    pub fn load_from(path: &Path) -> Result<Self, GuildError> {
        let content = std::fs::read_to_string(path).map_err(|_| GuildError::ConfigNotFound {
            path: path.to_path_buf(),
        })?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
}

fn dirs_or_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}
