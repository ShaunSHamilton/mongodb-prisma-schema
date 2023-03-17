use std::collections::HashMap;

use futures_util::TryStreamExt;
use mongodb::{
    bson::{doc, Bson, Document},
    options::{ClientOptions, FindOptions},
    Client,
};
use serde_json::Value;
use tokio;

// mongod --dbpath ~/data/prod/restore-6412e8c0ac8b9a17b12c4d47/

type HashValue = HashMap<String, Vec<Value>>;

use clap::Parser;

/// Script to generate a schema for a MongoDB collection
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of database to use
    #[arg(short, long)]
    db: String,

    /// Name of collection to query
    #[arg(short, long)]
    collection: String,

    /// Output file path relative to current directory
    /// If not provided, will default to `schema.json`
    /// in the current directory
    #[arg(short, long, default_value = "schema.json")]
    output: String,

    /// MongoDB connection string
    /// If not provided, will default to `mongodb://127.0.0.1:27017`
    #[arg(short, long, default_value = "mongodb://127.0.0.1:27017")]
    uri: String,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let args = Args::parse();
    // Parse your connection string into an options struct
    let mut client_options = ClientOptions::parse(args.uri).await?;

    // Manually set an option
    client_options.app_name = Some("Rust Mongeese".to_string());

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    client
        .database(&args.db)
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");
    let db = client.database(&args.db);

    let users_collection = db.collection::<Document>(&args.collection);

    // find the first 2 documents in users_collection, and print them, without iterating over the entire cursor
    let mut cursor = users_collection
        .find(doc! {}, FindOptions::builder().limit(2).build())
        .await?;

    let mut schema_map: HashValue = HashMap::new();

    while let Some(user) = cursor.try_next().await? {
        build_schema(&user, &mut schema_map);
    }
    write_hashmap(&schema_map, args.output);
    Ok(())
}

fn get_data_type(bson: &Bson, key: &str) -> Value {
    let t = match bson {
        Bson::Array(array) => {
            let mut types: Vec<Value> = Vec::new();
            for (_i, value) in array.iter().enumerate() {
                let t = get_data_type(value, key);
                if !types.contains(&t) {
                    types.push(t);
                }
            }
            serde_json::json!(types)
        }
        Bson::Document(doc) => {
            let mut types: Vec<Value> = Vec::new();
            for (key, value) in doc {
                let t = get_data_type(value, key);
                if !types.contains(&t) {
                    types.push(t);
                }
            }
            serde_json::json!(types)
        }
        _ => serde_json::json!(bson_to_string(bson)),
    };
    // TODO: Duplicates keys
    serde_json::json!({ key: t })
}

// Current logic is flawed:
// If nested values match, they will still be added to the schema:
// User A:
// ```json
// {
//     "_id": "1234",
//     "files": [{"name": "hi", "ext": "js"}]
// }
// ```
// User B:
// ```json
// {
//     "_id": 1234,
//     "files": [{"name": "hi", "ext": null}]
// }
// ```
// Will result in:
// ```json
// {
//     "_id": ["String", "Int32"],
//     "files": [
//         {
//             "name": ["String"],
//             "ext": ["String"]
//         },
//         {
//             "name": ["String"],
//             "ext": ["Null"]
//         }
//     ]
// }
fn build_schema(doc: &Document, schema_map: &mut HashValue) {
    for (key, value) in doc {
        let t = get_data_type(value, key);
        append_to_schema_map(schema_map, key.to_string(), t);
    }
}

fn append_to_schema_map(schema_map: &mut HashValue, key: String, value: Value) {
    if let Some(values) = schema_map.get_mut(&key) {
        if !values.contains(&value) {
            values.push(value);
        }
    } else {
        schema_map.insert(key, vec![value]);
    }
}

fn write_hashmap(schema_map: &HashValue, output: String) {
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
