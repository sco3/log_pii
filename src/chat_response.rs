use serde::{Deserialize, Serialize};
use crate::chat_message::ChatMessage;

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatResponse {
    agent: String,
    message: ChatMessage,
}