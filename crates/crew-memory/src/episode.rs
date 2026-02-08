//! Episode model: summary of a completed task.

use std::path::PathBuf;

use chrono::{DateTime, Utc};
use crew_core::{AgentId, TaskId};
use serde::{Deserialize, Serialize};

/// An episode is a summary of a completed task.
///
/// Episodes are stored in the episodic memory for future retrieval,
/// allowing agents to learn from past experiences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    /// Unique episode ID.
    pub id: String,
    /// The task this episode summarizes.
    pub task_id: TaskId,
    /// The agent that executed the task.
    pub agent_id: AgentId,
    /// Working directory where the task was executed.
    pub working_dir: PathBuf,
    /// LLM-generated summary of what happened.
    pub summary: String,
    /// Outcome of the task.
    pub outcome: EpisodeOutcome,
    /// Key decisions made during execution.
    pub key_decisions: Vec<String>,
    /// Files that were modified.
    pub files_modified: Vec<PathBuf>,
    /// When this episode was created.
    pub created_at: DateTime<Utc>,
}

impl Episode {
    /// Create a new episode.
    pub fn new(
        task_id: TaskId,
        agent_id: AgentId,
        working_dir: PathBuf,
        summary: String,
        outcome: EpisodeOutcome,
    ) -> Self {
        Self {
            id: uuid::Uuid::now_v7().to_string(),
            task_id,
            agent_id,
            working_dir,
            summary,
            outcome,
            key_decisions: Vec::new(),
            files_modified: Vec::new(),
            created_at: Utc::now(),
        }
    }
}

/// Outcome of a task episode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EpisodeOutcome {
    /// Task completed successfully.
    Success,
    /// Task failed.
    Failure,
    /// Task was blocked and needs human intervention.
    Blocked,
    /// Task was cancelled.
    Cancelled,
}
