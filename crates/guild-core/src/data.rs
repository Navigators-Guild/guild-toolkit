use crate::{GuildError, config::GuildConfig};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

/// Apprentice profile — stored in `~/.guild/data/profile.toml`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub handle: String,
    pub joined: String,
}

impl Profile {
    /// Returns the canonical path for the profile file.
    fn path() -> PathBuf {
        GuildConfig::guild_dir().join("data").join("profile.toml")
    }

    /// Load the profile from `~/.guild/data/profile.toml`.
    pub fn load() -> Result<Self, GuildError> {
        let path = Self::path();
        let content =
            fs::read_to_string(&path).map_err(|_| GuildError::DataError { path: path.clone() })?;
        let profile: Self = toml::from_str(&content)?;
        Ok(profile)
    }

    /// Save the profile to `~/.guild/data/profile.toml`.
    pub fn save(&self) -> Result<(), GuildError> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content =
            toml::to_string(self).map_err(|e| GuildError::SerializeError(e.to_string()))?;
        fs::write(&path, content)?;
        Ok(())
    }
}

/// Project registry entry — stored in `~/.guild/data/projects.toml`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRegistry {
    pub projects: Vec<Project>,
}

impl ProjectRegistry {
    fn path() -> PathBuf {
        GuildConfig::guild_dir().join("data").join("projects.toml")
    }

    /// Load the project registry from `~/.guild/data/projects.toml`.
    /// Returns an empty registry if the file does not exist.
    pub fn load() -> Result<Self, GuildError> {
        let path = Self::path();
        if !path.exists() {
            return Ok(Self {
                projects: Vec::new(),
            });
        }
        let content =
            fs::read_to_string(&path).map_err(|_| GuildError::DataError { path: path.clone() })?;
        let registry: Self = toml::from_str(&content)?;
        Ok(registry)
    }

    /// Save the project registry to `~/.guild/data/projects.toml`.
    pub fn save(&self) -> Result<(), GuildError> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content =
            toml::to_string(self).map_err(|e| GuildError::SerializeError(e.to_string()))?;
        fs::write(&path, content)?;
        Ok(())
    }

    /// Add a project to the registry. Returns an error if a project with the same name already exists (case-insensitive).
    /// Trims names and paths, validating character sets.
    pub fn add_project(&mut self, mut project: Project) -> Result<(), GuildError> {
        let trimmed_name = project.name.trim().to_string();
        if trimmed_name.is_empty() {
            return Err(GuildError::InvalidProjectProperties(
                "project name cannot be empty".to_string(),
            ));
        }

        // Enforce safe characters for name (alphanumeric, spaces, hyphens, underscores, dots)
        if !trimmed_name
            .chars()
            .all(|c| c.is_alphanumeric() || c == ' ' || c == '-' || c == '_' || c == '.')
        {
            return Err(GuildError::InvalidProjectProperties(
                "project name can only contain alphanumeric characters, spaces, hyphens, underscores, and dots".to_string(),
            ));
        }

        let trimmed_path = project.path.trim().to_string();
        if trimmed_path.is_empty() {
            return Err(GuildError::InvalidProjectProperties(
                "project path cannot be empty".to_string(),
            ));
        }

        // Path cannot contain null bytes
        if trimmed_path.contains('\0') {
            return Err(GuildError::InvalidProjectProperties(
                "project path cannot contain null bytes".to_string(),
            ));
        }

        let name_lower = trimmed_name.to_lowercase();
        if self
            .projects
            .iter()
            .any(|p| p.name.trim().to_lowercase() == name_lower)
        {
            return Err(GuildError::DuplicateProject(trimmed_name));
        }

        project.name = trimmed_name;
        project.path = trimmed_path;
        self.projects.push(project);
        Ok(())
    }

    /// Look up a project by name (case-insensitive, trimmed).
    pub fn find_project(&self, name: &str) -> Option<&Project> {
        let name_lower = name.trim().to_lowercase();
        self.projects
            .iter()
            .find(|p| p.name.trim().to_lowercase() == name_lower)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: String,
    pub status: ProjectStatus,
    pub difficulty: Difficulty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    NotStarted,
    InProgress,
    UnderReview,
    Complete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

/// Curriculum progress — stored in `~/.guild/data/progress.toml`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress {
    pub checkpoints: Vec<Checkpoint>,
}

impl Progress {
    fn path() -> PathBuf {
        GuildConfig::guild_dir().join("data").join("progress.toml")
    }

    /// Load the progress registry from `~/.guild/data/progress.toml`.
    /// Returns an empty progress structure if the file does not exist.
    pub fn load() -> Result<Self, GuildError> {
        let path = Self::path();
        if !path.exists() {
            return Ok(Self {
                checkpoints: Vec::new(),
            });
        }
        let content =
            fs::read_to_string(&path).map_err(|_| GuildError::DataError { path: path.clone() })?;
        let progress: Self = toml::from_str(&content)?;
        Ok(progress)
    }

    /// Save the progress to `~/.guild/data/progress.toml`.
    pub fn save(&self) -> Result<(), GuildError> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content =
            toml::to_string(self).map_err(|e| GuildError::SerializeError(e.to_string()))?;
        fs::write(&path, content)?;
        Ok(())
    }

    /// Mark a checkpoint as completed (case-insensitive).
    /// Returns an error if the checkpoint ID is not found.
    pub fn complete_checkpoint(&mut self, id: &str) -> Result<(), GuildError> {
        let id_lower = id.trim().to_lowercase();
        if let Some(checkpoint) = self
            .checkpoints
            .iter_mut()
            .find(|c| c.id.trim().to_lowercase() == id_lower)
        {
            checkpoint.completed = true;
            Ok(())
        } else {
            Err(GuildError::CheckpointNotFound(id.to_string()))
        }
    }

    /// Return the first incomplete checkpoint in the curriculum sequence.
    pub fn next_incomplete(&self) -> Option<&Checkpoint> {
        self.checkpoints.iter().find(|c| !c.completed)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

/// Review history — stored in `~/.guild/data/reviews.toml`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewHistory {
    pub reviews: Vec<ReviewRound>,
}

impl ReviewHistory {
    fn path() -> PathBuf {
        GuildConfig::guild_dir().join("data").join("reviews.toml")
    }

    /// Load the review history from `~/.guild/data/reviews.toml`.
    /// Returns an empty review history if the file does not exist.
    pub fn load() -> Result<Self, GuildError> {
        let path = Self::path();
        if !path.exists() {
            return Ok(Self {
                reviews: Vec::new(),
            });
        }
        let content =
            fs::read_to_string(&path).map_err(|_| GuildError::DataError { path: path.clone() })?;
        let history: Self = toml::from_str(&content)?;
        Ok(history)
    }

    /// Save the review history to `~/.guild/data/reviews.toml`.
    pub fn save(&self) -> Result<(), GuildError> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content =
            toml::to_string(self).map_err(|e| GuildError::SerializeError(e.to_string()))?;
        fs::write(&path, content)?;
        Ok(())
    }

    /// Add a review round to the history.
    /// Returns an error if the round number is 0, the project name is empty,
    /// or if a round with the same number already exists for the project.
    pub fn add_round(&mut self, mut round: ReviewRound) -> Result<(), GuildError> {
        let trimmed_project = round.project.trim().to_string();
        if trimmed_project.is_empty() {
            return Err(GuildError::InvalidReviewRound(
                "project name cannot be empty".to_string(),
            ));
        }

        if round.round == 0 {
            return Err(GuildError::InvalidReviewRound(
                "review round number must be greater than 0".to_string(),
            ));
        }

        let project_lower = trimmed_project.to_lowercase();
        if self
            .reviews
            .iter()
            .any(|r| r.project.trim().to_lowercase() == project_lower && r.round == round.round)
        {
            return Err(GuildError::DuplicateReviewRound(
                trimmed_project,
                round.round,
            ));
        }

        round.project = trimmed_project;
        self.reviews.push(round);
        Ok(())
    }

    /// Retrieve the latest review round for a project (case-insensitive).
    pub fn latest_round(&self, project: &str) -> Option<&ReviewRound> {
        let project_lower = project.trim().to_lowercase();
        self.reviews
            .iter()
            .filter(|r| r.project.trim().to_lowercase() == project_lower)
            .max_by_key(|r| r.round)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewRound {
    pub project: String,
    pub round: u32,
    pub status: ReviewStatus,
    pub feedback_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReviewStatus {
    Submitted,
    InReview,
    Returned,
    Accepted,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use tempfile::tempdir;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

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

    fn sample_profile() -> Profile {
        Profile {
            name: "Alice".to_string(),
            handle: "alice".to_string(),
            joined: "2025-01-01".to_string(),
    fn sample_project(name: &str, path: &str) -> Project {
        Project {
            name: name.to_string(),
            path: path.to_string(),
            status: ProjectStatus::NotStarted,
            difficulty: Difficulty::Beginner,
        }
    }

    fn sample_round(project: &str, round: u32) -> ReviewRound {
        ReviewRound {
            project: project.to_string(),
            round,
            status: ReviewStatus::Submitted,
            feedback_ref: None,
        }
    }

    fn sample_checkpoint(id: &str, title: &str, completed: bool) -> Checkpoint {
        Checkpoint {
            id: id.to_string(),
            title: title.to_string(),
            completed,
        }
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let _guard = ENV_LOCK.lock().unwrap();
    fn test_load_missing_returns_empty_reviews() {
    fn test_load_missing_returns_empty_progress() {
        let _guard = crate::TEST_ENV_LOCK.lock().unwrap();
    fn test_load_missing_returns_empty_registry() {
        let _guard1 = ENV_LOCK.lock().unwrap();
        let _guard2 = crate::TEST_ENV_LOCK.lock().unwrap();
        let original = std::env::var("HOME").ok();
        let dir = tempdir().unwrap();
        mock_home(dir.path());

        let profile = sample_profile();
        profile.save().unwrap();

        let loaded = Profile::load().unwrap();
        assert_eq!(loaded.name, "Alice");
        assert_eq!(loaded.handle, "alice");
        assert_eq!(loaded.joined, "2025-01-01");
        let registry = ProjectRegistry::load().unwrap();
        assert!(registry.projects.is_empty());
        let history = ReviewHistory::load().unwrap();
        assert!(history.reviews.is_empty());
        let progress = Progress::load().unwrap();
        assert!(progress.checkpoints.is_empty());

        restore_home(original);
    }

    #[test]
    fn test_load_missing_file_returns_data_error() {
        let _guard = ENV_LOCK.lock().unwrap();
        let original = std::env::var("HOME").ok();
        let dir = tempdir().unwrap();
        mock_home(dir.path());
        // Do NOT create the data directory or file

        let result = Profile::load();
        assert!(result.is_err());
        match result.unwrap_err() {
            GuildError::DataError { .. } => {}
            other => panic!("expected DataError, got: {:?}", other),
        }
    fn test_save_and_load_roundtrip() {
        let _guard = ENV_LOCK.lock().unwrap();
        let _guard = crate::TEST_ENV_LOCK.lock().unwrap();
        let original = std::env::var("HOME").ok();
        let dir = tempdir().unwrap();
        mock_home(dir.path());

        let mut registry = ProjectRegistry::load().unwrap();
        registry
            .add_project(sample_project("Project-A", "./a"))
            .unwrap();
        registry
            .add_project(sample_project("Project-B", "./b"))
            .unwrap();
        registry.save().unwrap();

        let loaded = ProjectRegistry::load().unwrap();
        assert_eq!(loaded.projects.len(), 2);

        let p_a = loaded.find_project("project-a").unwrap();
        assert_eq!(p_a.name, "Project-A");
        assert_eq!(p_a.path, "./a");

        let p_b = loaded.find_project("PROJECT-B").unwrap();
        assert_eq!(p_b.name, "Project-B");
        assert_eq!(p_b.path, "./b");
        let mut history = ReviewHistory::load().unwrap();
        history.add_round(sample_round("Project-A", 1)).unwrap();
        history.add_round(sample_round("Project-B", 2)).unwrap();
        history.save().unwrap();

        let loaded = ReviewHistory::load().unwrap();
        assert_eq!(loaded.reviews.len(), 2);
        assert_eq!(loaded.reviews[0].project, "Project-A");
        assert_eq!(loaded.reviews[0].round, 1);
        assert_eq!(loaded.reviews[1].project, "Project-B");
        assert_eq!(loaded.reviews[1].round, 2);
        let mut progress = Progress::load().unwrap();
        progress
            .checkpoints
            .push(sample_checkpoint("id-1", "Title 1", false));
        progress
            .checkpoints
            .push(sample_checkpoint("id-2", "Title 2", true));
        progress.save().unwrap();

        let loaded = Progress::load().unwrap();
        assert_eq!(loaded.checkpoints.len(), 2);
        assert_eq!(loaded.checkpoints[0].id, "id-1");
        assert_eq!(loaded.checkpoints[0].title, "Title 1");
        assert!(!loaded.checkpoints[0].completed);
        assert!(loaded.checkpoints[1].completed);

        restore_home(original);
    }

    #[test]
    fn test_load_invalid_toml_returns_parse_error() {
        let _guard = ENV_LOCK.lock().unwrap();
        let original = std::env::var("HOME").ok();
        let dir = tempdir().unwrap();
        mock_home(dir.path());

        let data_dir = dir.path().join(".guild").join("data");
        std::fs::create_dir_all(&data_dir).unwrap();
        let path = data_dir.join("profile.toml");
        let mut file = File::create(&path).unwrap();
        writeln!(file, "not valid toml !!! @@@").unwrap();

        let result = Profile::load();
        assert!(result.is_err());
        match result.unwrap_err() {
            GuildError::ConfigParse(_) => {}
            other => panic!("expected ConfigParse, got: {:?}", other),
        }

        restore_home(original);
    }

    #[test]
    fn test_load_wrong_schema_returns_parse_error() {
        let _guard = ENV_LOCK.lock().unwrap();
        let original = std::env::var("HOME").ok();
        let dir = tempdir().unwrap();
        mock_home(dir.path());

        let data_dir = dir.path().join(".guild").join("data");
        std::fs::create_dir_all(&data_dir).unwrap();
        let path = data_dir.join("profile.toml");
        let mut file = File::create(&path).unwrap();
        writeln!(file, "some_unrelated_key = \"hello\"").unwrap();

        let result = Profile::load();
        assert!(result.is_err());
        match result.unwrap_err() {
            GuildError::ConfigParse(_) => {}
            other => panic!("expected ConfigParse, got: {:?}", other),
        }

        restore_home(original);
    }

    #[test]
    fn test_save_creates_data_directory() {
        let _guard = ENV_LOCK.lock().unwrap();
        let original = std::env::var("HOME").ok();
        let dir = tempdir().unwrap();
        mock_home(dir.path());
        // Deliberately do NOT pre-create ~/.guild/data/

        let profile = sample_profile();
        profile.save().unwrap();

        let expected_path = dir.path().join(".guild").join("data").join("profile.toml");
        assert!(
            expected_path.exists(),
            "profile.toml should have been created"
        );

        restore_home(original);
    fn test_add_project_success_and_trimming() {
        let mut registry = ProjectRegistry { projects: vec![] };

        // Trims name and path
        registry
            .add_project(sample_project("  My-Project.1  ", "  /some/path  "))
            .unwrap();

        assert_eq!(registry.projects.len(), 1);
        assert_eq!(registry.projects[0].name, "My-Project.1");
        assert_eq!(registry.projects[0].path, "/some/path");
    }

    #[test]
    fn test_add_project_duplicate_fails_case_insensitively() {
        let mut registry = ProjectRegistry { projects: vec![] };
        registry
            .add_project(sample_project("My-Project", "/path1"))
            .unwrap();

        // Exact duplicate
        let res1 = registry.add_project(sample_project("My-Project", "/path2"));
        assert!(res1.is_err());
        match res1.unwrap_err() {
            GuildError::DuplicateProject(name) => assert_eq!(name, "My-Project"),
            other => panic!("expected DuplicateProject, got: {:?}", other),
        }

        // Case-insensitive duplicate
        let res2 = registry.add_project(sample_project("my-project", "/path3"));
        assert!(res2.is_err());
        match res2.unwrap_err() {
            GuildError::DuplicateProject(name) => assert_eq!(name, "my-project"),
            other => panic!("expected DuplicateProject, got: {:?}", other),
        }
    }

    #[test]
    fn test_add_project_invalid_properties() {
        let mut registry = ProjectRegistry { projects: vec![] };

        // Empty name
        let res_empty_name = registry.add_project(sample_project("   ", "/path"));
        assert!(res_empty_name.is_err());
        match res_empty_name.unwrap_err() {
            GuildError::InvalidProjectProperties(msg) => assert!(msg.contains("name")),
            other => panic!("expected InvalidProjectProperties, got: {:?}", other),
        }

        // Empty path
        let res_empty_path = registry.add_project(sample_project("Project-A", "   "));
        assert!(res_empty_path.is_err());
        match res_empty_path.unwrap_err() {
            GuildError::InvalidProjectProperties(msg) => assert!(msg.contains("path")),
            other => panic!("expected InvalidProjectProperties, got: {:?}", other),
        }

        // Invalid characters in name (slash /)
        let res_invalid_char = registry.add_project(sample_project("Project/A", "/path"));
        assert!(res_invalid_char.is_err());
        match res_invalid_char.unwrap_err() {
            GuildError::InvalidProjectProperties(msg) => assert!(msg.contains("alphanumeric")),
            other => panic!("expected InvalidProjectProperties, got: {:?}", other),
        }

        // Path containing null bytes
        let res_null_bytes =
            registry.add_project(sample_project("Project-A", "/path\0with\0nulls"));
        assert!(res_null_bytes.is_err());
        match res_null_bytes.unwrap_err() {
            GuildError::InvalidProjectProperties(msg) => assert!(msg.contains("null")),
            other => panic!("expected InvalidProjectProperties, got: {:?}", other),
        }
    }

    #[test]
    fn test_find_project() {
        let mut registry = ProjectRegistry { projects: vec![] };
        registry
            .add_project(sample_project("Project-A", "/path-a"))
            .unwrap();

        // Exact match
        assert!(registry.find_project("Project-A").is_some());
        // Case insensitive match
        assert!(registry.find_project("project-a").is_some());
        // Trimmed search
        assert!(registry.find_project("  project-a  ").is_some());
        // Non-existent search
        assert!(registry.find_project("Project-B").is_none());
    }

    #[test]
    fn test_add_round_success_and_trimming() {
        let mut history = ReviewHistory { reviews: vec![] };
        history
            .add_round(sample_round("  my-project  ", 1))
            .unwrap();

        assert_eq!(history.reviews.len(), 1);
        assert_eq!(history.reviews[0].project, "my-project");
    }

    #[test]
    fn test_add_round_duplicate_fails() {
        let mut history = ReviewHistory { reviews: vec![] };
        history.add_round(sample_round("Project-A", 1)).unwrap();

        // Exact match duplicate
        let res1 = history.add_round(sample_round("Project-A", 1));
        assert!(res1.is_err());
        match res1.unwrap_err() {
            GuildError::DuplicateReviewRound(project, round) => {
                assert_eq!(project, "Project-A");
                assert_eq!(round, 1);
            }
            other => panic!("expected DuplicateReviewRound, got: {:?}", other),
        }

        // Case-insensitive duplicate
        let res2 = history.add_round(sample_round("project-a", 1));
        assert!(res2.is_err());
        match res2.unwrap_err() {
            GuildError::DuplicateReviewRound(project, round) => {
                assert_eq!(project, "project-a");
                assert_eq!(round, 1);
            }
            other => panic!("expected DuplicateReviewRound, got: {:?}", other),
        }

        // Different round for same project succeeds
        history.add_round(sample_round("Project-A", 2)).unwrap();
        assert_eq!(history.reviews.len(), 2);
    }

    #[test]
    fn test_add_round_invalid_properties() {
        let mut history = ReviewHistory { reviews: vec![] };

        // Empty project name
        let res_empty = history.add_round(sample_round("   ", 1));
        assert!(res_empty.is_err());
        match res_empty.unwrap_err() {
            GuildError::InvalidReviewRound(msg) => assert!(msg.contains("empty")),
            other => panic!("expected InvalidReviewRound, got: {:?}", other),
        }

        // Round = 0
        let res_zero = history.add_round(sample_round("Project-A", 0));
        assert!(res_zero.is_err());
        match res_zero.unwrap_err() {
            GuildError::InvalidReviewRound(msg) => assert!(msg.contains("greater than 0")),
            other => panic!("expected InvalidReviewRound, got: {:?}", other),
        }
    }

    #[test]
    fn test_latest_round_out_of_order() {
        let mut history = ReviewHistory { reviews: vec![] };

        // Add rounds out of order
        history.add_round(sample_round("Project-A", 2)).unwrap();
        history.add_round(sample_round("Project-A", 1)).unwrap();
        history.add_round(sample_round("Project-A", 3)).unwrap();

        // Add rounds for another project to ensure filtering works
        history.add_round(sample_round("Project-B", 5)).unwrap();

        let latest_a = history.latest_round("project-a").unwrap();
        assert_eq!(latest_a.round, 3);

        let latest_b = history.latest_round("  PROJECT-B  ").unwrap();
        assert_eq!(latest_b.round, 5);

        assert!(history.latest_round("nonexistent").is_none());
    }

    #[test]
    fn test_complete_checkpoint_success_case_insensitive() {
        let mut progress = Progress {
            checkpoints: vec![
                sample_checkpoint("checkpoint-1", "Title 1", false),
                sample_checkpoint("checkpoint-2", "Title 2", false),
            ],
        };

        // Exact match
        progress.complete_checkpoint("checkpoint-1").unwrap();
        assert!(progress.checkpoints[0].completed);

        // Case-insensitive & trimmed match
        progress.complete_checkpoint("  CHECKPOINT-2  ").unwrap();
        assert!(progress.checkpoints[1].completed);
    }

    #[test]
    fn test_complete_checkpoint_not_found() {
        let mut progress = Progress {
            checkpoints: vec![sample_checkpoint("checkpoint-1", "Title 1", false)],
        };

        let res = progress.complete_checkpoint("nonexistent");
        assert!(res.is_err());
        match res.unwrap_err() {
            GuildError::CheckpointNotFound(id) => assert_eq!(id, "nonexistent"),
            other => panic!("expected CheckpointNotFound, got: {:?}", other),
        }
    }

    #[test]
    fn test_next_incomplete() {
        let mut progress = Progress {
            checkpoints: vec![
                sample_checkpoint("checkpoint-1", "Title 1", true),
                sample_checkpoint("checkpoint-2", "Title 2", false),
                sample_checkpoint("checkpoint-3", "Title 3", false),
            ],
        };

        // First incomplete is checkpoint-2
        let next = progress.next_incomplete().unwrap();
        assert_eq!(next.id, "checkpoint-2");

        // Complete checkpoint-2
        progress.complete_checkpoint("checkpoint-2").unwrap();

        // First incomplete is now checkpoint-3
        let next = progress.next_incomplete().unwrap();
        assert_eq!(next.id, "checkpoint-3");

        // Complete checkpoint-3
        progress.complete_checkpoint("checkpoint-3").unwrap();

        // No more incomplete checkpoints
        assert!(progress.next_incomplete().is_none());
    }
}
