use serde::{Deserialize, Serialize};

/// Apprentice profile — stored in `~/.guild/data/profile.toml`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub handle: String,
    pub joined: String,
}

/// Project registry entry — stored in `~/.guild/data/projects.toml`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRegistry {
    pub projects: Vec<Project>,
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
