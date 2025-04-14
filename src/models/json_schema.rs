use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Function definition
///
/// Since 0.3.0
#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionDef {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<Box<JsonSchema>>,
}

/// A **partly** implemented JSON Schema object.
///
/// Since 0.3.0
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum JsonSchema {
    Function {
        function: FunctionDef,
    },
    Integer {
        description: Option<String>,
    },
    Number {
        description: Option<String>,
    },
    Object {
        properties: BTreeMap<String, JsonSchema>,
        required: Option<Vec<String>>,
    },
    String {
        description: Option<String>,
        #[serde(rename = "enum")]
        enumeration: Option<Vec<String>>,
    },
}
