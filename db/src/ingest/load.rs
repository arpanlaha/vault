use crate::arango::document::{
    Category, Crate, CrateCategory, CrateKeyword, Dependency, Keyword, SqlDependency, Version,
    Vertex,
};
// use arangors::ClientError;
// use redisgraph::{Graph, RedisGraphResult};
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
use tokio::join;

fn get_collection_path(data_path: &str, collection_name: &str) -> String {
    format!("{}/{}.csv", data_path, collection_name)
}

pub async fn load_database(data_path: &str) {
    println!("Connecting to database...");
    let start = Instant::now();
    println!("Driver connection established.");

    println!("Database connection established.");
    println!("Loading documents...");

    // let categories = load_documents::<Category>(data_path, "categories");
    // let crates = load_documents::<Crate>(data_path, "crates");
    // let keywords = load_documents::<Keyword>(data_path, "keywords");

    let (mut categories, mut crates, mut keywords) = join!(
        load_documents::<Category>(data_path, "categories"),
        load_documents::<Crate>(data_path, "crates"),
        load_documents::<Keyword>(data_path, "keywords")
    );

    let versions_to_crates = create_versioned_crates(data_path, &mut crates);

    load_dependencies(data_path, &mut crates, versions_to_crates);
    load_crate_categories(data_path, &mut crates, &mut categories);
    load_crate_keywords(data_path, &mut crates, &mut keywords);

    // load_documents::<Crate>(&mut client, data_path, "crates").await?;
    // load_documents::<Keyword>(&mut client, data_path, "keywords").await?;

    // load_documents::<CrateCategory>(&mut client, data_path, "crates_categories").await?;
    // load_documents::<CrateKeyword>(&mut client, data_path, "crates_keywords").await?;

    // let versions_to_crates = load_versions(&mut client, data_path).await?;
    // load_dependencies(&mut client, data_path, &versions_to_crates).await?;

    println!(
        "Finished loading documents into database in {} seconds.",
        start.elapsed().as_secs_f64()
    );
}

async fn load_documents<T: DeserializeOwned + Vertex + Debug>(
    data_path: &str,
    collection_name: &str,
) -> HashMap<usize, T> {
    println!("Loading {}...", collection_name);
    let start = Instant::now();
    let mut count = 0usize;

    let file_path = get_collection_path(data_path, collection_name);

    // client.begin(None).await?;

    let mut collection = HashMap::<usize, T>::new();

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
        collection.insert(record.id(), record);
        // run_query(client, record.get_insert_query().as_str()).await;
    }

    // client.commit().await?;

    println!(
        "Loaded {} {} in {} seconds.",
        count,
        collection_name,
        start.elapsed().as_secs_f64()
    );

    collection
}

fn get_versions(data_path: &str) -> HashMap<usize, Version> {
    println!("Loading versions...");
    let start = Instant::now();
    let mut versions = HashMap::<usize, Version>::new();
    let mut count = 0usize;
    let filename = get_collection_path(data_path, "versions");

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

    println!(
        "Processed {} versions in {} seconds.",
        count,
        start.elapsed().as_secs_f64()
    );

    versions
}

fn create_versioned_crates(
    data_path: &str,
    crates: &mut HashMap<usize, Crate>,
) -> HashMap<usize, usize> {
    let versions = get_versions(data_path);
    let mut version_to_crates = HashMap::<usize, usize>::new();
    println!("Creating versioned crates...");

    let start = Instant::now();

    for Version {
        crate_id,
        created_at,
        downloads,
        id,
        num,
    } in versions.values()
    {
        let version_crate = crates
            .get_mut(crate_id)
            .expect(format!("Crate with id {} does not exist", crate_id).as_str());

        version_crate.created_at = created_at.to_owned();
        version_crate.downloads = downloads.to_owned();
        version_crate.version = num.to_owned();

        version_to_crates.insert(id.to_owned(), crate_id.to_owned());
    }

    println!(
        "Created versioned crates in {} seconds.",
        start.elapsed().as_secs_f64()
    );

    version_to_crates
}

fn load_dependencies(
    data_path: &str,
    crates: &mut HashMap<usize, Crate>,
    versions_to_crates: HashMap<usize, usize>,
) {
    println!("Loading dependencies...");
    let start = Instant::now();
    let mut count = 0usize;
    let dependencies_path = get_collection_path(data_path, "dependencies");
    for result in csv::Reader::from_reader(BufReader::new(
        File::open(Path::new(&dependencies_path))
            .expect(format!("Unable to open {}", dependencies_path).as_str()),
    ))
    .deserialize()
    {
        count += 1;
        let sql_dependency: SqlDependency =
            result.expect(format!("Unable to deserialize entry {} as Dependency", count).as_str());
        let SqlDependency { kind, optional, .. } = sql_dependency;
        let from_version_id = sql_dependency.version_id;
        let to = sql_dependency.crate_id;

        if let Some(from) = versions_to_crates.get(&from_version_id) {
            crates
                .get_mut(from)
                .expect(format!("Crate with id {} not found", from).as_str())
                .dependencies
                .insert(Dependency {
                    kind,
                    optional: optional == "t",
                    to,
                });
        }
    }
    println!(
        "Loaded {} dependencies into database in {} seconds.",
        count,
        start.elapsed().as_secs_f64()
    );
}

fn load_crate_categories(
    data_path: &str,
    crates: &mut HashMap<usize, Crate>,
    categories: &mut HashMap<usize, Category>,
) {
    println!("Loading crate categories...");
    let start = Instant::now();
    let mut count = 0usize;

    let file_path = get_collection_path(data_path, "crates_categories");

    for result in csv::Reader::from_reader(BufReader::new(
        File::open(Path::new(&file_path)).expect(format!("Unable to open {}", file_path).as_str()),
    ))
    .deserialize()
    {
        count += 1;
        let CrateCategory {
            category_id,
            crate_id,
        } = result
            .expect(format!("Unable to deserialize entry {} as crate category", count).as_str());

        crates
            .get_mut(&crate_id)
            .expect(format!("Crate with id {} not found", crate_id).as_str())
            .categories
            .insert(category_id);

        categories
            .get_mut(&category_id)
            .expect(format!("Category with id {} not found", category_id).as_str())
            .crates
            .insert(crate_id);
    }

    println!(
        "Loaded {} crate categories in {} seconds.",
        count,
        start.elapsed().as_secs_f64()
    );
}

fn load_crate_keywords(
    data_path: &str,
    crates: &mut HashMap<usize, Crate>,
    keywords: &mut HashMap<usize, Keyword>,
) {
    println!("Loading crate keywords...");
    let start = Instant::now();
    let mut count = 0usize;

    let file_path = get_collection_path(data_path, "crates_keywords");

    for result in csv::Reader::from_reader(BufReader::new(
        File::open(Path::new(&file_path)).expect(format!("Unable to open {}", file_path).as_str()),
    ))
    .deserialize()
    {
        count += 1;
        let CrateKeyword {
            crate_id,
            keyword_id,
        } = result
            .expect(format!("Unable to deserialize entry {} as crate keyword", count).as_str());

        crates
            .get_mut(&crate_id)
            .expect(format!("Crate with id {} not found", crate_id).as_str())
            .keywords
            .insert(keyword_id);

        keywords
            .get_mut(&keyword_id)
            .expect(format!("Keyword with id {} not found", keyword_id).as_str())
            .crates
            .insert(crate_id);
    }

    println!(
        "Loaded {} crate keywords in {} seconds.",
        count,
        start.elapsed().as_secs_f64()
    );
}
