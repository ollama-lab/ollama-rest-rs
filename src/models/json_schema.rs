use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionDef {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<Box<DataStructure>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DataStructure {
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
        properties: BTreeMap<String, DataStructure>,
        required: Option<Vec<String>>,
    },
    String {
        description: Option<String>,
        #[serde(rename = "enum")]
        enumeration: Option<Vec<String>>,
    },
}
