use serde::{Deserialize, Serialize};
use crate::total_function_times::TotalFunctionTimes;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TimeSummary {
    pub total_function_times: TotalFunctionTimes,
    pub total_request_time: f64,
}


