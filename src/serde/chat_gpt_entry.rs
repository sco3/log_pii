use serde::{Deserialize, Serialize};
use crate::serde::chat_request::ChatRequest;
use crate::serde::chat_response::ChatResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatGptEntry {
    pub request: ChatRequest,
    pub response: ChatResponse,
    pub time: f64,
}