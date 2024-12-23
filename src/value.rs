use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

#[cfg(test)]
pub(crate) static COMPLETE_JSON: &str = r#"{
    "string": "This is a string",
    "number": 42,
    "float": 3.14,
    "exponential": 2.998e8,
    "negative": -17,
    "true": true,
    "false": false,
    "null": null,
    "array": [
        1,
        2,
        "three",
        4.5,
        true,
        null
    ],
    "object": {
        "key1": "value1",
        "key2": 100,
        "key3": {
            "nested": "object"
        }
    },
    "emptyArray": [],
    "emptyObject": {}
}"#;
