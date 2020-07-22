use super::super::ingest::{
    load as vault_load,
    schema::{Category, Crate, Dependency, Keyword},
};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;

const MAX_SEARCH_LENGTH: usize = 10;

pub struct AppState {
    pub graph: RwLock<Graph>,
}

pub struct Graph {
    categories: HashMap<String, Category>,
    crates: HashMap<String, Crate>,
    keywords: HashMap<String, Keyword>,
}

#[derive(Serialize)]
pub struct DependencyGraph<'a> {
    crates: Vec<&'a Crate>,
    dependencies: Vec<&'a Dependency>,
}

impl Graph {
    pub async fn new() -> Graph {
        // let temp_dir = vault_fs::fetch_data();

        // let data_path = vault_fs::get_data_path(&temp_dir).unwrap();

        let data_path = String::from("/datadrive/vault/dump/data");

        let (categories, crates, keywords) = vault_load::load_database(data_path.as_str()).await;
        // vault_fs::clean_tempdir(temp_dir);

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

        let mut crates: HashSet<&String> = HashSet::new();
        let mut dependencies: HashSet<&Dependency> = HashSet::new();
        crates.insert(&self.crates.get(crate_id).unwrap().name);
        self.transitive_dependency_ids(crate_id, &mut crates, &mut dependencies, &features, true);

        Some(DependencyGraph {
            crates: crates
                .iter()
                .map(|crate_id| self.crates.get(crate_id.as_str()).unwrap())
                .collect(),
            dependencies: dependencies.iter().map(|dependency| *dependency).collect(),
        })
    }

    fn transitive_dependency_ids<'a>(
        &'a self,
        crate_id: &str,
        crates: &mut HashSet<&'a String>,
        dependencies: &mut HashSet<&'a Dependency>,
        features: &Vec<String>,
        default_features: bool,
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
            if dependency.kind == 0
                && !dependencies.contains(dependency)
                && (!dependency.optional || crate_dependency_names.contains(&dependency.to))
            {
                let mut transitive_features: Vec<String> = dependency.features.to_owned();

                if crate_dependency_names.iter().any(|crate_dependency_name| {
                    crate_dependency_name.starts_with(&format!("{}/", dependency.to))
                }) {
                    crate_dependency_names
                        .iter()
                        .filter(|crate_dependency_name| {
                            crate_dependency_name.starts_with(&format!("{}/", dependency.to))
                        })
                        .for_each(|crate_dependency_name| {
                            transitive_features.push(String::from(
                                crate_dependency_name.split('/').skip(1).next().unwrap(),
                            ));
                        });
                }

                dependencies.insert(dependency);
                crates.insert(&dependency.to);

                self.transitive_dependency_ids(
                    dependency.to.as_str(),
                    crates,
                    dependencies,
                    &transitive_features,
                    dependency.default_features,
                );
            }
        }
    }

    pub fn crate_search(&self, search_term: &str) -> Vec<&Crate> {
        let mut results: Vec<(f64, &Crate)> = vec![];

        for Crate {
            downloads, name, ..
        } in self.crates.values()
        {
            if name != search_term {
                let search_score =
                    strsim::jaro_winkler(name, search_term) * (*downloads as f64).log10().sqrt();

                if results.is_empty() {
                    results.push((search_score, self.crates.get(name).unwrap()))
                } else if search_score >= results.last().unwrap().0 {
                    let crate_res = self.crates.get(name).unwrap();
                    if let Some((index, _)) = results
                        .iter()
                        .enumerate()
                        .find(|result| search_score > (result.1).0)
                    {
                        results.insert(index, (search_score, crate_res));
                    }

                    if results.len() > MAX_SEARCH_LENGTH {
                        results.pop();
                    }
                }
            }
        }

        if self.crates().contains_key(search_term) {
            results.insert(0, (0f64, self.crates.get(search_term).unwrap()));
            if results.len() > MAX_SEARCH_LENGTH {
                results.pop();
            }
        }

        results.iter().map(|(_, crate_res)| *crate_res).collect()
    }

    pub fn category_search(&self, search_term: &str) -> Vec<&Category> {
        let mut results: Vec<(f64, &Category)> = vec![];

        for category in self.categories.values() {
            let name = &category.category;

            if name != search_term {
                let search_score = strsim::jaro_winkler(name, search_term)
                    * (category.crates.len() as f64).log10().sqrt();

                if results.is_empty() {
                    results.push((search_score, self.categories.get(name).unwrap()))
                } else if search_score >= results.last().unwrap().0 {
                    let category = self.categories.get(name).unwrap();
                    if let Some((index, _)) = results
                        .iter()
                        .enumerate()
                        .find(|result| search_score > (result.1).0)
                    {
                        results.insert(index, (search_score, category));
                    }

                    if results.len() > MAX_SEARCH_LENGTH {
                        results.pop();
                    }
                }
            }
        }

        if self.crates().contains_key(search_term) {
            results.insert(0, (0f64, self.categories.get(search_term).unwrap()));
            if results.len() > MAX_SEARCH_LENGTH {
                results.pop();
            }
        }

        results.iter().map(|(_, category)| *category).collect()
    }
}
