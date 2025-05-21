use serde::{Deserialize, Serialize};
use crate::function::Function;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolCall {
    pub index: i64,
    pub function: Function,
    pub id: String,
    pub type_: String,
}