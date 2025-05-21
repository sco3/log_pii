use serde::{Deserialize, Serialize};
use crate::chat_gpt_entry::ChatGptEntry;
use crate::time_summary::TimeSummary;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Entry {
    Message {
        role: String,
        content: String,
    },
    ChatGpt {
        chat_gpt: ChatGptEntry,
    },
    TimeSummary {
        time_summary_s: TimeSummary,
    },
}