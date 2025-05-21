use serde::{Deserialize, Serialize};
use crate::function::Function;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolCall {
    index: i64,
    function: Function,
    id: String,
    type_: String,
}