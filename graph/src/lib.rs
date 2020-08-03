#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::cast_precision_loss)]

mod fs;
mod load;
mod schema;
mod traits;

use chrono::NaiveDateTime;
pub use schema::{Category, Crate, Dependency, Keyword};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::process::Command;
use std::time::{Duration, Instant};
pub use traits::{Random, Search};

const DAY_SECONDS: u64 = 60 * 60 * 24;

/// A struct containing information about the crates.io registry.
pub struct Graph {
    /// A mapping of `Category` names to values.
    categories: HashMap<String, Category>,

    /// A set of `Category` names for searching.
    category_names: BTreeSet<String>,

    /// A mapping of `Crate` names to values.
    crates: HashMap<String, Crate>,

    /// A set of `Crate` names for searching.
    crate_names: BTreeSet<String>,

    /// A set of `Keyword` names for searching.
    keywords: HashMap<String, Keyword>,

    /// A set of `Keyword` names for searching.
    keyword_names: BTreeSet<String>,

    /// The time at which the `Graph` was last updated.
    last_updated: Instant,
}

impl Graph {
    /// Creates a new `Graph`.
    ///
    /// This pulls in the latest crates.io dump and is intended for production use.
    pub async fn new() -> Self {
        let temp_dir = fs::fetch_data();

        let data_path = fs::get_data_path(&temp_dir).unwrap();

        let (categories, crates, keywords) = load::get_data(data_path.as_str()).await;
        fs::clean_tempdir(temp_dir);

        Self {
            category_names: get_names(&categories),
            categories,
            crate_names: get_names(&crates),
            crates,
            keyword_names: get_names(&keywords),
            keywords,
            last_updated: Instant::now(),
        }
    }

    /// Creates a new `Graph`.
    ///
    /// This uses a saved backup dump of the crates.io registry and is intended for testing.
    pub async fn test() -> Self {
        let data_path = "./tests/data";

        if File::open(data_path).is_err() {
            Command::new("tar")
                .arg("-xzf")
                .arg("./tests/data.tar.gz")
                .arg("-C")
                .arg("tests")
                .output()
                .unwrap();
        }

        let (categories, crates, keywords) = load::get_data(data_path).await;

        Self {
            category_names: get_names(&categories),
            categories,
            crate_names: get_names(&crates),
            crates,
            keyword_names: get_names(&keywords),
            keywords,
            last_updated: Instant::now(),
        }
    }

    /// Creates a new `Graph`.
    ///
    /// This uses a saved backup dump of the crates.io registry and is intended for testing.
    ///
    /// This produces the same result as the `test` constructor, but with the `last_updated` field being set to a day prior.
    pub async fn yesterday() -> Self {
        let data_path = "./tests/data";

        if File::open(data_path).is_err() {
            Command::new("tar")
                .arg("-xzf")
                .arg("./tests/data.tar.gz")
                .arg("-C")
                .arg("tests")
                .output()
                .unwrap();
        }

        let (categories, crates, keywords) = load::get_data(data_path).await;

        Self {
            category_names: get_names(&categories),
            categories,
            crate_names: get_names(&crates),
            crates,
            keyword_names: get_names(&keywords),
            keywords,
            last_updated: Instant::now() - Duration::from_secs(DAY_SECONDS),
        }
    }

    /// Updates the `last_updated` time to the current time.
    pub fn update_time(&mut self) {
        self.last_updated = Instant::now();
    }

    /// Replaces the contents of this `Graph` with the contents of the other `Graph`.
    pub fn replace(&mut self, other: Self) {
        self.categories = other.categories;
        self.crates = other.crates;
        self.keywords = other.keywords;
        self.last_updated = Instant::now();
    }

    #[must_use]
    /// Returns an immutable reference to the `Category` map.
    pub const fn categories(&self) -> &HashMap<String, Category> {
        &self.categories
    }

    #[must_use]
    /// Returns an immutable reference to the `Crate` map.
    pub const fn crates(&self) -> &HashMap<String, Crate> {
        &self.crates
    }

    #[must_use]
    /// Returns an immutable reference to the `Keyword` map.
    pub const fn keywords(&self) -> &HashMap<String, Keyword> {
        &self.keywords
    }

    #[must_use]
    /// Returns an immutable reference to the `Category` name set.
    pub const fn category_names(&self) -> &BTreeSet<String> {
        &self.category_names
    }

    #[must_use]
    /// Returns an immutable reference to the `Crate` name set.
    pub const fn crate_names(&self) -> &BTreeSet<String> {
        &self.crate_names
    }

    #[must_use]
    /// Returns an immutable reference to the `Keyword` name set.
    pub const fn keyword_names(&self) -> &BTreeSet<String> {
        &self.keyword_names
    }

    #[must_use]
    /// Returns the time since the `Graph` was last updated in seconds.
    pub fn time_since_last_update(&self) -> u64 {
        self.last_updated.elapsed().as_secs()
    }

    #[must_use]
    /// Returns the dependency graph of the specified crate with the specified features enabled.
    ///
    /// If no crate matches the specified name, returns `None`.
    ///
    /// # Arguments
    /// * `crate_id` - the name of the crate to analyze.
    /// * `features` - the list of features to enable.
    pub fn get_dependency_graph(
        &self,
        crate_id: &str,
        features: Vec<String>,
    ) -> Option<DependencyGraph> {
        match self.crates().get(crate_id) {
            None => None,

            Some(crate_val) => {
                // a list of crate names and distances from the root crate
                let mut crate_distance_vec: Vec<(&String, usize)> = vec![];
                // a map of crates seen and which features have already been enabled for them
                let mut crates_seen: HashMap<&String, Vec<String>> = HashMap::new();

                // a list of dependencies to return
                let mut dependencies: Vec<&Dependency> = vec![];
                // a set of dependencies seen so far by source and destination name
                let mut dependencies_seen: HashSet<(String, String)> = HashSet::new();
                // the queue of dependnencies to process.
                let mut dependency_queue: VecDeque<QueueDependency> = VecDeque::new();

                // insert the root crate
                crate_distance_vec.push((&crate_val.name, 0));
                crates_seen.insert(&crate_val.name, features.to_owned());

                // add root crate dependendencies to the queue
                dependency_graph_helper(crate_val, features, &mut dependency_queue, 0);

                // while the queue is not empty
                while let Some(QueueDependency {
                    from,
                    to,
                    to_feature_names,
                    to_distance,
                }) = dependency_queue.pop_front()
                {
                    let from_crate_val = self.crates.get(&from).unwrap();
                    let to_crate_val = self.crates.get(&to).unwrap();
                    let dependency_tuple =
                        (from_crate_val.name.to_owned(), to_crate_val.name.to_owned());

                    // add dependency to list and set if not seen yet
                    if !dependencies_seen.contains(&dependency_tuple) {
                        dependencies.push(
                            from_crate_val
                                .dependencies
                                .iter()
                                .find(|dependency| dependency.to == to)
                                .unwrap(),
                        );

                        dependencies_seen.insert(dependency_tuple);
                    }

                    if let Some(crate_feature_names) = crates_seen.get_mut(&to_crate_val.name) {
                        // if crate has been seen

                        let is_feature_unseen = |dependency_feature_name| {
                            !crate_feature_names.contains(dependency_feature_name)
                        };

                        if to_feature_names.iter().any(is_feature_unseen) {
                            // but has features that haven't been enabled yet
                            // add dependencies to queue
                            dependency_graph_helper(
                                to_crate_val,
                                to_feature_names,
                                &mut dependency_queue,
                                to_distance,
                            );
                        }
                    } else {
                        // add crate to list and map
                        crate_distance_vec.push((&to_crate_val.name, to_distance));
                        crates_seen.insert(&to_crate_val.name, to_feature_names.to_owned());

                        // add crate dependencies to queue
                        dependency_graph_helper(
                            to_crate_val,
                            to_feature_names,
                            &mut dependency_queue,
                            to_distance,
                        );
                    }
                }

                Some(DependencyGraph {
                    crates: crate_distance_vec
                        .iter()
                        .map(|(crate_name, crate_distance)| {
                            CrateDistance::new((*crate_name, *crate_distance), &self.crates)
                        })
                        .collect(),
                    dependencies,
                })
            }
        }
    }
}

/// Creates a set of names from a `HashMap`.
///
/// # Arguments
/// * `collection` - the collection to convert.
fn get_names<T>(collection: &HashMap<String, T>) -> BTreeSet<String> {
    let mut names: BTreeSet<String> = BTreeSet::new();

    for name in collection.keys() {
        names.insert(name.to_owned());
    }

    names
}

#[derive(Serialize)]
/// A Crate intended for serialization, including the distance from the root crate.
pub struct CrateDistance<'a> {
    /// A list of categories the crate belongs to.
    pub categories: &'a Vec<String>,

    /// The time at which the most recent stable version (if available) of the crate was created.
    pub created_at: &'a NaiveDateTime,

    /// The description of the crate.
    pub description: &'a String,

    /// The distance from the root crate.
    pub distance: usize,

    /// The number of downloads of the crate.
    pub downloads: &'a usize,

    /// The features exposed by the crate.
    pub features: &'a HashMap<String, Vec<String>>,

    /// The keywords belonging to the crate.
    pub keywords: &'a Vec<String>,

    /// The name of the crate.
    pub name: &'a String,

    /// The most recent stable version (if available) of the crate.
    pub version: &'a String,
}

impl<'a> CrateDistance<'a> {
    #[must_use]
    /// Creates a new `CrateDistance`.
    ///
    /// # Arguments
    /// * `crate_distance_tuple` - a tuple containing the crate name and distance.
    /// * `crates` - the `HashMap` containing the crate values.
    pub fn new(
        crate_distance_tuple: (&String, usize),
        crates: &'a HashMap<String, Crate>,
    ) -> CrateDistance<'a> {
        let (crate_id, distance) = crate_distance_tuple;
        let Crate {
            categories,
            created_at,
            description,
            downloads,
            features,
            keywords,
            name,
            version,
            ..
        } = &crates.get(crate_id).unwrap();

        CrateDistance {
            categories,
            created_at,
            description,
            distance,
            downloads,
            features,
            keywords,
            name,
            version,
        }
    }
}

#[derive(Serialize)]
/// A dependency graph containing crates as nodes and dependencies as edges.
pub struct DependencyGraph<'a> {
    /// The list of crates included in the dependency graph.
    pub crates: Vec<CrateDistance<'a>>,

    /// The list of dependencies included in the dependency graph.
    pub dependencies: Vec<&'a Dependency>,
}

/// A struct containing information about a `Dependency` for processing in a queue to create a dependency graph.
struct QueueDependency {
    /// The source fof the `Dependency`.
    pub from: String,

    /// The destination of the `Dependency`.
    pub to: String,

    /// The list of features to included with the destination crate.
    pub to_feature_names: Vec<String>,

    /// The distance of the destination crate from the root.
    pub to_distance: usize,
}

/// A helper function to construct the dependency graph.
///
/// Adds all relevant dependencies of a crate into the dependency queue for processing.
///
/// # Arguments
/// * `crate_val` - the `Crate` being examined.
/// * `feature_names` - the list of enabled features.
/// * `dependency_queue` - the queue of dependencies to process.
/// * `distance` - the distance from the root crate.
fn dependency_graph_helper(
    crate_val: &Crate,
    mut feature_names: Vec<String>,
    dependency_queue: &mut VecDeque<QueueDependency>,
    distance: usize,
) {
    // dependencies included in traversal
    let mut dependencies_to_check: BTreeMap<String, Vec<String>> = BTreeMap::new();

    // add mandatory dependencies
    for dependency in &crate_val.dependencies {
        if !dependency.optional {
            dependencies_to_check.insert(dependency.to.to_owned(), dependency.features.to_owned());
        }
    }

    let default_string = String::from("default");
    let default_features_enabled = feature_names.contains(&default_string);
    let default_features = match crate_val.features.get(&default_string) {
        Some(default_features) => default_features.to_owned(),
        None => vec![],
    };

    // add dependencies enabled by features
    for (feature_name, feature_dependencies) in &crate_val.features {
        if feature_name != "default"
            && (feature_names.contains(feature_name)
                || (default_features_enabled && default_features.contains(feature_name)))
        {
            for feature_dependency in feature_dependencies {
                if crate_val.features.contains_key(feature_dependency) {
                    // if feature enables another feature
                    if !feature_names.contains(feature_dependency) {
                        // if the enabled feature is not already included
                        feature_names.push(feature_dependency.to_owned());
                    }
                } else if let Some(slash_index) = feature_dependency.find('/') {
                    // if features enabled
                    let feature_dependency_name = String::from(&feature_dependency[..slash_index]);

                    if crate_val
                        .dependencies
                        .iter()
                        .any(|dependency| dependency.to == feature_dependency_name)
                    {
                        let feature_dependency_transitive_feature =
                            String::from(&feature_dependency[slash_index + 1..]);

                        // if dependency already added, add feature if feature was not added
                        // otherwise add dependency and feature
                        dependencies_to_check
                            .entry(feature_dependency_name)
                            .and_modify(|dependency_feature_list| {
                                if !dependency_feature_list
                                    .contains(&feature_dependency_transitive_feature)
                                {
                                    dependency_feature_list
                                        .push(feature_dependency_transitive_feature);
                                }
                            })
                            .or_insert_with(|| {
                                vec![String::from(&feature_dependency[slash_index + 1..])]
                            });
                    }
                } else if crate_val
                    .dependencies
                    .iter()
                    .any(|dependency| dependency.to == *feature_dependency)
                {
                    // if features not enabled, insert dependency if not already present
                    dependencies_to_check
                        .entry(feature_dependency.to_owned())
                        .or_insert_with(Vec::new);
                }
            }
        }
    }

    for (dependency_name, mut dependency_features) in dependencies_to_check {
        if let Some(dependency) = crate_val
            .dependencies
            .iter()
            .find(|dependency| dependency.to == dependency_name)
        {
            if dependency.default_features {
                dependency_features.push(default_string.to_owned());
            }
        }
        dependency_queue.push_back(QueueDependency {
            from: crate_val.name.to_owned(),
            to: dependency_name,
            to_feature_names: dependency_features,
            to_distance: distance + 1,
        });
    }
}
