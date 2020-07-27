use super::super::ingest::{
    fs as vault_fs, load as vault_load,
    schema::{Category, Crate, Dependency, Keyword},
};
use chrono::NaiveDateTime;
use serde::Serialize;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::time::Instant;
use tokio::sync::{Mutex, RwLock};

pub struct AppState {
    pub graph: RwLock<Graph>,
    pub last_updated: Mutex<Instant>,
}

impl AppState {
    pub async fn new() -> AppState {
        AppState {
            graph: RwLock::new(Graph::new().await),
            last_updated: Mutex::new(Instant::now()),
        }
    }
}

pub struct Graph {
    categories: HashMap<String, Category>,
    crates: HashMap<String, Crate>,
    keywords: HashMap<String, Keyword>,
}

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
            features,
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
            features,
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
    feature_names: Vec<String>,
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
                if let Some(slash_index) = feature_dependency.find('/') {
                    // if features enabled
                    let feature_dependency_name = String::from(&feature_dependency[..slash_index]);
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
                                dependency_feature_list.push(feature_dependency_transitive_feature);
                            }
                        })
                        .or_insert(vec![String::from(&feature_dependency[slash_index + 1..])]);
                } else {
                    // if features not enabled, insert dependency if not already present
                    dependencies_to_check
                        .entry(feature_dependency.to_owned())
                        .or_insert(vec![]);
                }
            }
        }
    }

    for (dependency_name, dependency_features) in dependencies_to_check {
        dependency_queue.push_back(QueueDependency {
            from: crate_val.name.to_owned(),
            to: dependency_name,
            to_feature_names: dependency_features,
            to_distance: distance + 1,
        });
    }
}

impl Graph {
    pub async fn new() -> Graph {
        let temp_dir = vault_fs::fetch_data();

        let data_path = vault_fs::get_data_path(&temp_dir).unwrap();

        // let data_path = String::from("/datadrive/vault/dump/data");

        let (categories, crates, keywords) = vault_load::load_database(data_path.as_str()).await;
        vault_fs::clean_tempdir(temp_dir);

        Graph {
            categories,
            crates,
            keywords,
        }
    }

    pub fn replace(&mut self, other: Graph) {
        self.categories = other.categories;
        self.crates = other.crates;
        self.keywords = other.keywords;
    }

    pub fn categories(&self) -> &HashMap<String, Category> {
        &self.categories
    }

    pub fn crates(&self) -> &HashMap<String, Crate> {
        &self.crates
    }

    pub fn keywords(&self) -> &HashMap<String, Keyword> {
        &self.keywords
    }

    pub fn set_categories(&mut self, categories: HashMap<String, Category>) {
        self.categories = categories;
    }

    pub fn set_crates(&mut self, crates: HashMap<String, Crate>) {
        self.crates = crates;
    }

    pub fn set_keywords(&mut self, keywords: HashMap<String, Keyword>) {
        self.keywords = keywords;
    }

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
                    let to_crate_val = self.crates.get(&to).unwrap_or_else(|| {
                        panic!("crate {} does not have dependency {}", from, to)
                    });
                    let dependency_tuple =
                        (from_crate_val.name.to_owned(), to_crate_val.name.to_owned());

                    if !dependencies_seen.contains(&dependency_tuple) {
                        if let Some(dependency) = from_crate_val
                            .dependencies
                            .iter()
                            .find(|dependency| dependency.to == to)
                        {
                            dependencies.push(dependency);
                        }

                        dependencies_seen.insert(dependency_tuple);
                    }

                    match crates_seen.get_mut(&to_crate_val.name) {
                        Some(crate_feature_names) => {
                            if to_feature_names.iter().any(|dependency_feature_name| {
                                !crate_feature_names.contains(dependency_feature_name)
                            }) {
                                dependency_graph_helper(
                                    &to_crate_val,
                                    to_feature_names,
                                    &mut dependency_queue,
                                    to_distance,
                                );
                            }
                        }
                        None => {
                            crate_distance_vec.push((&to_crate_val.name, to_distance));
                            crates_seen.insert(&to_crate_val.name, to_feature_names.to_owned());
                            dependency_graph_helper(
                                &to_crate_val,
                                to_feature_names,
                                &mut dependency_queue,
                                to_distance,
                            );
                        }
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
