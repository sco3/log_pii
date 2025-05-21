use serde::{Deserialize, Serialize};
use crate::serde::chat_message::ChatMessage;

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatResponse {
    pub agent: String,
    pub message: ChatMessage,
}