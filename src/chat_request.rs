use serde::{Deserialize, Serialize};
use crate::chat_message::ChatMessage;

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatRequest {
    chat_history: Vec<ChatMessage>,
    functions: Vec<serde_json::Value>, // Placeholder, define if needed
    model: String,
    gpt_client: String,
}