mod session_log_entry;
mod total_function_times;
mod time_summary;
mod property;
mod parameters;
mod function;
mod tool_call;
mod chat_message;

mod chat_gpt_entry;
mod chat_response;
mod chat_request;

use session_log_entry::SessionLogEntry;
use total_function_times::TotalFunctionTimes;
use crate::parameters::Parameters;
use crate::time_summary::TimeSummary;
use maplit::{hashmap};
use crate::function::Function;
use crate::tool_call::ToolCall;

use std::fs;


fn main() {
    let data_str = fs::read_to_string("test/log.json").unwrap();

    println!("file content: {} bytes", data_str.len());
    let log_data: Vec<SessionLogEntry> = serde_json::from_str(&data_str).unwrap();

    println!("{:?}", log_data);
    let log_str = serde_json::to_string_pretty(&log_data).unwrap();
    println!("{}", log_str);
}
