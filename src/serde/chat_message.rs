use serde::{Deserialize, Serialize};
use crate::serde::tool_call::ToolCall;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    pub tool_calls: Option<Vec<ToolCall>>,
}