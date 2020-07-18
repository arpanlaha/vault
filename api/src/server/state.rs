use super::super::ingest::{
    load as vault_load,
    schema::{Category, Crate, Keyword},
};
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;

pub struct AppState {
    pub graph: RwLock<Graph>,
}

pub struct Graph {
    categories: HashMap<String, Category>,
    crates: HashMap<String, Crate>,
    keywords: HashMap<String, Keyword>,
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

    pub fn categories(&self) -> &HashMap<String, Category> {
        &self.categories
    }

    pub fn crates(&self) -> &HashMap<String, Crate> {
        &self.crates
    }

    pub fn keyword(&self) -> &HashMap<String, Keyword> {
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

    pub fn transitive_dependencies(&self, crate_id: String) -> Option<Vec<&Crate>> {
        if !self.crates.contains_key(&crate_id) {
            return None;
        }

        let mut dependency_ids: HashSet<String> = HashSet::new();
        self.transitive_dependency_ids(crate_id, &mut dependency_ids);

        Some(
            dependency_ids
                .iter()
                .map(|crate_id| self.crates.get(crate_id).unwrap())
                .collect(),
        )
    }

    fn transitive_dependency_ids(&self, crate_id: String, dependency_ids: &mut HashSet<String>) {
        for dependency in &self
            .crates
            .get(&crate_id)
            .expect(format!("Unable to find crate with id {}", crate_id).as_str())
            .dependencies
        {
            if dependency.kind == 0 && dependency_ids.insert(dependency.to.to_owned()) {
                self.transitive_dependency_ids(dependency.to.to_owned(), dependency_ids);
            }
        }
    }
}
