#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::cast_precision_loss)]

mod fs;
mod load;
mod schema;
mod traits;

use ahash::{AHashMap, AHashSet};
use cargo_platform::{Cfg, Platform};
use chrono::NaiveDateTime;
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs::File,
    process::Command,
    str::{self, FromStr},
    time::Instant,
};

pub use schema::{Category, Crate, Dependency, Keyword};
pub use traits::{Random, Search};

/// A struct containing information about the crates.io registry.
pub struct Graph {
    /// A mapping of `Category` names to values.
    categories: AHashMap<String, Category>,

    /// A set of `Category` names for searching.
    category_names: BTreeSet<String>,

    /// A set of cfg names (e.g. `unix`, `cargo_web`) present among all dependencies.
    cfg_names: BTreeSet<String>,

    /// A mapping of `Crate` names to values.
    crates: AHashMap<String, Crate>,

    /// A set of `Crate` names for searching.
    crate_names: BTreeSet<String>,

    /// A set of `Keyword` names for searching.
    keywords: AHashMap<String, Keyword>,

    /// A set of `Keyword` names for searching.
    keyword_names: BTreeSet<String>,

    /// The time at which the `Graph` was last updated.
    last_updated: Instant,

    /// A mapping of rustc-supported targets to cfg attributes.
    targets: BTreeMap<String, Vec<Cfg>>,
}

impl Graph {
    /// Creates a new `Graph`.
    ///
    /// This pulls in the latest crates.io dump and is intended for production use.
    #[must_use]
    pub fn new() -> Self {
        let temp_dir = fs::fetch_data();

        let data_path = fs::get_data_path(&temp_dir).unwrap();

        let (categories, crates, keywords) = load::get_data(data_path.as_str());
        fs::clean_tempdir(temp_dir);

        Self {
            category_names: get_names(&categories),
            categories,
            cfg_names: get_cfg_names(&crates),
            crate_names: get_names(&crates),
            crates,
            keyword_names: get_names(&keywords),
            keywords,
            last_updated: Instant::now(),
            targets: vault_targets::get_targets(),
        }
    }

    /// Creates a new `Graph`.
    ///
    /// This uses a saved backup dump of the crates.io registry and is intended for testing.
    #[must_use]
    pub fn test() -> Self {
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

        let (categories, crates, keywords) = load::get_data(data_path);

        Self {
            category_names: get_names(&categories),
            categories,
            cfg_names: get_cfg_names(&crates),
            crate_names: get_names(&crates),
            crates,
            keyword_names: get_names(&keywords),
            keywords,
            last_updated: Instant::now(),
            targets: vault_targets::get_targets(),
        }
    }

    /// Updates the `last_updated` time to the current time.
    pub fn update_time(&mut self) {
        self.last_updated = Instant::now();
    }

    /// Returns an immutable reference to the `Category` map.
    #[must_use]
    pub const fn categories(&self) -> &AHashMap<String, Category> {
        &self.categories
    }

    /// Returns an immutable reference to the set of cfg names.
    #[must_use]
    pub const fn cfg_names(&self) -> &BTreeSet<String> {
        &self.cfg_names
    }

    /// Returns an immutable reference to the `Crate` map.
    #[must_use]
    pub const fn crates(&self) -> &AHashMap<String, Crate> {
        &self.crates
    }

    /// Returns an immutable reference to the `Keyword` map.
    #[must_use]
    pub const fn keywords(&self) -> &AHashMap<String, Keyword> {
        &self.keywords
    }

    /// Returns an immutable reference to the `Category` name set.
    #[must_use]
    pub const fn category_names(&self) -> &BTreeSet<String> {
        &self.category_names
    }

    /// Returns an immutable reference to the `Crate` name set.
    #[must_use]
    pub const fn crate_names(&self) -> &BTreeSet<String> {
        &self.crate_names
    }

    /// Returns an immutable reference to the `Keyword` name set.
    #[must_use]
    pub const fn keyword_names(&self) -> &BTreeSet<String> {
        &self.keyword_names
    }

    /// Returns an immutable reference to the map from targets to cfg attributes.
    #[must_use]
    pub const fn targets(&self) -> &BTreeMap<String, Vec<Cfg>> {
        &self.targets
    }

    /// Returns the time since the `Graph` was last updated in seconds.
    #[must_use]
    pub fn time_since_last_update(&self) -> u64 {
        self.last_updated.elapsed().as_secs()
    }

    /// Returns the dependency graph of the specified crate with the specified features enabled.
    ///
    /// If no crate matches the specified name, returns `None`.
    ///
    /// # Arguments
    /// * `crate_id` - the name of the crate to analyze.
    /// * `features` - the list of features to enable.
    #[must_use]
    pub fn get_dependency_graph(
        &self,
        crate_id: &str,
        mut features: Vec<String>,
        target: &Option<String>,
        cfg_name: &Option<String>,
    ) -> Option<DependencyGraph> {
        match self.crates().get(crate_id) {
            None => None,

            Some(crate_val) => {
                // a list of crate names and distances from the root crate
                let mut crate_distance_vec: Vec<(&String, usize)> = vec![];
                // a map of crates seen and which features have already been enabled for them
                let mut crates_seen: AHashMap<&String, Vec<String>> = AHashMap::new();

                // a list of dependencies to return
                let mut dependencies: Vec<&Dependency> = vec![];
                // a set of dependencies seen so far by source and destination name
                let mut dependencies_seen: AHashSet<(String, String)> = AHashSet::new();
                // the queue of dependnencies to process.
                let mut dependency_queue: VecDeque<QueueDependency> = VecDeque::new();

                let target = String::from(match target {
                    Some(target) => target,
                    None => "x86_64-unknown-linux-gnu",
                });

                let cfg_name = Cfg::from_str(match cfg_name {
                    Some(cfg_name) => cfg_name.as_str(),
                    None => "unix",
                })
                .unwrap();

                // insert the root crate
                crate_distance_vec.push((&crate_val.name, 0));
                crates_seen.insert(&crate_val.name, features.to_owned());
                features.push(String::from("default"));

                // add root crate dependendencies to the queue
                self.dependency_graph_helper(
                    crate_val,
                    features,
                    &mut dependency_queue,
                    0,
                    &target,
                    &cfg_name,
                );

                // while the queue is not empty
                while let Some(QueueDependency {
                    from,
                    to,
                    mut to_feature_names,
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

                        // remove already seen features
                        to_feature_names.retain(|dependency_feature_name| {
                            !crate_feature_names.contains(dependency_feature_name)
                        });

                        if !to_feature_names.is_empty() {
                            // but has features that haven't been enabled yet
                            // add dependencies to queue
                            self.dependency_graph_helper(
                                to_crate_val,
                                to_feature_names.clone(),
                                &mut dependency_queue,
                                to_distance,
                                &target,
                                &cfg_name,
                            );
                        }

                        crate_feature_names.append(&mut to_feature_names);
                    } else {
                        // add crate to list and map
                        crate_distance_vec.push((&to_crate_val.name, to_distance));
                        crates_seen.insert(&to_crate_val.name, to_feature_names.to_owned());

                        // add crate dependencies to queue
                        self.dependency_graph_helper(
                            to_crate_val,
                            to_feature_names,
                            &mut dependency_queue,
                            to_distance,
                            &target,
                            &cfg_name,
                        );
                    }
                }

                Some(DependencyGraph {
                    crates: crate_distance_vec
                        .iter()
                        .map(|&(crate_id, distance)| {
                            let mut enabled_features =
                                crates_seen.get(crate_id).unwrap().to_owned();
                            enabled_features.retain(|feature_name| feature_name != "default");

                            CrateDistance::new(
                                CrateDistanceInfo {
                                    crate_id,
                                    distance,
                                    enabled_features,
                                },
                                &self.crates,
                            )
                        })
                        .collect(),
                    dependencies,
                })
            }
        }
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
        &self,
        crate_val: &Crate,
        mut feature_names: Vec<String>,
        dependency_queue: &mut VecDeque<QueueDependency>,
        distance: usize,
        target: &str,
        cfg_name: &Cfg,
    ) {
        // dependencies included in traversal
        let mut dependencies_to_check: BTreeMap<String, Vec<String>> = BTreeMap::new();

        // add mandatory dependencies
        for dependency in &crate_val.dependencies {
            if !dependency.optional {
                dependencies_to_check
                    .insert(dependency.to.to_owned(), dependency.features.to_owned());
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
                        let feature_dependency_name =
                            String::from(&feature_dependency[..slash_index]);

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
            let mut target_supported = true;

            if let Some(dependency) = crate_val
                .dependencies
                .iter()
                .find(|dependency| dependency.to == dependency_name)
            {
                if dependency.default_features {
                    dependency_features.push(default_string.to_owned());
                }

                if let Some(dependency_target) = &dependency.target {
                    if let Ok(dependency_platform) = Platform::from_str(dependency_target) {
                        let mut cfg_attributes = self.targets.get(target).unwrap().to_owned();

                        cfg_attributes.push(cfg_name.to_owned());

                        target_supported =
                            dependency_platform.matches(target, cfg_attributes.as_slice());
                    }
                }
            }

            if target_supported {
                dependency_queue.push_back(QueueDependency {
                    from: crate_val.name.to_owned(),
                    to: dependency_name,
                    to_feature_names: dependency_features,
                    to_distance: distance + 1,
                });
            }
        }
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates a set of names from a `AHashMap`.
///
/// # Arguments
/// * `collection` - the collection to convert.
fn get_names<T>(collection: &AHashMap<String, T>) -> BTreeSet<String> {
    collection.keys().cloned().collect()
}

/// Returns a set of cfg names (e.g. `unix`, `cargo_web`) present among all dependencies.
fn get_cfg_names(crates: &AHashMap<String, Crate>) -> BTreeSet<String> {
    println!("Collecting cfg names...");
    let start = Instant::now();

    let mut cfg_names: BTreeSet<String> = BTreeSet::new();

    for crate_val in crates.values() {
        for dependency in &crate_val.dependencies {
            if let Some(target) = &dependency.target {
                if target.matches('(').count() == 1
                    && target.matches(')').count() == 1
                    && !target.contains('=')
                    && target != "cfg(test)"
                    && target != "cfg(proc_macro)"
                    && target != "cfg(debug_assertions)"
                {
                    cfg_names.insert(String::from(
                        &target[target.find('(').unwrap() + 1..target.find(')').unwrap()],
                    ));
                }
            }
        }
    }

    println!(
        "Finished collecting cfg names in {} seconds.",
        start.elapsed().as_secs_f64()
    );

    cfg_names
}

#[derive(Serialize)]
/// A Crate intended for serialization, including the distance from the root crate and enabled features.
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

    /// The features enabled for this crate in this `DependencyGraph`.
    pub enabled_features: Vec<String>,

    /// The features exposed by the crate.
    pub features: &'a BTreeMap<String, Vec<String>>,

    /// The keywords belonging to the crate.
    pub keywords: &'a Vec<String>,

    /// The name of the crate.
    pub name: &'a String,

    /// The most recent stable version (if available) of the crate.
    pub version: &'a String,
}

pub struct CrateDistanceInfo<'a> {
    pub crate_id: &'a String,
    pub distance: usize,
    pub enabled_features: Vec<String>,
}

impl<'a> CrateDistance<'a> {
    /// Creates a new `CrateDistance`.
    ///
    /// # Arguments
    /// * `crate_distance_info` - a the `CrateDistanceInfo` containing the relevant information.
    /// * `crates` - the `AHashMap` containing the crate values.
    #[must_use]
    pub fn new(
        crate_distance_info: CrateDistanceInfo<'a>,
        crates: &'a AHashMap<String, Crate>,
    ) -> CrateDistance<'a> {
        let CrateDistanceInfo {
            crate_id,
            distance,
            enabled_features,
        } = crate_distance_info;

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
            enabled_features,
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
