use arangors::{client::reqwest::ReqwestClient, ClientError, Database};
use flate2::read::GzDecoder;
use semver_parser::version as semver_version;
use serde::de::DeserializeOwned;
use std::any;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use std::process::Command;
use std::time::Instant;
use tar::Archive;
use tempfile::TempDir;
use vault::arango::{
    client::{get_connection, get_db},
    document::{
        ArangoDocument, Category, Crate, CrateCategory, CrateKeyword, Dependency, Keyword,
        SqlDependency, Version,
    },
};

fn get_data_path(temp_dir: &TempDir) -> Option<String> {
    for dir_entry in fs::read_dir(temp_dir.path()).expect("Unable to read temporary directory") {
        let dir_entry = dir_entry.unwrap();
        if let Ok(file_type) = dir_entry.file_type() {
            if file_type.is_dir() {
                return Some(String::from(
                    dir_entry
                        .path()
                        .as_path()
                        .to_str()
                        .expect("Data path is not valid UTF-8"),
                ));
            }
        }
    }

    None
}

fn fetch_data() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let tgz_path = temp_dir.path().join("crates_data.tar.gz");
    let tgz_path_name = tgz_path
        .as_path()
        .to_str()
        .expect("Tarball path not valid UTF-8");

    println!("Downloading tarballed database dump...");
    Command::new("curl")
        .arg("https://static.crates.io/db-dump.tar.gz")
        .arg("-o")
        .arg(tgz_path_name)
        .output()
        .expect("Unable to fetch Crates database dump");
    println!("Tarballed database dump downloaded.");

    println!("Unzipping tarballed database dump...");
    let tar = GzDecoder::new(
        File::open(tgz_path_name).expect(format!("Unable to open {}", tgz_path_name).as_str()),
    );
    println!("Unzipped tarballed database dump into TAR archive.");

    println!("Unpacking database dump TAR archive...");
    Archive::new(tar)
        .unpack(temp_dir.path())
        .expect("Unable to unpack database dump TAR archive");
    println!("Unpacked database dump TAR.");

    temp_dir
}

fn get_collection_pathname(temp_dir_pathname: &str, filename: &str) -> String {
    format!("{}/data/{}.csv", temp_dir_pathname, filename)
}

async fn connect_db(data_path: &str) -> Result<(), ClientError> {
    println!("Connecting to database...");
    let start = Instant::now();
    let connection = get_connection().await?;
    println!("Driver connection established.");
    let db = get_db(&connection, "vault").await?;

    println!("Database connection established.");
    println!("Loading documents...");

    load_documents::<Category>(&db, get_collection_pathname(data_path, "categories")).await?;
    load_documents::<Crate>(&db, get_collection_pathname(data_path, "crates")).await?;
    load_documents::<Keyword>(&db, get_collection_pathname(data_path, "keywords")).await?;

    load_documents::<CrateCategory>(&db, get_collection_pathname(data_path, "crates_categories"))
        .await?;
    load_documents::<CrateKeyword>(&db, get_collection_pathname(data_path, "crates_keywords"))
        .await?;

    let versions_to_crates =
        load_versions(&db, get_collection_pathname(data_path, "versions")).await?;
    load_dependencies(
        &db,
        get_collection_pathname(data_path, "dependencies"),
        &versions_to_crates,
    )
    .await?;

    println!(
        "Finished loading documents into database in {} seconds.",
        start.elapsed().as_secs_f64()
    );

    Ok(())
}

async fn load_documents<T: DeserializeOwned + ArangoDocument + Debug>(
    db: &Database<'_, ReqwestClient>,
    filename: String,
) -> Result<(), ClientError> {
    println!("Loading {}...", filename);
    let start = Instant::now();
    let mut count = 0usize;

    for result in csv::Reader::from_reader(BufReader::new(
        File::open(Path::new(&filename)).expect(format!("Unable to open {}", filename).as_str()),
    ))
    .deserialize()
    {
        count += 1;
        let record: T = result.expect(
            format!(
                "Unable to deserialize entry {} as {}",
                count,
                any::type_name::<T>()
            )
            .as_str(),
        );
        let _: Vec<T> = db.aql_str(record.get_insert_query().as_str()).await?;
    }

    println!(
        "Loaded {} {} into database in {} seconds.",
        count,
        filename,
        start.elapsed().as_secs_f64()
    );

    Ok(())
}

fn get_versions(filename: String) -> HashMap<usize, Version> {
    let mut versions = HashMap::<usize, Version>::new();
    let mut count = 0usize;
    for result in csv::Reader::from_reader(BufReader::new(
        File::open(Path::new(&filename)).expect(format!("Unable to open {}", filename).as_str()),
    ))
    .deserialize()
    {
        count += 1;
        let version: Version =
            result.expect(format!("Unable to deserialize entry {} as Version", count).as_str());
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
                        existing_version.id = version.id;
                    } else if version.is_pre() == existing_version.is_pre() {
                        if version_num.major > existing_version_num.major
                            || (version_num.major == existing_version_num.major
                                && version_num.minor > existing_version_num.minor)
                            || (version_num.major == existing_version_num.major
                                && version_num.minor == existing_version_num.minor
                                && version_num.patch > existing_version_num.patch)
                        {
                            existing_version.num = version.num;
                            existing_version.id = version.id;
                        }
                    }
                } else {
                    existing_version.num = version.num;
                    existing_version.id = version.id;
                }
            } else if let Err(_) = semver_version::parse(existing_version.num.as_str()) {
                if version.created_at.cmp(&existing_version.created_at) == Ordering::Greater {
                    existing_version.num = version.num;
                    existing_version.id = version.id;
                }
            }
        } else {
            versions.insert(crate_id, version);
        }
    }

    versions
}

async fn load_versions<'a>(
    db: &'a Database<'_, ReqwestClient>,
    filename: String,
) -> Result<HashMap<usize, usize>, ClientError> {
    println!("Loading versions...");
    let start = Instant::now();
    let mut count = 0usize;

    let versions = get_versions(filename);

    for version in versions.values() {
        let _: Vec<Version> = db.aql_str(version.get_insert_query().as_str()).await?;
        count += 1;
    }

    println!(
        "Loaded {} versions into database in {} seconds.",
        count,
        start.elapsed().as_secs_f64()
    );

    Ok(versions
        .iter()
        .map(|(crate_id, version)| (version.id, *crate_id))
        .collect())
}

fn get_dependencies(
    filename: String,
    versions_to_crates: &HashMap<usize, usize>,
) -> Vec<Dependency> {
    let mut dependencies = Vec::with_capacity(versions_to_crates.len());
    let mut count = 0usize;
    for result in csv::Reader::from_reader(BufReader::new(
        File::open(Path::new(&filename)).expect(format!("Unable to open {}", filename).as_str()),
    ))
    .deserialize()
    {
        count += 1;
        let sql_dependency: SqlDependency =
            result.expect(format!("Unable to deserialize entry {} as Dependency", count).as_str());
        let SqlDependency { id, kind, .. } = sql_dependency;
        let from_version_id = sql_dependency.version_id;
        let to = sql_dependency.crate_id;

        if let Some(&from) = versions_to_crates.get(&from_version_id) {
            dependencies.push(Dependency { from, id, kind, to });
        }
    }
    dependencies
}

async fn load_dependencies<'a>(
    db: &'a Database<'_, ReqwestClient>,
    filename: String,
    versions_to_crates: &HashMap<usize, usize>,
) -> Result<(), ClientError> {
    println!("Loading dependencies...");
    let start = Instant::now();
    let mut count = 0usize;

    let dependencies = get_dependencies(filename, versions_to_crates);

    for dependency in dependencies {
        let _: Vec<Dependency> = db.aql_str(dependency.get_insert_query().as_str()).await?;
        count += 1;
    }

    println!(
        "Loaded {} dependencies into database in {} seconds.",
        count,
        start.elapsed().as_secs_f64()
    );

    Ok(())
}

fn clean_tempdir(temp_dir: TempDir) {
    println!("Cleaning up temporary files and directories...");
    temp_dir
        .close()
        .expect("Unable to close temporary directory");
    println!("Temporary files and directories removed.");
}

#[tokio::main]
async fn main() {
    let temp_dir = fetch_data();

    let data_path = get_data_path(&temp_dir).expect("Unable to locate data path");

    dotenv::dotenv().unwrap();

    connect_db(data_path.as_str()).await.unwrap();

    clean_tempdir(temp_dir);
}
