use serde::{Deserialize, Serialize};
use crate::chat_gpt_entry::ChatGptEntry;
use crate::chat_message::ChatMessage;
use crate::time_summary::TimeSummary;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SessionLogEntry {
    ChatMessage(ChatMessage),
    ChatGpt{chat_gpt: ChatGptEntry},
    TimeSummary(TimeSummary),
}