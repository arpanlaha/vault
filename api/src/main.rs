use arangors::{client::reqwest::ReqwestClient, ClientError, Connection, Database};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use vault::{ArangoDocument, Category, Crate, Keyword};

async fn get_connection() -> Result<Connection, ClientError> {
    println!("Establishing driver connection...");
    Connection::establish_jwt(
        dotenv::var("ARANGODB_URI").unwrap().as_str(),
        dotenv::var("ARANGODB_USER").unwrap().as_str(),
        dotenv::var("ARANGODB_PASSWORD").unwrap().as_str(),
    )
    .await
}

async fn connect_db() -> Result<(), ClientError> {
    println!("Connecting to database...");
    let connection = get_connection().await?;
    let db = connection.db("vault").await?;

    println!("Database connection established.");
    println!("Loading documents...");

    load_documents::<Category>(&db, "categories").await?;
    load_documents::<Crate>(&db, "crates").await?;
    load_documents::<Keyword>(&db, "keywords").await?;
    println!("Finished loading documents into database.");

    Ok(())
}

async fn load_documents<T: DeserializeOwned + ArangoDocument + Debug>(
    db: &Database<'_, ReqwestClient>,
    filename: &str,
) -> Result<(), ClientError> {
    println!("Loading {}...", filename);
    let mut count = 0usize;

    for result in csv::Reader::from_reader(BufReader::new(
        File::open(Path::new(format!("../dump/data/{}.csv", filename).as_str())).unwrap(),
    ))
    .deserialize()
    {
        let record: T = result.unwrap();
        let _vec: Vec<T> = db.aql_str(record.get_insert_query().as_str()).await?;
        count += 1
    }

    println!("Loaded {} {} into database.", count, filename);

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    connect_db().await.unwrap();
}
