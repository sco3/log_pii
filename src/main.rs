mod serde;

use crate::serde::session_log_entry::SessionLogEntry;

//use sonic_rs;

use std::{fs, time::Instant};

fn main() {
    let data_str = fs::read("test/log.json").unwrap();

    println!("file content: {} bytes", data_str.len());

    let mut log_data: Vec<SessionLogEntry> = vec![];
    let mut log_bytes: Vec<u8> = vec![];

    let start = Instant::now();
    for _ in 0..1_000_000 {
        log_data = serde_json::from_slice(&data_str).unwrap();
        log_bytes = serde_json::to_vec(&log_data).unwrap();
        //log_data = sonic_rs::from_slice(&data_str).unwrap();
        //log_bytes = sonic_rs::to_vec(&log_data).unwrap();
    }

    let took = start.elapsed().as_millis();

    println!("{:?}\n{:?}", log_data, log_bytes);
    println!("Took: {} ms", took);
}
