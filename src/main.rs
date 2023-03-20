use std::collections::HashMap;

use clap::Parser;
use db::get_collection;
use futures_util::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};
use tokio;

mod clapper;
mod db;
mod schema;

use clapper::Args;
use schema::{build_schema, write_hashmap, Schema};

// mongod --dbpath ~/data/prod/restore-6412e8c0ac8b9a17b12c4d47/

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let args = Args::parse();
    // Parse your connection string into an options struct
    let collection = get_collection(&args.uri, &args.db, &args.collection).await?;
    // find the first 2 documents in users_collection, and print them, without iterating over the entire cursor
    let mut cursor = collection
        .find(doc! {}, FindOptions::builder().limit(10).build())
        .await?;

    let mut schema_map: Schema = HashMap::new();

    while let Some(user) = cursor.try_next().await? {
        build_schema(&user, &mut schema_map);
    }
    write_hashmap(&schema_map, args.output);
    Ok(())
}
