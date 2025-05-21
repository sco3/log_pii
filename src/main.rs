mod session_log_entry;
mod total_function_times;
mod time_summary;
mod property;
mod parameters;
mod function;
mod tool_call;
mod chat_message;
mod log_entry;
mod chat_gpt_entry;
mod chat_response;
mod chat_request;

use session_log_entry::SessionLogEntry;
use total_function_times::TotalFunctionTimes;
use crate::parameters::Parameters;
use crate::time_summary::TimeSummary;
use maplit::hashmap;
use crate::function::Function;
use crate::tool_call::ToolCall;

use std::fs;


fn main() {
    let log_entry = SessionLogEntry {};
    println!("{}", serde_json::to_string(&log_entry).unwrap());

    let total_f_times = TotalFunctionTimes::default();
    println!("{}", serde_json::to_string(&total_f_times).unwrap());

    let time_summary = TimeSummary::default();
    println!("{}", serde_json::to_string(&time_summary).unwrap());

    let property = property::Property::default();
    println!("{}", serde_json::to_string(&property).unwrap());

    let parameters = Parameters {
        properties: Some(hashmap! {
            "a1".to_string() => property.clone()
        }),
        required: vec!["a1".to_string()].into(),
        ..Default::default()
    };
    println!("{}", serde_json::to_string_pretty(&parameters).unwrap());

    let function = Function {
        parameters: Some(parameters.clone()),
        ..Default::default()
    };
    println!("{}", serde_json::to_string_pretty(&function).unwrap());

    let tool_call = ToolCall::default();
    println!( //
              "ToolCall: {}\n{:?}",  //
              serde_json::to_string_pretty(&tool_call).unwrap(),
              tool_call,
    );
    let chat_message = chat_message::ChatMessage::default();
    println!("ChatMessage {}", serde_json::to_string_pretty(&chat_message).unwrap());

    let data_str = fs::read_to_string("test/log.json").unwrap();

    println!("file content: {} bytes", data_str.len());
}
