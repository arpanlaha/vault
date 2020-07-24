use super::schema::{
    Category, Crate, CrateCategory, CrateKeyword, Dependency, Keyword, SqlDependency, Version,
    Vertex,
};
use csv::Reader;
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

pub async fn load_database(
    data_path: &str,
) -> (
    HashMap<String, Category>,
    HashMap<String, Crate>,
    HashMap<String, Keyword>,
) {
    let start = Instant::now();
    println!("Loading registry graph...");

    let (
        (mut categories, category_id_lookup),
        (mut crates, crate_id_lookup),
        (mut keywords, keyword_id_lookup),
    ) = join!(
        load_vertices::<Category>(data_path, "categories"),
        load_vertices::<Crate>(data_path, "crates"),
        load_vertices::<Keyword>(data_path, "keywords")
    );

    let versions_to_crates = create_versioned_crates(data_path, &mut crates, &crate_id_lookup);

    load_dependencies(data_path, &mut crates, versions_to_crates, &crate_id_lookup);
    load_crate_categories(
        data_path,
        &mut crates,
        &mut categories,
        &crate_id_lookup,
        &category_id_lookup,
    );
    load_crate_keywords(
        data_path,
        &mut crates,
        &mut keywords,
        &crate_id_lookup,
        &keyword_id_lookup,
    );

    println!(
        "Finished loading registry graph in {} seconds.",
        start.elapsed().as_secs_f64()
    );

    (categories, crates, keywords)
}

async fn load_vertices<T: DeserializeOwned + Vertex + Debug>(
    data_path: &str,
    collection_name: &str,
) -> (HashMap<String, T>, HashMap<usize, String>) {
    println!("Loading {}...", collection_name);
    let start = Instant::now();
    let mut count = 0usize;

    let file_path = get_collection_path(data_path, collection_name);

    let mut collection = HashMap::<String, T>::new();
    let mut id_lookup = HashMap::<usize, String>::new();

    for result in Reader::from_reader(BufReader::new(
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
        id_lookup.insert(record.sql_id(), String::from(record.id()));
        collection.insert(String::from(record.id()), record);
    }

    println!(
        "Loaded {} {} in {} seconds.",
        count,
        collection_name,
        start.elapsed().as_secs_f64()
    );

    (collection, id_lookup)
}

fn get_versions(data_path: &str) -> HashMap<usize, Version> {
    println!("Loading versions...");
    let start = Instant::now();
    let mut versions = HashMap::<usize, Version>::new();
    let mut count = 0usize;
    let filename = get_collection_path(data_path, "versions");

    for result in Reader::from_reader(BufReader::new(
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
                    } else if version.is_pre() == existing_version.is_pre()
                        && (version_num.major > existing_version_num.major
                            || (version_num.major == existing_version_num.major
                                && version_num.minor > existing_version_num.minor)
                            || (version_num.major == existing_version_num.major
                                && version_num.minor == existing_version_num.minor
                                && version_num.patch > existing_version_num.patch))
                    {
                        existing_version.num = version.num;
                        existing_version.id = version.id;
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
    crates: &mut HashMap<String, Crate>,
    crate_id_lookup: &HashMap<usize, String>,
) -> HashMap<usize, String> {
    let versions = get_versions(data_path);
    let mut version_to_crates = HashMap::<usize, String>::new();
    println!("Creating versioned crates...");

    let start = Instant::now();

    for Version {
        crate_id,
        created_at,
        downloads,
        features,
        id,
        num,
    } in versions.values()
    {
        let crate_id = crate_id_lookup
            .get(crate_id)
            .expect(format!("Crate with SQL id {} does not exist", crate_id).as_str());

        let version_crate = crates
            .get_mut(crate_id)
            .expect(format!("Crate with id {} does not exist", crate_id).as_str());

        version_crate.created_at = created_at.to_owned();
        version_crate.downloads = downloads.to_owned();
        version_crate.features = serde_json::from_str(features)
            .expect(format!("Unable to deserialize {} as HashMap", features).as_str());
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
    crates: &mut HashMap<String, Crate>,
    versions_to_crates: HashMap<usize, String>,
    crate_id_lookup: &HashMap<usize, String>,
) {
    println!("Loading dependencies...");
    let start = Instant::now();
    let mut count = 0usize;
    let dependencies_path = get_collection_path(data_path, "dependencies");

    for result in Reader::from_reader(BufReader::new(
        File::open(Path::new(&dependencies_path))
            .expect(format!("Unable to open {}", dependencies_path).as_str()),
    ))
    .deserialize()
    {
        let sql_dependency: SqlDependency =
            result.expect(format!("Unable to deserialize entry {} as Dependency", count).as_str());
        let SqlDependency {
            default_features,
            features,
            kind,
            optional,
            version_id,
            ..
        } = sql_dependency;

        if let Some(from) = versions_to_crates.get(&version_id) {
            count += 1;
            // TODO: split count by kind -  1.151MB - 1.051MB = 100MB

            crates
                .get_mut(from)
                .expect(format!("Crate with id {} not found", from).as_str())
                .dependencies
                .insert(Dependency {
                    default_features: default_features == "t",
                    features: String::from(&features[1..features.len() - 1])
                        .split(",")
                        .filter(|split| split.len() > 0)
                        .map(|feature_str| String::from(feature_str))
                        .collect(),

                    from: from.to_owned(),
                    kind,
                    optional: optional == "t",
                    to: crate_id_lookup
                        .get(&sql_dependency.crate_id)
                        .expect(
                            format!("Unable to find crate with id {}", sql_dependency.crate_id)
                                .as_str(),
                        )
                        .to_owned(),
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
    crates: &mut HashMap<String, Crate>,
    categories: &mut HashMap<String, Category>,
    crate_id_lookup: &HashMap<usize, String>,
    category_id_lookup: &HashMap<usize, String>,
) {
    println!("Loading crate categories...");
    let start = Instant::now();
    let mut count = 0usize;

    let file_path = get_collection_path(data_path, "crates_categories");

    for result in Reader::from_reader(BufReader::new(
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

        let category_id = category_id_lookup
            .get(&category_id)
            .expect(format!("Unable to find category with id {}", category_id).as_str());

        let crate_id = crate_id_lookup
            .get(&crate_id)
            .expect(format!("Unable to find crate with id {}", crate_id).as_str());

        crates
            .get_mut(crate_id)
            .expect(format!("Crate with id {} not found", crate_id).as_str())
            .categories
            .insert(category_id.to_owned());

        categories
            .get_mut(category_id)
            .expect(format!("Category with id {} not found", category_id).as_str())
            .crates
            .insert(crate_id.to_owned());
    }

    println!(
        "Loaded {} crate categories in {} seconds.",
        count,
        start.elapsed().as_secs_f64()
    );
}

fn load_crate_keywords(
    data_path: &str,
    crates: &mut HashMap<String, Crate>,
    keywords: &mut HashMap<String, Keyword>,
    crate_id_lookup: &HashMap<usize, String>,
    keyword_id_lookup: &HashMap<usize, String>,
) {
    println!("Loading crate keywords...");
    let start = Instant::now();
    let mut count = 0usize;

    let file_path = get_collection_path(data_path, "crates_keywords");

    for result in Reader::from_reader(BufReader::new(
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

        let crate_id = crate_id_lookup
            .get(&crate_id)
            .expect(format!("Unable to find crate with id {}", crate_id).as_str());

        let keyword_id = keyword_id_lookup
            .get(&keyword_id)
            .expect(format!("Unable to find keyword with id {}", keyword_id).as_str());

        crates
            .get_mut(crate_id)
            .expect(format!("Crate with id {} not found", crate_id).as_str())
            .keywords
            .insert(keyword_id.to_owned());

        keywords
            .get_mut(keyword_id)
            .expect(format!("Keyword with id {} not found", keyword_id).as_str())
            .crates
            .push(crate_id.to_owned());
    }

    println!(
        "Loaded {} crate keywords in {} seconds.",
        count,
        start.elapsed().as_secs_f64()
    );
}
