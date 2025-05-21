use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TotalFunctionTimes {
    pub timestamp: f64,
    pub retrieve_state: f64,
    pub get_rules: f64,
    pub get_groups: f64,
    pub get_devices: f64,
    pub get_response: f64,
}
