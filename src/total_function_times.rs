use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TotalFunctionTimes {
    timestamp: f64,
    retrieve_state: f64,
    get_rules: f64,
    get_groups: f64,
    get_devices: f64,
    get_response: f64,
}
