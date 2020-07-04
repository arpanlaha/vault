// use git2::Repository;

use arangors::{client::reqwest::ReqwestClient, ClientError, Connection, Database};
use serde::{de::DeserializeOwned, Deserialize};
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Category {
    category: String,
    description: String,
    id: usize,
    path: String,
    slug: String,
}

#[derive(Deserialize, Debug)]
struct Crate {
    description: String,
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Keyword {
    crates_cnt: usize,
    id: usize,
    keyword: String,
}

trait ArangoDocument {
    fn get_insert_query(&self) -> String;
}

fn escape_quotes(input: &String) -> String {
    input.replace("\"", "\\\"")
}

impl ArangoDocument for Category {
    fn get_insert_query(&self) -> String {
        let Category {
            category,
            description,
            id,
            path,
            slug,
        } = self;
        format!(
            r#"INSERT {{ category: "{}", description: "{}", id: {}, path: "{}", slug: "{}" }} INTO categories"#,
            category,
            // TODO: fix appearance in db
            escape_quotes(description),
            id,
            path,
            slug
        )
    }
}

impl ArangoDocument for Crate {
    fn get_insert_query(&self) -> String {
        let Crate {
            description,
            id,
            name,
        } = self;
        format!(
            r#"INSERT {{ description: "{}", id: {}, name: "{}" }} INTO crates"#,
            escape_quotes(description),
            id,
            name
        )
    }
}

impl ArangoDocument for Keyword {
    fn get_insert_query(&self) -> String {
        let Keyword {
            crates_cnt,
            id,
            keyword,
        } = self;
        format!(
            r#"INSERT {{ crates_cnt: {}, id: {}, keyword: "{}" }}"#,
            crates_cnt,
            id,
            escape_quotes(keyword),
        )
    }
}

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

    load_documents::<Category>(&db, "categories").await?;
    load_documents::<Crate>(&db, "crates").await?;
    load_documents::<Keyword>(&db, "keywords").await?;

    Ok(())
}

async fn load_documents<T: DeserializeOwned + ArangoDocument + Debug>(
    db: &Database<'_, ReqwestClient>,
    filename: &str,
) -> Result<(), ClientError> {
    println!("Loading {}...", filename);
    for result in csv::Reader::from_reader(BufReader::new(
        File::open(Path::new(format!("../dump/data/{}.csv", filename).as_str())).unwrap(),
    ))
    .deserialize()
    {
        let record: T = result.unwrap();
        let _vec: Vec<T> = db.aql_str(record.get_insert_query().as_str()).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    connect_db().await.unwrap();
}
