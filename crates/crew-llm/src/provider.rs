//! LLM provider trait.

use async_trait::async_trait;
use crew_core::Message;
use eyre::Result;

use crate::config::ChatConfig;
use crate::types::{ChatResponse, ToolSpec};

/// Trait for LLM providers.
///
/// This is intentionally minimal to reduce abstraction overhead.
/// Each provider implements the specifics of its API.
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Send a chat completion request.
    async fn chat(
        &self,
        messages: &[Message],
        tools: &[ToolSpec],
        config: &ChatConfig,
    ) -> Result<ChatResponse>;

    /// Get the model identifier.
    fn model_id(&self) -> &str;

    /// Get the provider name (e.g., "anthropic", "openai").
    fn provider_name(&self) -> &str;
}
