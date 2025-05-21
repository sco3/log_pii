use serde::{Deserialize, Serialize};
use crate::chat_message::ChatMessage;

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatRequest {
    pub chat_history: Vec<ChatMessage>,
    pub functions: Vec<serde_json::Value>, // Placeholder, define if needed
    pub model: String,
    pub gpt_client: String,
}