use std::fs::File;
use std::io::BufReader;

use serde_json;

use crate::{Schema, Set};

type SchemaArray = Vec<Schema>;

fn read_schema_array() -> SchemaArray {
    let file_path = "schema-array.json";
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let schema_array: SchemaArray = serde_json::from_reader(reader).expect("Failed to read file");
    schema_array
}

// Add "Undefined" to types that are not common
// Always traverse the master schema to look for "Undefined"s
fn merge_arrays_into_single_schema(schema_array: SchemaArray) -> Schema {
    let master = Schema::new();
    for schema in schema_array {
        let combined_schema = combine(master, schema);
    }
    master
}

fn combine(master: Schema, schema: Schema) -> Schema {
    let mut combined_schema = master;
    for (key, value) in schema.0 {
        match combined_schema.0.get(&key) {
            Some(master_value) => {
                let v = combine(master_value, value);
                combined_schema.insert(key, v);
                if master_value != value {
                    combined_schema.insert(key, "Undefined".to_string());
                }
            }
            None => {
                combined_schema.insert(key, value);
            }
        }
    }
    combined_schema
}
