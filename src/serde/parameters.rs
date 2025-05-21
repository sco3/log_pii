use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::serde::property::Property;
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Parameters {
    pub type_field: Option<String>,
    pub properties: Option<HashMap<String, Property>>,
    pub required: Option<Vec<String>>,
}

