use mongodb::bson::{Bson, Document};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Schema(pub HashMap<String, Vec<Value>>);

// pub type Schema = HashMap<String, Vec<Value>>;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct Set {
    pub schemas: Vec<Schema>,
}

impl Set {
    pub fn new() -> Self {
        Self { schemas: vec![] }
    }
    pub fn insert(&mut self, schema: Schema) {
        // Before inserting, check if the schema already exists
        if self.contains(&schema) {
            return;
        }
        self.schemas.push(schema);
    }
    pub fn contains(&self, schema: &Schema) -> bool {
        self.schemas.contains(schema)
    }
}

impl Schema {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn insert(&mut self, key: String, value: Vec<Value>) {
        self.0.insert(key, value);
    }
}

impl From<&Document> for Schema {
    fn from(doc: &Document) -> Self {
        let mut schema = Schema::new();
        for (key, value) in doc {
            let t = get_type(value);
            schema.insert(key.to_string(), vec![t]);
        }
        schema
    }
}

fn get_type(bson: &Bson) -> Value {
    let t = match bson {
        Bson::Document(doc) => {
            let mut sub_doc = HashMap::new();
            for (key, value) in doc {
                let t = get_type(value);
                sub_doc.insert(key.to_string(), json!(t));
            }
            json!(sub_doc)
        }
        Bson::Array(arr) => {
            let mut sub_arr = Vec::new();
            for value in arr {
                let t = get_type(value);

                if !sub_arr.contains(&t) {
                    sub_arr.push(t);
                }
            }
            json!(sub_arr)
        }
        _ => json!(&bson_to_string(bson)),
    };
    t
}

pub fn write_hashmap(set: &Set, output: String) {
    let path = std::path::Path::new(&output);

    std::fs::write(path, serde_json::to_string_pretty(&set.schemas).unwrap()).unwrap();
}

fn bson_to_string(bson: &Bson) -> String {
    match bson {
        Bson::Double(_) => "Double".to_string(),
        Bson::String(_) => "String".to_string(),
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
        // Unreachable
        Bson::Array(_) => "Array".to_string(),
        Bson::Document(_) => "Document".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::bson::doc;
    use serde_json::json;
    #[test]
    fn test_build_schema() {
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
        let mut schemas = Set::new();
        for doc in vec![doc_1, doc_2] {
            let schema = Schema::from(&doc);
            schemas.insert(schema);
        }
        let mut expected_schemas = Set::new();
        let mut schema = Schema::new();
        schema.insert("name".to_string(), vec![json!("String")]);
        schema.insert("age".to_string(), vec![json!("Int32"), json!("String")]);
        schema.insert(
            "likes".to_string(),
            vec![json!(["String"]), json!("String")],
        );
        schema.insert(
            "dislikes".to_string(),
            vec![json!([
                "String",
                "Null",
                { "jsObjects": ["String"] },
                { "presentations": [["String"]] }
            ])],
        );
        schema.insert(
            "skills".to_string(),
            vec![json!({"frameworks": [["String"]], "languages": [["String"]]})],
        );
        schema.insert("arr".to_string(), vec![json!([])]);

        expected_schemas.insert(schema);

        println!("{:#?}", schemas.schemas);
        println!("{:#?}", expected_schemas.schemas);
        assert_eq!(schemas, expected_schemas);
    }
}
