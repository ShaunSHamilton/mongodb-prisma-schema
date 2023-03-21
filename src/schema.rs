use mongodb::bson::{Bson, Document};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
pub struct Schema(pub HashMap<String, Vec<Value>>);

// pub type Schema = HashMap<String, Vec<Value>>;

#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
pub struct Set {
    pub schemas: Vec<Schema>,
}

#[derive(Debug, PartialEq, Eq)]
enum Op<'a> {
    /// The schema compared to should be taken, and the schema compared from should be removed
    Take(&'a Schema),
    /// The schema compared from should be taken, and the schema compared to should be ignored
    Keep,
    /// The schemas are different, and the schema compared to should be added to the set
    Push,
    NoOp,
}

impl Set {
    pub fn new() -> Self {
        Self { schemas: vec![] }
    }
    pub fn insert(&mut self, schema: Schema) {
        if !self.contains(&schema) {
            self.compare_schemas(&schema);
        }
    }
    pub fn contains(&self, schema: &Schema) -> bool {
        self.schemas.contains(schema)
    }
    fn compare_schemas(&mut self, schema: &Schema) {
        let mut action = Op::Push;

        // Compare all values in schemas to each other
        let schemas_clone = self.schemas.clone();
        'outer: for s1 in &schemas_clone {
            for (k, v_vec) in schema.0.iter() {
                if let Some(s1_vec) = s1.0.get(k) {
                    if s1_vec == v_vec {
                        // values are the same. Do nothing here
                    } else {
                        for v1 in s1_vec {
                            for v2 in v_vec {
                                let a = rec(v1, v2, s1);
                                action = a;
                                match action {
                                    Op::Push => {
                                        // Continue comparing
                                    }
                                    Op::Keep => {
                                        // Break out and do not push
                                        break 'outer;
                                    }
                                    Op::Take(_original_schema) => {
                                        // Break out and take
                                        break 'outer;
                                    }
                                    Op::NoOp => {
                                        // Continue comparing
                                    }
                                }
                            }
                        }
                        // action = Op::Push;
                    }
                }
            }
        }
        match action {
            Op::Push => {
                self.schemas.push(schema.clone());
            }
            Op::Keep => {
                // No push
                // No change to self.schemas
            }
            Op::Take(original_schema) => {
                // Remove schema from self.schemas
                // Push schema
                self.schemas.retain(|s| s != original_schema);
                self.schemas.push(schema.clone());
            }
            Op::NoOp => {}
        };
    }
}

fn rec<'a>(s1_value: &Value, s2_value: &Value, original_schema: &'a Schema) -> Op<'a> {
    match (s1_value, s2_value) {
        (Value::Array(s1_arr), Value::Array(s2_arr)) => {
            // Handle empty arrays
            if s1_arr.is_empty() {
                return Op::Take(original_schema);
            }
            if s2_arr.is_empty() {
                return Op::Keep;
            }
            for s1 in s1_arr {
                for s2 in s2_arr {
                    let t = rec(s1, s2, original_schema);
                    if t == Op::Push {
                        return t;
                    }
                }
            }
            return Op::NoOp;
        }
        (Value::Object(s1_obj), Value::Object(s2_obj)) => {
            for (k, s1) in s1_obj {
                if let Some(s2) = s2_obj.get(k) {
                    let t = rec(s1, s2, original_schema);
                    if t == Op::Push {
                        return t;
                    }
                } else {
                    return Op::Push;
                }
            }
            return Op::NoOp;
        }
        (x, y) => {
            if x == y {
                return Op::NoOp;
            }
            return Op::Push;
        }
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
                sub_doc.insert(key.to_string(), json!([t]));
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
    #[ignore]
    fn build_single_schema() {
        let doc_1 = doc! {
            "name": "Shaun",
            "age": 25,
            "likes": ["cats", "dogs", "rust"],
            "dislikes": ["jquery", null, {"jsObjects": "because they are bad"}],
            "arr": [],
            "p": {
                "a": true,
                "b": false
            }
        };
        let doc_3 = doc! {
            "name": "Oliver",
            "age": 32,
            "likes": ["worms", "fish", "F#"],
            "dislikes": ["typescript", null, {"jsObjects": "because he can"}],
            "arr": ["string"],
            "p": {
                "a": true,
                "b": false
            }
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
        for doc in vec![doc_1, doc_2, doc_3] {
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
        schema.insert("arr".to_string(), vec![json!(["String"])]);
        schema.insert(
            "p".to_string(),
            vec![json!({"a": ["Boolean"], "b": ["Boolean"]})],
        );

        expected_schemas.insert(schema);

        println!("{:#?}", schemas.schemas);
        println!("{:#?}", expected_schemas.schemas);
        assert_eq!(schemas, expected_schemas);
    }

    #[test]
    fn generate_schemas_set() {
        // To Test:
        // 1. Empty arrays are typed as `key: [[]]`
        // 2. Existing Array<Never> is replaced with Array<T> if schema is equal
        // 3. Nested objects are correctly typed
        // 4. Nested arrays are correctly typed
        // 5. Root level fields are correctly typed
        let doc_1 = doc! {
            "name": "Shaun",
            "age": 25,
            "likes": ["cats", "dogs", "rust"],
        };
        let doc_3 = doc! {
            "name": "Oliver",
            "age": 32,
            "likes": ["worms", "fish", "F#"],
        };
        let doc_5 = doc! {
            "name": "Kris",
            "age": 400,
            "likes": []
        };

        let doc_2 = doc! {
            "name": "Tom",
            "age": "28?",
            "likes": "fishing",
        };
        let doc_4 = doc! {
            "name": "Mrugesh",
            "age": null,
            "likes": "devops"
        };
        let doc_6 = doc! {
            "name": "Niraj",
            "age": 20,
            "likes": [["traveling"], "Fluttering"],
            "arr": [
                {
                    "a": true,
                },
                {
                    "b": false
                }
            ]
        };
        let doc_7 = doc! {
            "name": "Sem",
            "age": 20,
            "likes": [["hiking"], "Cypress"],
            "arr": [
                {
                    "a": true,
                },
                {
                    "c": false
                }
            ]
        };

        let mut schemas = Set::new();
        for doc in vec![doc_1, doc_2, doc_3, doc_4, doc_5, doc_6, doc_7] {
            let schema = Schema::from(&doc);
            schemas.insert(schema);
        }
        let mut expected_schemas = Vec::new();
        let mut schema_1 = Schema::new();
        schema_1.insert("name".to_string(), vec![json!("String")]);
        schema_1.insert("age".to_string(), vec![json!("Int32")]);
        schema_1.insert("likes".to_string(), vec![json!(["String"])]);
        expected_schemas.push(schema_1);

        let mut schema_2 = Schema::new();
        schema_2.insert("name".to_string(), vec![json!("String")]);
        schema_2.insert("age".to_string(), vec![json!("String")]);
        schema_2.insert("likes".to_string(), vec![json!("String")]);
        expected_schemas.push(schema_2);

        let mut schema_3 = Schema::new();
        schema_3.insert("name".to_string(), vec![json!("String")]);
        schema_3.insert("age".to_string(), vec![json!("Null")]);
        schema_3.insert("likes".to_string(), vec![json!("String")]);
        expected_schemas.push(schema_3);

        let mut schema_4 = Schema::new();
        schema_4.insert("name".to_string(), vec![json!("String")]);
        schema_4.insert("age".to_string(), vec![json!("Int32")]);
        schema_4.insert("likes".to_string(), vec![json!([["String"], "String"])]);
        schema_4.insert(
            "arr".to_string(),
            vec![json!([{"a": ["Boolean"], "b": ["Boolean"]}])],
        );
        expected_schemas.push(schema_4);

        let mut schema_5 = Schema::new();
        schema_5.insert("name".to_string(), vec![json!("String")]);
        schema_5.insert("age".to_string(), vec![json!("Int32")]);
        schema_5.insert("likes".to_string(), vec![json!([["String"], "String"])]);
        schema_5.insert(
            "arr".to_string(),
            vec![json!([{"a": ["Boolean"], "c": ["Boolean"]}])],
        );
        expected_schemas.push(schema_5);

        // TODO: Figure out how to test nested structures
        // NOTE: Simple Eq cannot compare nested structures
        assert_eq!(schemas.schemas.len(), 5);
    }
}
