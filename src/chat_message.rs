use serde::{Deserialize, Serialize};
use crate::tool_call::ToolCall;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatMessage {
    role: String,
    content: String,
    tool_calls: Option<Vec<ToolCall>>,
}