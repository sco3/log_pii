mod session_log_entry;
mod total_function_times;
mod time_summary;
mod property;
mod parameters;

use session_log_entry::SessionLogEntry;
use total_function_times::TotalFunctionTimes;
use crate::parameters::Parameters;
use crate::time_summary::TimeSummary;
use maplit::hashmap;

fn main() {
    let log_entry = SessionLogEntry {};
    println!("{}", serde_json::to_string(&log_entry).unwrap());

    let total_f_times = TotalFunctionTimes::default();
    println!("{}", serde_json::to_string(&total_f_times).unwrap());

    let time_summary = TimeSummary::default();
    println!("{}", serde_json::to_string(&time_summary).unwrap());

    let property = property::Property::default();
    println!("{}", serde_json::to_string(&property).unwrap());

    let parameter = Parameters {
        properties: Some(hashmap! {
            "a1".to_string() => property.clone()
        }),
        required: vec!["a1".to_string()].into(),
        ..Default::default()
    };
    println!("{}", serde_json::to_string_pretty(&parameter).unwrap());
}
