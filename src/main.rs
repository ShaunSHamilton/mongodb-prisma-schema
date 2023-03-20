use clap::Parser;
use db::get_collection;
use futures_util::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};
use tokio;

mod clapper;
mod db;
mod schema;

use clapper::Args;
use schema::{write_hashmap, Schema, Set};

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

    let mut schemas = Set::new();

    while let Some(user) = cursor.try_next().await? {
        let schema = Schema::from(&user);
        schemas.insert(schema);
    }
    write_hashmap(&schemas, args.output);
    Ok(())
}
