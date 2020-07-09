use crate::arango::{
    client::{drop_collections, get_connection, get_db},
    document::{
        Category, Crate, CrateCategory, CrateKeyword, Dependency, Keyword, RedisGraphDocument,
        RedisGraphNode, SqlDependency, Version,
    },
};
use arangors::ClientError;
use redisgraph::{Graph, RedisGraphResult};
use semver_parser::version as semver_version;
use serde::de::DeserializeOwned;
use std::any;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;

fn get_collection_path(data_path: &str, collection_name: &str) -> String {
    format!("{}/data/{}.csv", data_path, collection_name)
}

fn create_indices(db: &mut Graph) -> RedisGraphResult<()> {
    db.mutate(Category::create_index().as_str())?;
    db.mutate(Crate::create_index().as_str())?;
    db.mutate(Keyword::create_index().as_str())?;
    Ok(())
}

pub async fn load_database(data_path: &str) -> Result<(), ClientError> {
    println!("Connecting to database...");
    let start = Instant::now();
    let connection = get_connection().unwrap();
    println!("Driver connection established.");
    let mut db = get_db(connection, "vault").unwrap();

    println!("Database connection established.");
    println!("Loading documents...");

    drop_collections(
        &mut db,
        vec![
            "Category",
            "Crate",
            "Keyword",
            "HAS_CATEGORY",
            "HAS_KEYWORD",
            "DEPENDS_ON",
        ],
    )
    .unwrap();

    create_indices(&mut db).unwrap();

    load_documents::<Category>(&mut db, data_path, "categories").await?;
    load_documents::<Crate>(&mut db, data_path, "crates").await?;
    load_documents::<Keyword>(&mut db, data_path, "keywords").await?;

    load_documents::<CrateCategory>(&mut db, data_path, "crates_categories").await?;
    load_documents::<CrateKeyword>(&mut db, data_path, "crates_keywords").await?;

    let versions_to_crates = load_versions(&mut db, data_path).await?;
    load_dependencies(&mut db, data_path, &versions_to_crates).await?;

    println!(
        "Finished loading documents into database in {} seconds.",
        start.elapsed().as_secs_f64()
    );

    Ok(())
}

async fn load_documents<T: DeserializeOwned + RedisGraphDocument + Debug>(
    db: &mut Graph,
    data_path: &str,
    collection_name: &str,
) -> Result<(), ClientError> {
    println!("Loading {}...", collection_name);
    let start = Instant::now();
    let mut count = 0usize;

    let file_path = get_collection_path(data_path, collection_name);

    for result in csv::Reader::from_reader(BufReader::new(
        File::open(Path::new(&file_path)).expect(format!("Unable to open {}", file_path).as_str()),
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
        db.mutate(record.get_insert_query().as_str()).unwrap();
    }

    println!(
        "Loaded {} {} into database in {} seconds.",
        count,
        collection_name,
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

async fn load_versions(
    db: &mut Graph,
    data_path: &str,
) -> Result<HashMap<usize, usize>, ClientError> {
    println!("Loading versions...");
    let start = Instant::now();
    let mut count = 0usize;
    let versions_path = get_collection_path(data_path, "versions");

    let versions = get_versions(versions_path);

    for version in versions.values() {
        db.mutate(version.get_insert_query().as_str()).unwrap();
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
        let SqlDependency { kind, optional, .. } = sql_dependency;
        let from_version_id = sql_dependency.version_id;
        let to = sql_dependency.crate_id;

        if let Some(&from) = versions_to_crates.get(&from_version_id) {
            dependencies.push(Dependency {
                from,
                kind,
                optional: optional == "t",
                to,
            });
        }
    }
    dependencies
}

async fn load_dependencies(
    db: &mut Graph,
    data_path: &str,
    versions_to_crates: &HashMap<usize, usize>,
) -> Result<(), ClientError> {
    println!("Loading dependencies...");
    let start = Instant::now();
    let mut count = 0usize;

    let dependencies_path = get_collection_path(data_path, "dependencies");

    let dependencies = get_dependencies(dependencies_path, versions_to_crates);

    for dependency in dependencies {
        db.mutate(dependency.get_insert_query().as_str()).unwrap();
        count += 1;
    }

    println!(
        "Loaded {} dependencies into database in {} seconds.",
        count,
        start.elapsed().as_secs_f64()
    );

    Ok(())
}
