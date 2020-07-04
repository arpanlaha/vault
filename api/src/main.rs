// use git2::Repository;

use serde::Deserialize;
use std::path::Path;
// use tempfile::{tempdir, TempDir};
// use semver_parser::version;
use arangors::{
    client::reqwest::ReqwestClient, ClientError, Collection, Connection, Database, Document,
};
// use derive::ArangoDocument;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
// use traits::ArangoDocument;
#[derive(Deserialize, Debug)]
struct Crate {
    description: String,
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Category {
    category: String,
    description: String,
    id: usize,
    path: String,
    slug: String,
}

trait ArangoDocument {
    fn get_insert(&self) -> String;
}

impl ArangoDocument for Category {
    fn get_insert(&self) -> String {
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
            description.replace("\"", "\\\""),
            id,
            path,
            slug
        )
    }
}

impl ArangoDocument for Crate {
    fn get_insert(&self) -> String {
        let Crate {
            description,
            id,
            name,
        } = self;
        format!(
            r#"INSERT {{ description: "{}", id: {}, name: "{}" }} INTO documents"#,
            description.replace("\"", "\\\""),
            id,
            name
        )
    }
}

// fn traverse_dir(path: &Path, crates: &mut HashMap<String, Crate>) -> io::Result<()> {
//     if !path.to_str().unwrap().ends_with(".git") {
//         for dir_entry in fs::read_dir(path)? {
//             let dir_entry = dir_entry?;
//             if dir_entry.file_type()?.is_dir() {
//                 traverse_dir(dir_entry.path().as_path(), crates)?;
//             } else {
//                 for line in BufReader::new(File::open(dir_entry.path().as_path())?).lines() {
//                     let deserialized: Result<Crate, _> = serde_json::from_str(&line?);
//                     if let Ok(line_crate) = deserialized {
//                         crates.insert(
//                             format!("{}@{}", line_crate.name, line_crate.vers),
//                             line_crate,
//                         );
//                     }
//                 }
//             }
//         }
//     }
//     Ok(())
// }

async fn get_connection() -> Result<Connection, ClientError> {
    Connection::establish_jwt(
        dotenv::var("ARANGODB_URI").unwrap().as_str(),
        dotenv::var("ARANGODB_USER").unwrap().as_str(),
        dotenv::var("ARANGODB_PASSWORD").unwrap().as_str(),
    )
    .await
}

// async fn get_db(connection: &Connection, db: &str) -> Result<Database, ClientError> {
//     connection.db(db).await
// }

// async fn g

// async fn get_collection(collection: &str) -> Result<Collection, ClientError> {
//     let conn = Connection::establish_jwt(
//         dotenv::var("ARANGODB_URI").unwrap().as_str(),
//         dotenv::var("ARANGODB_USER").unwrap().as_str(),
//         dotenv::var("ARANGODB_PASSWORD").unwrap().as_str(),
//     )
//     .await
//     .unwrap().db("vault").await.unwrap()collection("categories").await
// }

async fn connect_db() -> Result<(), ClientError> {
    let connection = get_connection().await?;
    let db = connection.db("vault").await?;

    load_categories(db).await?;

    Ok(())
}

async fn load_categories(db: Database<'_, ReqwestClient>) -> Result<(), ClientError> {
    for result in csv::Reader::from_reader(BufReader::new(
        File::open(Path::new("../dump/data/categories.csv")).unwrap(),
    ))
    .deserialize()
    {
        let record: Category = result.unwrap();
        let _vec: Vec<Category> = db.aql_str(record.get_insert().as_str()).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();

    // let temp_dir: TempDir = tempdir().unwrap();
    // let path: &Path = temp_dir.path();
    // Repository::clone("https://github.com/rust-lang/crates.io-index.git", path).unwrap();
    println!("Hello, world!");
    // read_categories().unwrap();

    connect_db().await.unwrap();

    // for dir_entry in fs::read_dir(Path::new("data")).unwrap() {
    //     let dir_entry = dir_entry
    // }

    // let mut crates: HashMap<String, Crate> = HashMap::new();

    // traverse_dir(Path::new("data"), &mut crates).unwrap();

    // println!("number of crates: {}", crates.len());

    // let example_path = Path::new("data/ac/ti/actix-web");
    // let file_contents = fs::read_to_string(example_path).unwrap();
    // for line in file_contents.split("\n").filter(|line| line.len() > 0) {
    //     // println!("size: {}", line.len());
    //     let line_crate: Crate = serde_json::from_str(&line).unwrap();
    //     println!("crate: {:?}\n", line_crate);
    // }
    // let example_file = File::open(example_path).unwrap();
    // temp_dir.close().unwrap();
}
