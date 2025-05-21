use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]

pub struct Property {
    #[serde(rename = "type")]
    pub type_: String,
    pub description: String,
}
