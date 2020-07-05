use arangors::{client::reqwest::ReqwestClient, ClientError, Database};
use semver_parser::version as semver_version;
use serde::de::DeserializeOwned;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;
use vault::arango::{
    client::{get_connection, get_db},
    document::{ArangoDocument, Category, Crate, Keyword, Version},
};

async fn connect_db() -> Result<(), ClientError> {
    println!("Connecting to database...");
    let start = Instant::now();
    let connection = get_connection().await?;
    let db = get_db(&connection, "vault").await?;

    println!("Database connection established.");
    println!("Loading documents...");

    load_documents::<Category>(&db, "categories").await?;
    load_documents::<Crate>(&db, "crates").await?;
    load_documents::<Keyword>(&db, "keywords").await?;
    load_versions(&db).await?;

    println!(
        "Finished loading documents into database in {} seconds.",
        start.elapsed().as_secs_f64()
    );

    Ok(())
}

async fn load_documents<T: DeserializeOwned + ArangoDocument + Debug>(
    db: &Database<'_, ReqwestClient>,
    filename: &str,
) -> Result<(), ClientError> {
    println!("Loading {}...", filename);
    let start = Instant::now();
    let mut count = 0usize;

    for result in csv::Reader::from_reader(BufReader::new(
        File::open(Path::new(format!("../dump/data/{}.csv", filename).as_str())).unwrap(),
    ))
    .deserialize()
    {
        let record: T = result.unwrap();
        let _: Vec<T> = db.aql_str(record.get_insert_query().as_str()).await?;
        count += 1;
    }

    println!(
        "Loaded {} {} into database in {} seconds.",
        count,
        filename,
        start.elapsed().as_secs_f64()
    );

    Ok(())
}

fn get_versions() -> HashMap<usize, Version> {
    let mut versions = HashMap::<usize, Version>::new();
    let mut line_count = 0usize;
    for result in csv::Reader::from_reader(BufReader::new(
        File::open(Path::new("../dump/data/versions.csv")).unwrap(),
    ))
    .deserialize()
    {
        line_count += 1;
        let version: Version = result.expect(format!("line: {}", line_count).as_str());
        let Version { crate_id, .. } = version;

        if versions.contains_key(&crate_id) {
            let existing_version = versions.get_mut(&crate_id).unwrap();
            existing_version.downloads += version.downloads;

            if let Ok(version_num) = semver_version::parse(version.num.as_str()) {
                if let Ok(existing_version_num) =
                    semver_version::parse(existing_version.num.as_str())
                {
                    if !version.is_pre() && existing_version.is_pre() {
                        existing_version.num = version.num;
                    } else if version.is_pre() == existing_version.is_pre() {
                        if version_num.major > existing_version_num.major
                            || (version_num.major == existing_version_num.major
                                && version_num.minor > existing_version_num.minor)
                            || (version_num.major == existing_version_num.major
                                && version_num.minor == existing_version_num.minor
                                && version_num.patch > existing_version_num.patch)
                        {
                            existing_version.num = version.num;
                        }
                    }
                } else {
                    existing_version.num = version.num;
                }
            } else if let Err(_) = semver_version::parse(existing_version.num.as_str()) {
                if version.created_at.cmp(&existing_version.created_at) == Ordering::Greater {
                    existing_version.num = version.num;
                }
            }
        } else {
            versions.insert(crate_id, version);
        }
    }

    versions
}

async fn load_versions(db: &Database<'_, ReqwestClient>) -> Result<(), ClientError> {
    println!("Loading versions...");
    let start = Instant::now();
    let mut count = 0usize;

    for version in get_versions().values() {
        let _: Vec<Version> = db.aql_str(version.get_insert_query().as_str()).await?;
        count += 1;
    }

    println!(
        "Loaded {} versions into database in {} seconds.",
        count,
        start.elapsed().as_secs_f64()
    );

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    connect_db().await.unwrap();
}
