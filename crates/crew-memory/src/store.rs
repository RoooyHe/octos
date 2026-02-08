//! Episode store: persistent storage for episodes using redb (pure Rust).

use std::path::Path;
use std::sync::Arc;

use eyre::{Result, WrapErr};
use redb::{Database, ReadableTable, TableDefinition};
use tracing::debug;

use crate::episode::Episode;

/// Table for episodes: key = episode_id, value = JSON
const EPISODES_TABLE: TableDefinition<&str, &str> = TableDefinition::new("episodes");

/// Index table for episodes by working directory: key = cwd, value = list of episode IDs (JSON)
const CWD_INDEX_TABLE: TableDefinition<&str, &str> = TableDefinition::new("cwd_index");

/// Store for episodes using redb (pure Rust embedded database).
pub struct EpisodeStore {
    db: Arc<Database>,
}

impl EpisodeStore {
    /// Open or create an episode store at the given path.
    pub async fn open(data_dir: impl AsRef<Path>) -> Result<Self> {
        let data_dir = data_dir.as_ref().to_path_buf();
        tokio::fs::create_dir_all(&data_dir)
            .await
            .wrap_err("failed to create data directory")?;

        let db_path = data_dir.join("episodes.redb");

        // redb is sync, so we spawn_blocking for the initial open
        let db = tokio::task::spawn_blocking(move || {
            Database::create(&db_path).wrap_err("failed to open redb database")
        })
        .await??;

        // Initialize tables
        let write_txn = db.begin_write()?;
        {
            // Create tables if they don't exist
            let _ = write_txn.open_table(EPISODES_TABLE)?;
            let _ = write_txn.open_table(CWD_INDEX_TABLE)?;
        }
        write_txn.commit()?;

        debug!(path = %data_dir.display(), "opened episode store");

        Ok(Self { db: Arc::new(db) })
    }

    /// Store an episode.
    pub async fn store(&self, episode: Episode) -> Result<()> {
        let db = self.db.clone();
        let episode_id = episode.id.clone();
        let cwd = episode.working_dir.to_string_lossy().to_string();
        let episode_json =
            serde_json::to_string(&episode).wrap_err("failed to serialize episode")?;

        tokio::task::spawn_blocking(move || {
            let write_txn = db.begin_write()?;
            {
                // Store episode
                let mut table = write_txn.open_table(EPISODES_TABLE)?;
                table.insert(episode_id.as_str(), episode_json.as_str())?;

                // Update cwd index
                let mut index = write_txn.open_table(CWD_INDEX_TABLE)?;
                let existing: Vec<String> = index
                    .get(cwd.as_str())?
                    .map(|v| serde_json::from_str(v.value()).unwrap_or_default())
                    .unwrap_or_default();

                let mut ids = existing;
                if !ids.contains(&episode_id) {
                    ids.push(episode_id);
                }
                let ids_json = serde_json::to_string(&ids)?;
                index.insert(cwd.as_str(), ids_json.as_str())?;
            }
            write_txn.commit()?;
            Ok::<_, eyre::Report>(())
        })
        .await??;

        Ok(())
    }

    /// Find episodes relevant to a query in the given directory.
    pub async fn find_relevant(
        &self,
        cwd: &Path,
        query: &str,
        limit: usize,
    ) -> Result<Vec<Episode>> {
        let db = self.db.clone();
        let cwd_str = cwd.to_string_lossy().to_string();
        let query = query.to_lowercase();

        tokio::task::spawn_blocking(move || {
            let read_txn = db.begin_read()?;
            let episodes_table = read_txn.open_table(EPISODES_TABLE)?;
            let index_table = read_txn.open_table(CWD_INDEX_TABLE)?;

            // Get episode IDs for this cwd
            let episode_ids: Vec<String> = index_table
                .get(cwd_str.as_str())?
                .map(|v| serde_json::from_str(v.value()).unwrap_or_default())
                .unwrap_or_default();

            // Load and filter episodes
            let mut results: Vec<(Episode, usize)> = Vec::new();

            for id in episode_ids {
                if let Some(json) = episodes_table.get(id.as_str())? {
                    if let Ok(episode) = serde_json::from_str::<Episode>(json.value()) {
                        // Simple relevance: count query term matches in summary
                        let summary_lower = episode.summary.to_lowercase();
                        let relevance = query
                            .split_whitespace()
                            .filter(|term| summary_lower.contains(term))
                            .count();

                        if relevance > 0 {
                            results.push((episode, relevance));
                        }
                    }
                }
            }

            // Sort by relevance (descending) then by date (descending)
            results.sort_by(|a, b| {
                b.1.cmp(&a.1)
                    .then_with(|| b.0.created_at.cmp(&a.0.created_at))
            });

            Ok(results.into_iter().take(limit).map(|(e, _)| e).collect())
        })
        .await?
    }

    /// Get the N most recent episodes for a directory.
    pub async fn recent_for_cwd(&self, cwd: &Path, n: usize) -> Result<Vec<Episode>> {
        let db = self.db.clone();
        let cwd_str = cwd.to_string_lossy().to_string();

        tokio::task::spawn_blocking(move || {
            let read_txn = db.begin_read()?;
            let episodes_table = read_txn.open_table(EPISODES_TABLE)?;
            let index_table = read_txn.open_table(CWD_INDEX_TABLE)?;

            // Get episode IDs for this cwd
            let episode_ids: Vec<String> = index_table
                .get(cwd_str.as_str())?
                .map(|v| serde_json::from_str(v.value()).unwrap_or_default())
                .unwrap_or_default();

            // Load episodes
            let mut episodes: Vec<Episode> = Vec::new();

            for id in episode_ids {
                if let Some(json) = episodes_table.get(id.as_str())? {
                    if let Ok(episode) = serde_json::from_str::<Episode>(json.value()) {
                        episodes.push(episode);
                    }
                }
            }

            // Sort by created_at descending
            episodes.sort_by(|a, b| b.created_at.cmp(&a.created_at));

            Ok(episodes.into_iter().take(n).collect())
        })
        .await?
    }

    /// Get an episode by ID.
    pub async fn get(&self, id: &str) -> Result<Option<Episode>> {
        let db = self.db.clone();
        let id = id.to_string();

        tokio::task::spawn_blocking(move || {
            let read_txn = db.begin_read()?;
            let table = read_txn.open_table(EPISODES_TABLE)?;

            if let Some(json) = table.get(id.as_str())? {
                let episode: Episode = serde_json::from_str(json.value())?;
                Ok(Some(episode))
            } else {
                Ok(None)
            }
        })
        .await?
    }
}
