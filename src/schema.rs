use mongodb::bson::{Bson, Document};
// use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;

pub type Schema = HashMap<String, Vec<Value>>;

// #[derive(Serialize, PartialEq, Debug)]
// pub enum HashValue {
//     String(String),
//     Array(Vec<HashValue>),
//     Document(HashMap<String, Schema>),
// }

/// Recursive function that builds a schema from a bson document.
/// Takes a bson document and a mutable reference to a hashmap.
///
/// 1. For each field in Bson
///   - If Document, check each key
///     - Recurse
///   - If Array, check each item
///     - Keep track of associated key
///   - Else, add type to `key` Vec in HashMap
fn g(bson: &Bson, schema_map: &mut Schema, prev_key: Option<String>) {
    match bson {
        Bson::Document(doc) => {
            for (key, value) in doc {
                match value {
                    Bson::Document(doc) => {
                        let mut sub_schema_map: Schema = HashMap::new();
                        let bson = Bson::Document(doc.clone());
                        let prev_key = Some(key.to_string());
                        g(&bson, &mut sub_schema_map, prev_key);
                        append_to_schema_map(schema_map, key.to_string(), json!(sub_schema_map));
                    }
                    Bson::Array(arr) => {
                        let mut sub_schema_map: Schema = HashMap::new();
                        for item in arr {
                            let prev_key = Some(key.to_string());
                            g(item, &mut sub_schema_map, prev_key);
                        }
                        let sub_schema_values = sub_schema_map.values().collect::<Vec<_>>();
                        let prev_key = Some(key.to_string());
                        append_to_schema_map(
                            schema_map,
                            prev_key.unwrap(),
                            json!(sub_schema_values),
                        );
                    }
                    _ => {
                        append_to_schema_map(
                            schema_map,
                            key.to_string(),
                            json!(bson_to_string(&value)),
                        );
                    }
                }
            }
        }
        Bson::Array(arr) => {
            let mut sub_schema_map: Schema = HashMap::new();
            for item in arr {
                g(item, &mut sub_schema_map, prev_key.clone());
            }
            let sub_schema_values = sub_schema_map.values().collect::<Vec<_>>();
            append_to_schema_map(schema_map, prev_key.unwrap(), json!(sub_schema_values));
        }
        other => {
            append_to_schema_map(schema_map, prev_key.unwrap(), json!(bson_to_string(&other)));
        }
    }
}

pub fn build_schema(doc: &Document, schema_map: &mut Schema) {
    let bson = Bson::Document(doc.clone());
    g(&bson, schema_map, None);
}

fn append_to_schema_map(schema_map: &mut Schema, key: String, value: Value) {
    if let Some(values) = schema_map.get_mut(&key) {
        if !values.contains(&value) {
            values.push(value);
        }
    } else {
        schema_map.insert(key, vec![value]);
    }
}

pub fn write_hashmap(schema_map: &Schema, output: String) {
    let path = std::path::Path::new(&output);

    std::fs::write(path, serde_json::to_string_pretty(schema_map).unwrap()).unwrap();
}

fn bson_to_string(bson: &Bson) -> String {
    match bson {
        Bson::Double(_) => "Double".to_string(),
        Bson::String(_) => "String".to_string(),
        Bson::Array(_) => "Array".to_string(),
        Bson::Document(_) => "Document".to_string(),
        Bson::Boolean(_) => "Boolean".to_string(),
        Bson::Null => "Null".to_string(),
        Bson::RegularExpression(_) => "RegularExpression".to_string(),
        Bson::JavaScriptCode(_) => "JavaScriptCode".to_string(),
        Bson::JavaScriptCodeWithScope(_) => "JavaScriptCodeWithScope".to_string(),
        Bson::Int32(_) => "Int32".to_string(),
        Bson::Int64(_) => "Int64".to_string(),
        Bson::Timestamp(_) => "Timestamp".to_string(),
        Bson::Binary(_) => "Binary".to_string(),
        Bson::ObjectId(_) => "ObjectId".to_string(),
        Bson::DateTime(_) => "DateTime".to_string(),
        Bson::Symbol(_) => "Symbol".to_string(),
        Bson::Decimal128(_) => "Decimal128".to_string(),
        Bson::Undefined => "Undefined".to_string(),
        Bson::MaxKey => "MaxKey".to_string(),
        Bson::MinKey => "MinKey".to_string(),
        Bson::DbPointer(_) => "DbPointer".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::bson::doc;
    use serde_json::json;
    #[test]
    fn test_build_schema() {
        let mut schema_map: Schema = HashMap::new();
        let doc_1 = doc! {
            "name": "Shaun",
            "age": 25,
            "likes": ["cats", "dogs", "rust"],
            "dislikes": ["jquery", null, {"jsObjects": "because they are bad"}],
            "arr": []

        };
        let doc_2 = doc! {
            "name": "Tom",
            "age": "28?",
            "likes": "fishing",
            "dislikes": ["not fishing", null, {"presentations": ["because they are difficult"]}],
            "skills": {
                "languages": ["js", "sql"],
                "frameworks": ["react"]
            }
        };
        build_schema(&doc_1, &mut schema_map);
        build_schema(&doc_2, &mut schema_map);
        let mut expected_hashmap = HashMap::new();
        expected_hashmap.insert("name".to_string(), vec![json!("String")]);
        expected_hashmap.insert("age".to_string(), vec![json!("Int32"), json!("String")]);
        expected_hashmap.insert(
            "likes".to_string(),
            vec![json!(["String"]), json!("String")],
        );
        expected_hashmap.insert(
            "dislikes".to_string(),
            vec![json!([
                "String",
                "Null",
                { "jsObjects": ["String"] },
                { "presentations": [["String"]] }
            ])],
        );
        expected_hashmap.insert(
            "skills".to_string(),
            vec![json!({"frameworks": [["String"]], "languages": [["String"]]})],
        );
        expected_hashmap.insert("arr".to_string(), vec![json!([])]);

        println!("{:#?}", schema_map);
        println!("{:#?}", expected_hashmap);
        assert_eq!(schema_map, expected_hashmap);
    }
}
