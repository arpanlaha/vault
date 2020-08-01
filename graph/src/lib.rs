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

#[derive(Serialize)]
pub struct CrateDistance<'a> {
    /// A list of categories the crate belongs to.
    pub categories: &'a Vec<String>,

    /// The time at which the most recent stable version (if available) of the crate was created.
    pub created_at: &'a NaiveDateTime,

    /// The description of the crate.
    pub description: &'a String,

    pub distance: usize,

    /// The number of downloads of the crate.
    pub downloads: &'a usize,

    /// The keywords belonging to the crate.
    pub keywords: &'a Vec<String>,

    /// The name of the crate.
    pub name: &'a String,

    /// The most recent stable version (if available) of the crate.
    pub version: &'a String,
}

impl<'a> CrateDistance<'a> {
    #[must_use]
    pub fn new(
        crate_distance_tuple: &(&&String, &usize),
        crates: &'a HashMap<String, Crate>,
    ) -> CrateDistance<'a> {
        let (crate_id, crate_distance) = *crate_distance_tuple;
        let crate_val = crates.get(crate_id.as_str()).unwrap();
        let Crate {
            categories,
            created_at,
            description,
            downloads,
            keywords,
            name,
            version,
            ..
        } = &crate_val;

        CrateDistance {
            categories,
            created_at,
            description,
            distance: *crate_distance,
            downloads,
            keywords,
            name,
            version,
        }
    }
}

#[derive(Serialize)]
pub struct DependencyGraph<'a> {
    pub crates: Vec<CrateDistance<'a>>,
    pub dependencies: Vec<&'a Dependency>,
}

struct QueueDependency {
    pub from: String,
    pub to: String,
    pub to_feature_names: Vec<String>,
    pub to_distance: usize,
}

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

pub struct Graph {
    categories: HashMap<String, Category>,
    category_names: BTreeSet<String>,
    crates: HashMap<String, Crate>,
    crate_names: BTreeSet<String>,
    keywords: HashMap<String, Keyword>,
    keyword_names: BTreeSet<String>,
    last_updated: Instant,
}

impl Graph {
    pub async fn new() -> Self {
        let temp_dir = fs::fetch_data();

        let data_path = fs::get_data_path(&temp_dir).unwrap();

        let (categories, crates, keywords) = load::get_data(data_path.as_str()).await;
        fs::clean_tempdir(temp_dir);

        let mut crate_names: BTreeSet<String> = BTreeSet::new();

        for crate_name in crates.keys() {
            crate_names.insert(crate_name.to_owned());
        }

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

        let mut crate_names: BTreeSet<String> = BTreeSet::new();

        for crate_name in crates.keys() {
            crate_names.insert(crate_name.to_owned());
        }

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

        let mut crate_names: BTreeSet<String> = BTreeSet::new();

        for crate_name in crates.keys() {
            crate_names.insert(crate_name.to_owned());
        }

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

    pub fn update_time(&mut self) {
        self.last_updated = Instant::now();
    }

    pub fn replace(&mut self, other: Self) {
        self.categories = other.categories;
        self.crates = other.crates;
        self.keywords = other.keywords;
        self.last_updated = Instant::now();
    }

    #[must_use]
    pub const fn categories(&self) -> &HashMap<String, Category> {
        &self.categories
    }

    #[must_use]
    pub const fn crates(&self) -> &HashMap<String, Crate> {
        &self.crates
    }

    #[must_use]
    pub const fn keywords(&self) -> &HashMap<String, Keyword> {
        &self.keywords
    }

    #[must_use]
    pub const fn category_names(&self) -> &BTreeSet<String> {
        &self.category_names
    }

    #[must_use]
    pub const fn crate_names(&self) -> &BTreeSet<String> {
        &self.crate_names
    }

    #[must_use]
    pub const fn keyword_names(&self) -> &BTreeSet<String> {
        &self.keyword_names
    }

    // #[must_use]
    // pub fn search_crates(&self, search_term: &str) -> VecDeque<&Crate> {
    //     if search_term.is_empty() {
    //         VecDeque::new()
    //     } else {
    //         let mut range_end = String::from(search_term);
    //         let to_push = (range_end.pop().unwrap() as u8 + 1) as char;
    //         range_end.push(to_push);

    //         let prefixed_crate_names = self
    //             .crate_names
    //             .range::<String, _>((Included(&String::from(search_term)), Excluded(&range_end)));

    //         let mut search_results: VecDeque<&Crate> =
    //             VecDeque::with_capacity(MAX_SEARCH_LENGTH + 1);

    //         for prefixed_crate_name in prefixed_crate_names {
    //             if prefixed_crate_name != search_term {
    //                 let prefixed_crate = self.crates.get(prefixed_crate_name).unwrap();
    //                 if search_results.is_empty() {
    //                     search_results.push_back(prefixed_crate);
    //                 } else {
    //                     if should_replace(prefixed_crate, search_results.back().unwrap()) {
    //                         if let Some((index, _)) =
    //                             search_results
    //                                 .iter()
    //                                 .enumerate()
    //                                 .find(|(_, results_crate)| {
    //                                     should_replace(prefixed_crate, results_crate)
    //                                 })
    //                         {
    //                             search_results.insert(index, prefixed_crate);

    //                             if search_results.len() > MAX_SEARCH_LENGTH {
    //                                 search_results.pop_back();
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }

    //         if let Some(search_crate) = self.crates.get(search_term) {
    //             search_results.push_front(search_crate);
    //             if search_results.len() > MAX_SEARCH_LENGTH {
    //                 search_results.pop_back();
    //             }
    //         }

    //         search_results

    //         // let in_range: Vec<&String> = self
    //         //     .crate_names
    //         //     .range::<String, _>((Included(&search_term), Excluded(&range_end)))
    //         //     .collect();

    //         // if in_range.is_empty() {
    //         //     return vec![];
    //         // }

    //         // in_range
    //         //     .iter()
    //         //     .filter(|crate_name| {
    //         //         crate_name.len()
    //         //             <= in_range
    //         //                 .get(10)
    //         //                 .unwrap_or_else(|| in_range.last().unwrap())
    //         //                 .len()
    //         //     })
    //         //     .map(|&crate_name| crate_name)
    //         //     .collect()
    //     }
    // }

    // pub fn set_categories(&mut self, categories: HashMap<String, Category>) {
    //     self.categories = categories;
    // }

    // pub fn set_crates(&mut self, crates: HashMap<String, Crate>) {
    //     self.crates = crates;
    // }

    // pub fn set_keywords(&mut self, keywords: HashMap<String, Keyword>) {
    //     self.keywords = keywords;
    // }

    #[must_use]
    pub fn time_since_last_update(&self) -> u64 {
        self.last_updated.elapsed().as_secs()
    }

    #[must_use]
    pub fn get_dependency_graph(
        &self,
        crate_id: &str,
        features: Vec<String>,
    ) -> Option<DependencyGraph> {
        match self.crates().get(crate_id) {
            None => None,
            Some(crate_val) => {
                let mut crate_distance_vec: Vec<(&String, usize)> = vec![];
                let mut crates_seen: HashMap<&String, Vec<String>> = HashMap::new();

                let mut dependencies: Vec<&Dependency> = vec![];
                let mut dependencies_seen: HashSet<(String, String)> = HashSet::new();
                let mut dependency_queue: VecDeque<QueueDependency> = VecDeque::new();

                crate_distance_vec.push((&crate_val.name, 0));
                crates_seen.insert(&crate_val.name, features.to_owned());

                dependency_graph_helper(crate_val, features, &mut dependency_queue, 0);

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
                        let is_feature_unseen = |dependency_feature_name| {
                            !crate_feature_names.contains(dependency_feature_name)
                        };

                        if to_feature_names.iter().any(is_feature_unseen) {
                            dependency_graph_helper(
                                to_crate_val,
                                to_feature_names,
                                &mut dependency_queue,
                                to_distance,
                            );
                        }
                    } else {
                        crate_distance_vec.push((&to_crate_val.name, to_distance));
                        crates_seen.insert(&to_crate_val.name, to_feature_names.to_owned());
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
                            CrateDistance::new(&(crate_name, crate_distance), &self.crates)
                        })
                        .collect(),
                    dependencies,
                })
            }
        }
    }
}

fn get_names<T>(collection: &HashMap<String, T>) -> BTreeSet<String> {
    let mut names: BTreeSet<String> = BTreeSet::new();

    for name in collection.keys() {
        names.insert(name.to_owned());
    }

    names
}

// fn should_replace(a: &Crate, b: &Crate) -> bool {
//     a.downloads > b.downloads || (a.downloads == b.downloads && a.name.len() < b.name.len())
// }
