use super::super::ingest::{
    fs as vault_fs, load as vault_load,
    schema::{Category, Crate, Dependency, Keyword},
};
use chrono::NaiveDateTime;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
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
    crates: Vec<CrateDistance<'a>>,
    dependencies: Vec<&'a Dependency>,
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
        if !self.crates.contains_key(crate_id) {
            return None;
        }

        // TODO: BFS

        let mut crates: HashMap<&String, usize> = HashMap::new();
        let mut dependencies: HashSet<&Dependency> = HashSet::new();
        crates.insert(&self.crates.get(crate_id).unwrap().name, 0);
        self.transitive_dependency_ids(
            crate_id,
            &mut crates,
            &mut dependencies,
            &features,
            true,
            1,
        );

        let mut crates_distance_vec: Vec<(&&String, &usize)> = crates.iter().collect();
        crates_distance_vec.sort_unstable_by_key(|(name, _)| name.as_str());
        crates_distance_vec.sort_by_key(|(_, distance)| *distance);

        let mut dependency_vec: Vec<&Dependency> = dependencies.iter().copied().collect();
        dependency_vec.sort_unstable_by_key(|dependency| dependency.from.as_str());
        dependency_vec.sort_by_key(|dependency| *crates.get(&dependency.from).unwrap());

        Some(DependencyGraph {
            crates: crates_distance_vec
                .iter()
                .map(|crate_distance_tuple| CrateDistance::new(crate_distance_tuple, &self.crates))
                .collect(),
            dependencies: dependency_vec,
        })
    }

    fn transitive_dependency_ids<'a>(
        &'a self,
        crate_id: &str,
        crates: &mut HashMap<&'a String, usize>,
        dependencies: &mut HashSet<&'a Dependency>,
        features: &[String],
        default_features: bool,
        distance: usize,
    ) {
        let crate_val = &self.crates.get(crate_id).unwrap();

        let mut crate_dependency_names: HashSet<&String> = HashSet::new();

        if default_features {
            if let Some(crate_default_features) = crate_val.features.get("default") {
                for dependency in crate_default_features {
                    crate_dependency_names.insert(dependency);
                }
            }
        }

        for feature in features {
            if let Some(crate_features) = crate_val.features.get(feature) {
                for dependency in crate_features {
                    crate_dependency_names.insert(dependency);
                }
            }
        }

        for dependency in &crate_val.dependencies {
            if !dependencies.contains(dependency)
                && (!dependency.optional || crate_dependency_names.contains(&dependency.to))
            {
                let mut transitive_features: Vec<String> = dependency.features.to_owned();

                let is_sub_feature = |crate_dependency_name: &&String| {
                    crate_dependency_name.starts_with(&format!("{}/", dependency.to))
                };

                if crate_dependency_names.iter().any(is_sub_feature) {
                    crate_dependency_names
                        .iter()
                        .filter(|crate_dependency_name| {
                            crate_dependency_name.starts_with(&format!("{}/", dependency.to))
                        })
                        .for_each(|crate_dependency_name| {
                            transitive_features.push(String::from(
                                crate_dependency_name.split('/').nth(1).unwrap(),
                            ));
                        });
                }

                dependencies.insert(dependency);
                crates
                    .entry(&dependency.to)
                    .and_modify(|dependency_distance| {
                        if distance < *dependency_distance {
                            *dependency_distance = distance;
                        }
                    })
                    .or_insert(distance);

                self.transitive_dependency_ids(
                    dependency.to.as_str(),
                    crates,
                    dependencies,
                    &transitive_features,
                    dependency.default_features,
                    distance + 1,
                );
            }
        }
    }
}
