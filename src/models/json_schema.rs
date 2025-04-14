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

/// A **partly** implemented JSON Schema enum.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn def_function_schema() {
        const FUNC_NAME: &'static str = "query_weather";
        const FUNC_DESC: &'static str = "Get current weather in a specified location.";

        const LOC_DESC: &'static str = "Keywords of the location.";

        let obj = serde_json::from_value::<JsonSchema>(serde_json::json!({
            "type": "function",
            "function": {
                "name": FUNC_NAME,
                "description": FUNC_DESC,
                "parameters": {
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": LOC_DESC,
                        },
                    },
                    "required": ["location"],
                },
            },
        })).unwrap();

        assert!(matches!(obj, JsonSchema::Function { .. }));

        if let JsonSchema::Function { function } = obj {
            assert_eq!(function.name, FUNC_NAME);

            assert!(matches!(function.description, Some(_)));
            assert_eq!(function.description.unwrap(), FUNC_DESC);

            assert!(matches!(function.parameters, Some(_)));

            if let Some(boxed_schema) = function.parameters {
                let param_schema = *boxed_schema;

                assert!(matches!(param_schema, JsonSchema::Object { .. }));
                if let JsonSchema::Object { properties, required } = param_schema {
                    let location_schema = properties.get("location");
                    assert!(matches!(location_schema, Some(_)));
                    if let Some(location_schema) = location_schema {
                        assert!(matches!(location_schema, JsonSchema::String { .. }));
                        if let JsonSchema::String { description, enumeration } = location_schema {
                            assert!(matches!(description, Some(_)));
                            if let Some(description) = description {
                                assert_eq!(description, LOC_DESC);
                            }

                            assert!(matches!(enumeration, None));
                        }
                    }

                    assert!(matches!(required, Some(_)));
                    if let Some(required_fields) = required {
                        assert_eq!(required_fields.len(), 1);
                        assert_eq!(required_fields[0], "location");
                    }
                }
            }
        }
    }
}
