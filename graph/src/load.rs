use super::{
    schema::{
        Category, Crate, CrateCategory, CrateKeyword, Dependency, Keyword, SqlDependency, Version,
    },
    traits::Vertex,
};
use csv::Reader;
use semver_parser::version as semver_version;
use serde::de::DeserializeOwned;
use std::{
    any, cmp::Ordering, collections::HashMap, fmt::Debug, fs::File, io::BufReader, path::Path,
    time::Instant,
};

/// Returns the path of the file containing rows for the specified collection.
///
/// # Arguments
/// * `data_path` - the path to the `data` directory inside the database dump.
/// * `collection_name` - the name of the collection.
fn get_collection_path(data_path: &str, collection_name: &str) -> String {
    format!("{}/{}.csv", data_path, collection_name)
}

/// Returns a tuple containing the categories, crates, and keywords loaded from a crates.io database dump.
///
/// # Arguments
/// * `data_path` - the path to the `data` directory inside the database dump.
pub fn get_data(
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
    ) = (
        load_vertices::<Category>(data_path, "categories"),
        load_vertices::<Crate>(data_path, "crates"),
        load_vertices::<Keyword>(data_path, "keywords"),
    );

    let versions_to_crates = create_versioned_crates(data_path, &mut crates, &crate_id_lookup);

    load_dependencies(
        data_path,
        &mut crates,
        &versions_to_crates,
        &crate_id_lookup,
    );

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

    alphabetize_crate_contents(&mut crates);

    println!(
        "Finished loading registry graph in {} seconds.",
        start.elapsed().as_secs_f64()
    );

    (categories, crates, keywords)
}

/// Loads vertices (categories, crates, keywords) from a CSV file in the database dump.
///
/// Returns a tuple containing a map from names to vertices and a map from SQL ids to names.
///
/// # Arguments
/// * `data_path` - the path to the `data` directory inside the database dump.
/// * `collection_name` - the name of the vertex collection.
fn load_vertices<T: DeserializeOwned + Vertex + Debug>(
    data_path: &str,
    collection_name: &str,
) -> (HashMap<String, T>, HashMap<usize, String>) {
    println!("Loading {}...", collection_name);
    let start = Instant::now();
    let mut count = 0_usize;

    let file_path = get_collection_path(data_path, collection_name);

    // map names to objects
    let mut collection = HashMap::<String, T>::new();

    // map SQL ids to names
    let mut id_lookup = HashMap::<usize, String>::new();

    for result in Reader::from_reader(BufReader::new(
        File::open(Path::new(&file_path))
            .unwrap_or_else(|_| panic!("Unable to open {}", file_path)),
    ))
    .deserialize()
    {
        count += 1;

        let record: T = result.unwrap_or_else(|_| {
            panic!(
                "Unable to deserialize entry {} as {}",
                count,
                any::type_name::<T>()
            )
        });

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

/// Inserts the contents of one `Version` into another.
///
/// # Arguments
/// * `version` - the `Version` with the contents to insert.
/// * `other` - the `Version` to update.
fn replace_version(version: Version, other: &mut Version) {
    let Version {
        num,
        id,
        created_at,
        features,
        ..
    } = version;

    other.num = num;
    other.id = id;
    other.created_at = created_at;
    other.features = features;
}

/// Returns a map of SQL ids to versions.
///
/// Only the most recent stable version of each crate is kept, if a crate has a stable version.
/// If the crate does not have a stable version, then the most recent version is used.
///
/// # Arguments
/// * `data_path` - the path to the `data` directory inside the database dump.
fn get_versions(data_path: &str) -> HashMap<usize, Version> {
    println!("Loading versions...");
    let start = Instant::now();
    let mut versions = HashMap::<usize, Version>::new();
    let mut count = 0_usize;
    let filename = get_collection_path(data_path, "versions");

    for result in Reader::from_reader(BufReader::new(
        File::open(Path::new(&filename)).unwrap_or_else(|_| panic!("Unable to open {}", filename)),
    ))
    .deserialize()
    {
        count += 1;
        let version: Version =
            result.unwrap_or_else(|_| panic!("Unable to deserialize entry {} as Version", count));

        let Version {
            num,
            created_at,
            crate_id,
            ..
        } = &version;

        let version_clone = version.clone();

        versions
            .entry(*crate_id)
            .and_modify(|existing_version| {
                // if the crate has a version already

                if let Ok(version_num) = semver_version::parse(num.as_str()) {
                    if let Ok(existing_version_num) =
                        semver_version::parse(existing_version.num.as_str())
                    {
                        // if both versions are SemVer adherent

                        let version_is_pre = version.is_pre();
                        let existing_version_is_pre = existing_version.is_pre();

                        if !version_is_pre && existing_version_is_pre // if is stable and existing one isn't
                            || (version_is_pre == existing_version_is_pre // otherwise if the two are the same and the current one is a newer release
                                && (version_num.major > existing_version_num.major
                                    || (version_num.major == existing_version_num.major
                                        && version_num.minor > existing_version_num.minor)
                                    || (version_num.major == existing_version_num.major
                                        && version_num.minor == existing_version_num.minor
                                        && version_num.patch > existing_version_num.patch)))
                        {
                            replace_version(version_clone, existing_version);
                        }
                    } else {
                        // if existing version is not SemVer adherent but current one is
                        replace_version(version_clone, existing_version);
                    }
                } else if semver_version::parse(existing_version.num.as_str()).is_err()
                    && created_at.cmp(&existing_version.created_at) == Ordering::Greater
                {
                    // if both are not SemVer adherent and current was created more recent
                    replace_version(version_clone, existing_version);
                }
            })
            .or_insert(version);
    }

    println!(
        "Processed {} versions in {} seconds.",
        count,
        start.elapsed().as_secs_f64()
    );

    versions
}

/// Assign versions to crates, returning a map of version ids to crate names.
///
/// # Arguments
/// * `data_path` - the path to the `data` directory inside the database dump.
/// * `crates` - a map of crate names to values.
/// * `crate_id_lookup` - a map of crate SQL ids to names.
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
        features,
        id,
        num,
    } in versions.values()
    {
        let crate_id = crate_id_lookup
            .get(crate_id)
            .unwrap_or_else(|| panic!("Crate with SQL id {} does not exist", crate_id));

        let version_crate = crates
            .get_mut(crate_id)
            .unwrap_or_else(|| panic!("Crate with id {} does not exist", crate_id));

        version_crate.created_at = created_at.to_owned();
        version_crate.features = serde_json::from_str(features)
            .unwrap_or_else(|_| panic!("Unable to deserialize {} as HashMap", features));
        version_crate.version = num.to_owned();

        version_to_crates.insert(id.to_owned(), crate_id.to_owned());
    }

    println!(
        "Created versioned crates in {} seconds.",
        start.elapsed().as_secs_f64()
    );

    version_to_crates
}

/// Loads dependencies from a crates.io database dump.
///
/// # Arguments
/// * `data_path` - the path to the `data` directory inside the database dump.
/// * `crates` - a map of crate names to values.
/// * `versions_to_crates` - a map of version ids to crate names.
/// * `crate_id_lookup` - a map of crate SQL ids to names.
fn load_dependencies(
    data_path: &str,
    crates: &mut HashMap<String, Crate>,
    versions_to_crates: &HashMap<usize, String>,
    crate_id_lookup: &HashMap<usize, String>,
) {
    println!("Loading dependencies...");
    let start = Instant::now();
    let mut count = 0_usize;
    let dependencies_path = get_collection_path(data_path, "dependencies");

    for result in Reader::from_reader(BufReader::new(
        File::open(Path::new(&dependencies_path))
            .unwrap_or_else(|_| panic!("Unable to open {}", dependencies_path)),
    ))
    .deserialize()
    {
        let sql_dependency: SqlDependency = result
            .unwrap_or_else(|_| panic!("Unable to deserialize entry {} as Dependency", count));
        let SqlDependency {
            default_features,
            features,
            kind,
            optional,
            target,
            version_id,
            ..
        } = sql_dependency;

        let sql_dependency_crate_id = sql_dependency.crate_id;

        if let Some(from) = versions_to_crates.get(&version_id) {
            if kind == 0 {
                count += 1;

                crates
                    .get_mut(from)
                    .unwrap_or_else(|| panic!("Crate with id {} not found", from))
                    .dependencies
                    .push(Dependency {
                        default_features: default_features == "t",
                        features: String::from(&features[1..features.len() - 1]) // convert brace array to array ({a, b, c} => [a, b, c])
                            .split(',')
                            .filter_map(|split| {
                                if split.is_empty() {
                                    None
                                } else {
                                    Some(String::from(split))
                                }
                            })
                            .collect(),
                        from: from.to_owned(),
                        optional: optional == "t",
                        target: if target.is_empty() {
                            None
                        } else {
                            Some(target)
                        },
                        to: crate_id_lookup
                            .get(&sql_dependency_crate_id)
                            .unwrap_or_else(|| {
                                panic!("Crate with id {} not found", sql_dependency_crate_id)
                            })
                            .to_owned(),
                    });
            }
        }
    }
    println!(
        "Loaded {} dependencies into database in {} seconds.",
        count,
        start.elapsed().as_secs_f64()
    );
}

/// Loads crate-category relationships from a crates.io database dump.
///
/// # Arguments
/// * `data_path` - the path to the `data` directory inside the database dump.
/// * `crates` - a map of crate names to values.
/// * `categories` - a map of category names to values.
/// * `crate_id_lookup` - a map of crate SQL ids to names.
/// * `category_id_lookup` - a map of category SQL ids to names.
fn load_crate_categories(
    data_path: &str,
    crates: &mut HashMap<String, Crate>,
    categories: &mut HashMap<String, Category>,
    crate_id_lookup: &HashMap<usize, String>,
    category_id_lookup: &HashMap<usize, String>,
) {
    println!("Loading crate categories...");
    let start = Instant::now();
    let mut count = 0_usize;

    let file_path = get_collection_path(data_path, "crates_categories");

    for result in Reader::from_reader(BufReader::new(
        File::open(Path::new(&file_path))
            .unwrap_or_else(|_| panic!("Unable to open {}", file_path)),
    ))
    .deserialize()
    {
        count += 1;
        let CrateCategory {
            category_id,
            crate_id,
        } = result
            .unwrap_or_else(|_| panic!("Unable to deserialize entry {} as crate category", count));

        let category_id = category_id_lookup
            .get(&category_id)
            .unwrap_or_else(|| panic!("Category with id {} not found", category_id));

        let crate_id = crate_id_lookup
            .get(&crate_id)
            .unwrap_or_else(|| panic!("Crate with id {} not found", crate_id));

        crates
            .get_mut(crate_id)
            .unwrap_or_else(|| panic!("Crate with id {} not found", crate_id))
            .categories
            .push(category_id.to_owned());

        categories
            .get_mut(category_id)
            .unwrap_or_else(|| panic!("Category with id {} not found", category_id))
            .crates
            .push(crate_id.to_owned());
    }

    println!(
        "Loaded {} crate categories in {} seconds.",
        count,
        start.elapsed().as_secs_f64()
    );
}

/// Loads crate-keywords relationships from a crates.io database dump.
///
/// # Arguments
/// * `data_path` - the path to the `data` directory inside the database dump.
/// * `crates` - a map of crate names to values.
/// * `keywords` - a map of keyword names to values.
/// * `crate_id_lookup` - a map of crate SQL ids to names.
/// * `keyword_id_lookup` - a map of keyword SQL ids to names.
fn load_crate_keywords(
    data_path: &str,
    crates: &mut HashMap<String, Crate>,
    keywords: &mut HashMap<String, Keyword>,
    crate_id_lookup: &HashMap<usize, String>,
    keyword_id_lookup: &HashMap<usize, String>,
) {
    println!("Loading crate keywords...");
    let start = Instant::now();
    let mut count = 0_usize;

    let file_path = get_collection_path(data_path, "crates_keywords");

    for result in Reader::from_reader(BufReader::new(
        File::open(Path::new(&file_path))
            .unwrap_or_else(|_| panic!("Unable to open {}", file_path)),
    ))
    .deserialize()
    {
        count += 1;
        let CrateKeyword {
            crate_id,
            keyword_id,
        } = result
            .unwrap_or_else(|_| panic!("Unable to deserialize entry {} as crate keyword", count));

        let crate_id = crate_id_lookup
            .get(&crate_id)
            .unwrap_or_else(|| panic!("Unable to find crate with id {}", crate_id));

        let keyword_id = keyword_id_lookup
            .get(&keyword_id)
            .unwrap_or_else(|| panic!("Unable to find keyword with id {}", keyword_id));

        crates
            .get_mut(crate_id)
            .unwrap_or_else(|| panic!("Crate with id {} not found", crate_id))
            .keywords
            .push(keyword_id.to_owned());

        keywords
            .get_mut(keyword_id)
            .unwrap_or_else(|| panic!("Keyword with id {} not found", keyword_id))
            .crates
            .push(crate_id.to_owned());
    }

    println!(
        "Loaded {} crate keywords in {} seconds.",
        count,
        start.elapsed().as_secs_f64()
    );
}

/// Alphabetize crate category, dependency, and keyword lists.
///
/// # Arguments
/// * `crates` - a map of crate names to values.
fn alphabetize_crate_contents(crates: &mut HashMap<String, Crate>) {
    println!("Alphabetizing crate contents...");
    let start = Instant::now();

    for crate_val in crates.values_mut() {
        crate_val.categories.sort_unstable();
        crate_val.keywords.sort_unstable();
        crate_val
            .dependencies
            .sort_unstable_by_key(|dependency| dependency.to.to_owned());
    }

    println!(
        "Alphabetized crate contents in {} seconds.",
        start.elapsed().as_secs_f64()
    );
}
