use serde::{Deserialize, Serialize};
use crate::parameters::Parameters;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Function {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<Parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}
