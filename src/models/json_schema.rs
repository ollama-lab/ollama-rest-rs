use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionDef {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<Box<TypeDef>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum TypeDef {
    Function {
        function: FunctionDef,
    },
    Object {
        properties: BTreeMap<String, TypeDef>,
        required: Option<Vec<String>>,
    },
    String {
        description: Option<String>,
        #[serde(rename = "enum")]
        enumeration: Option<Vec<String>>,
    },
}
