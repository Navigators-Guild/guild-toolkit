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

    /// Initializes the ~/.guild/ directory structure and default config template.
    ///
    /// This will:
    /// 1. Create `~/.guild/` and `~/.guild/data/` if they do not exist.
    /// 2. Write a default config template to `~/.guild/config.toml` if it does not exist.
    ///
    /// This operation is idempotent and will not overwrite an existing configuration.
    pub fn init() -> Result<(), GuildError> {
        let guild_dir = Self::guild_dir();
        let data_dir = guild_dir.join("data");

        // Create directory structure
        std::fs::create_dir_all(&data_dir)?;

        // Initialize default config if it doesn't exist
        let config_path = Self::default_path();
        if !config_path.exists() {
            let default_toml = r#"[user]
name = "Apprentice"
handle = "apprentice"
"#;
            std::fs::write(&config_path, default_toml)?;
        }

        Ok(())
    }
}

fn dirs_or_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn mock_home(dir: &std::path::Path) {
        unsafe {
            std::env::set_var("HOME", dir);
        }
    }

    fn restore_home(original: Option<String>) {
        if let Some(home) = original {
            unsafe {
                std::env::set_var("HOME", home);
            }
        } else {
            unsafe {
                std::env::remove_var("HOME");
            }
        }
    }
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_load_from_valid_config() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("config.toml");
        let mut file = File::create(&path).unwrap();
        writeln!(file, "[user]\nname = \"Alice\"\nhandle = \"alice\"").unwrap();

        let config = GuildConfig::load_from(&path).unwrap();
        assert_eq!(config.user.name, "Alice");
        assert_eq!(config.user.handle, "alice");
    }

    #[test]
    fn test_load_from_missing_file_returns_config_not_found() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nonexistent.toml");

        let result = GuildConfig::load_from(&path);
        assert!(result.is_err());
        match result.unwrap_err() {
            GuildError::ConfigNotFound { path: err_path } => {
                assert_eq!(err_path, path);
            }
            other => panic!("expected ConfigNotFound, got: {:?}", other),
        }
    }

    #[test]
    fn test_init_creates_directories_and_default_config() {
        let _guard = crate::TEST_ENV_LOCK.lock().unwrap();
        let original = std::env::var("HOME").ok();
        let dir = tempdir().unwrap();
        mock_home(dir.path());

        // Prior to init, the files/directories shouldn't exist
        let guild_dir = GuildConfig::guild_dir();
        let config_path = GuildConfig::default_path();
        assert!(!guild_dir.exists());
        assert!(!config_path.exists());

        // Perform initialization
        GuildConfig::init().unwrap();

        // After init, everything should exist
        assert!(guild_dir.exists());
        assert!(guild_dir.join("data").exists());
        assert!(config_path.exists());

        // We can load it and check defaults
        let loaded = GuildConfig::load().unwrap();
        assert_eq!(loaded.user.name, "Apprentice");
        assert_eq!(loaded.user.handle, "apprentice");

        restore_home(original);
    }

    #[test]
    fn test_init_is_idempotent() {
        let _guard = crate::TEST_ENV_LOCK.lock().unwrap();
        let original = std::env::var("HOME").ok();
        let dir = tempdir().unwrap();
        mock_home(dir.path());

        // Perform initialization once
        GuildConfig::init().unwrap();

        // Mutate the configuration file
        let config_path = GuildConfig::default_path();
        let custom_toml = r#"[user]
name = "Bob"
handle = "bob"
"#;
        std::fs::write(&config_path, custom_toml).unwrap();

        // Perform initialization again
        GuildConfig::init().unwrap();

        // Verify config was not overwritten
        let loaded = GuildConfig::load().unwrap();
        assert_eq!(loaded.user.name, "Bob");
        assert_eq!(loaded.user.handle, "bob");

        restore_home(original);
    }

    #[test]
    fn test_init_fails_if_path_is_blocked_by_file() {
        let _guard = crate::TEST_ENV_LOCK.lock().unwrap();
        let original = std::env::var("HOME").ok();
        let dir = tempdir().unwrap();
        mock_home(dir.path());

        // Pre-create a regular file at ~/.guild (which blocks directory creation)
        let guild_dir = GuildConfig::guild_dir();
        std::fs::write(&guild_dir, "this is a file, not a directory").unwrap();

        // Initialization should fail because create_dir_all fails on a file
        let result = GuildConfig::init();
        assert!(result.is_err());
        match result.unwrap_err() {
            GuildError::Io(_) => {}
            other => panic!("expected GuildError::Io, got: {:?}", other),
        }

        restore_home(original);
    }

    #[test]
    fn test_load_from_invalid_toml_returns_parse_error() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("invalid.toml");
        let mut file = File::create(&path).unwrap();
        writeln!(file, "not valid toml formatting").unwrap();

        let result = GuildConfig::load_from(&path);
        assert!(result.is_err());
        match result.unwrap_err() {
            GuildError::ConfigParse(_) => {}
            other => panic!("expected ConfigParse, got: {:?}", other),
        }
    }

    #[test]
    fn test_load_from_valid_toml_but_incorrect_schema_returns_parse_error() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("wrong_schema.toml");
        let mut file = File::create(&path).unwrap();
        writeln!(file, "some_unrelated_setting = 42").unwrap();

        let result = GuildConfig::load_from(&path);
        assert!(result.is_err());
        match result.unwrap_err() {
            GuildError::ConfigParse(_) => {}
            other => panic!("expected ConfigParse, got: {:?}", other),
        }
    }

    #[test]
    fn test_guild_dir_resolves_with_home_env() {
        let _guard = crate::TEST_ENV_LOCK.lock().unwrap();
        let original_home = std::env::var("HOME").ok();

        unsafe {
            std::env::set_var("HOME", "/tmp/mock_home");
        }
        assert_eq!(
            GuildConfig::guild_dir(),
            PathBuf::from("/tmp/mock_home/.guild")
        );
        assert_eq!(
            GuildConfig::default_path(),
            PathBuf::from("/tmp/mock_home/.guild/config.toml")
        );

        if let Some(home) = original_home {
            unsafe {
                std::env::set_var("HOME", home);
            }
        } else {
            unsafe {
                std::env::remove_var("HOME");
            }
        }
    }

    #[test]
    fn test_load_from_default_path() {
        let _guard = crate::TEST_ENV_LOCK.lock().unwrap();
        let original_home = std::env::var("HOME").ok();
        let dir = tempdir().unwrap();
        let mock_home = dir.path();

        unsafe {
            std::env::set_var("HOME", mock_home);
        }

        let guild_dir = mock_home.join(".guild");
        std::fs::create_dir_all(&guild_dir).unwrap();
        let path = guild_dir.join("config.toml");
        let mut file = File::create(&path).unwrap();
        writeln!(file, "[user]\nname = \"Bob\"\nhandle = \"bob\"").unwrap();

        let config = GuildConfig::load().unwrap();
        assert_eq!(config.user.name, "Bob");
        assert_eq!(config.user.handle, "bob");

        if let Some(home) = original_home {
            unsafe {
                std::env::set_var("HOME", home);
            }
        } else {
            unsafe {
                std::env::remove_var("HOME");
            }
        }
    }
}
