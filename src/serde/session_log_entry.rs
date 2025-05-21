use serde::{Deserialize, Serialize};
use crate::serde::chat_gpt_entry::ChatGptEntry;
use crate::serde::chat_message::ChatMessage;
use crate::serde::time_summary::TimeSummary;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SessionLogEntry {
    ChatMessage(ChatMessage),
    ChatGpt { chat_gpt: ChatGptEntry },
    TimeSummary { time_summary_s: TimeSummary },
}