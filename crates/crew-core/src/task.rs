//! Task model: atomic unit of work.

use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{AgentId, EpisodeRef, Message, TaskId};

/// A task is an atomic unit of work assigned to an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique identifier.
    pub id: TaskId,
    /// Parent task ID (for subtasks).
    pub parent_id: Option<TaskId>,
    /// Current status.
    pub status: TaskStatus,
    /// What kind of task this is.
    pub kind: TaskKind,
    /// Context passed to the agent.
    pub context: TaskContext,
    /// Result after completion (if any).
    pub result: Option<TaskResult>,
    /// When the task was created.
    pub created_at: DateTime<Utc>,
    /// When the task was last updated.
    pub updated_at: DateTime<Utc>,
}

impl Task {
    /// Create a new task with the given kind and context.
    pub fn new(kind: TaskKind, context: TaskContext) -> Self {
        let now = Utc::now();
        Self {
            id: TaskId::new(),
            parent_id: None,
            status: TaskStatus::Pending,
            kind,
            context,
            result: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Create a subtask of this task.
    pub fn subtask(&self, kind: TaskKind) -> Self {
        let now = Utc::now();
        Self {
            id: TaskId::new(),
            parent_id: Some(self.id.clone()),
            status: TaskStatus::Pending,
            kind,
            context: self.context.clone(),
            result: None,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Task execution status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum TaskStatus {
    /// Waiting to be assigned.
    Pending,
    /// Currently being executed by an agent.
    InProgress { agent_id: AgentId },
    /// Blocked waiting for something.
    Blocked { reason: String },
    /// Successfully completed.
    Completed,
    /// Failed with an error.
    Failed { error: String },
}

/// What kind of work the task represents.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TaskKind {
    /// Plan: decompose a goal into subtasks.
    Plan { goal: String },
    /// Code: write or modify code.
    Code {
        instruction: String,
        files: Vec<PathBuf>,
    },
    /// Review: review code changes.
    Review { diff: String },
    /// Test: run tests or verification.
    Test { command: String },
    /// Custom task type.
    Custom {
        name: String,
        params: serde_json::Value,
    },
}

/// Context passed to an agent when executing a task.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskContext {
    /// Working directory for the task.
    pub working_dir: PathBuf,
    /// Git state (branch, uncommitted changes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_state: Option<GitState>,
    /// Recent conversation turns (working memory).
    pub working_memory: Vec<Message>,
    /// References to relevant past episodes.
    pub episodic_refs: Vec<EpisodeRef>,
    /// Files in scope for this task.
    pub files_in_scope: Vec<PathBuf>,
}

/// Git repository state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitState {
    pub branch: String,
    pub has_uncommitted_changes: bool,
    pub head_commit: Option<String>,
}

/// Result of task execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    /// Whether the task succeeded.
    pub success: bool,
    /// Output or summary.
    pub output: String,
    /// Files that were modified.
    pub files_modified: Vec<PathBuf>,
    /// Subtasks created (for Plan tasks).
    pub subtasks: Vec<TaskId>,
    /// Token usage.
    pub token_usage: TokenUsage,
}

/// Token usage statistics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_new() {
        let task = Task::new(
            TaskKind::Plan {
                goal: "test goal".to_string(),
            },
            TaskContext::default(),
        );

        assert_eq!(task.status, TaskStatus::Pending);
        assert!(task.parent_id.is_none());
        assert!(task.result.is_none());
    }

    #[test]
    fn test_task_subtask() {
        let parent = Task::new(
            TaskKind::Plan {
                goal: "parent goal".to_string(),
            },
            TaskContext {
                working_dir: PathBuf::from("/test"),
                ..Default::default()
            },
        );

        let child = parent.subtask(TaskKind::Code {
            instruction: "implement feature".to_string(),
            files: vec![],
        });

        assert_eq!(child.parent_id, Some(parent.id.clone()));
        assert_eq!(child.status, TaskStatus::Pending);
        assert_eq!(child.context.working_dir, parent.context.working_dir);
    }

    #[test]
    fn test_task_status_serialization() {
        let status = TaskStatus::InProgress {
            agent_id: crate::AgentId::new("test-agent"),
        };
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("in_progress"));
        assert!(json.contains("test-agent"));

        let parsed: TaskStatus = serde_json::from_str(&json).unwrap();
        assert!(matches!(parsed, TaskStatus::InProgress { .. }));
    }

    #[test]
    fn test_task_kind_serialization() {
        let kind = TaskKind::Code {
            instruction: "fix bug".to_string(),
            files: vec![PathBuf::from("src/main.rs")],
        };
        let json = serde_json::to_string(&kind).unwrap();
        assert!(json.contains("code"));
        assert!(json.contains("fix bug"));

        let parsed: TaskKind = serde_json::from_str(&json).unwrap();
        assert!(matches!(parsed, TaskKind::Code { .. }));
    }

    #[test]
    fn test_token_usage_default() {
        let usage = TokenUsage::default();
        assert_eq!(usage.input_tokens, 0);
        assert_eq!(usage.output_tokens, 0);
    }
}
