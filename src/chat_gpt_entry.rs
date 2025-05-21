use serde::{Deserialize, Serialize};
use crate::chat_request::ChatRequest;
use crate::chat_response::ChatResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatGptEntry {
    request: ChatRequest,
    response: ChatResponse,
    time: f64,
}