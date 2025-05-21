mod serde;

use crate::serde::session_log_entry::SessionLogEntry;
//use maplit::{hashmap};

use std::fs;


fn main() {
    let data_str = fs::read_to_string("test/log.json").unwrap();

    println!("file content: {} bytes", data_str.len());
    let log_data: Vec<SessionLogEntry> = serde_json::from_str(&data_str).unwrap();

    println!("{:?}", log_data);
    let log_str = serde_json::to_string_pretty(&log_data).unwrap();
    println!("{}", log_str);
}
