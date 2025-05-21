use std::os::linux::raw::stat;
use serde::{Deserialize, Serialize};
use crate::total_function_times::TotalFunctionTimes;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]

pub struct Property {
    #[serde(rename = "type")]
    type_: String,
    description: String,
}