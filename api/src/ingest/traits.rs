use chrono::NaiveDateTime;
use semver_parser::version as semver_version;
use serde::Deserialize;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};

#[derive(Deserialize, Debug)]
pub struct Category {
    pub category: String,
    #[serde(skip_deserializing, default)]
    pub crates: HashSet<usize>,
    pub description: String,
    pub id: usize,
    pub path: String,
    pub slug: String,
}

#[derive(Deserialize, Debug)]
pub struct Crate {
    #[serde(skip_deserializing, default)]
    pub categories: HashSet<usize>,
    #[serde(skip_deserializing, default = "default_naive_date_time")]
    pub created_at: NaiveDateTime,
    #[serde(skip_deserializing, default)]
    pub dependencies: HashSet<Dependency>,
    pub description: String,
    #[serde(skip_deserializing, default)]
    pub downloads: usize,
    #[serde(skip_deserializing, default)]
    pub features: HashMap<String, Vec<String>>,
    #[serde(skip_deserializing, default)]
    pub keywords: HashSet<usize>,
    pub id: usize,
    pub name: String,
    #[serde(skip_deserializing, default)]
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct CrateCategory {
    pub category_id: usize,
    pub crate_id: usize,
}

#[derive(Deserialize, Debug)]
pub struct CrateKeyword {
    pub crate_id: usize,
    pub keyword_id: usize,
}

#[derive(Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct Dependency {
    pub default_features: bool,
    pub features: Vec<String>,
    pub from: usize,
    pub kind: usize,
    pub optional: bool,
    pub to: usize,
}

#[derive(Deserialize, Debug)]
pub struct Keyword {
    #[serde(skip_deserializing, default)]
    pub crates: Vec<usize>,
    pub crates_cnt: usize,
    pub id: usize,
    pub keyword: String,
}

#[derive(Deserialize, Debug)]
pub struct Version {
    pub crate_id: usize,
    #[serde(with = "custom_time")]
    pub created_at: NaiveDateTime,
    pub downloads: usize,
    pub features: String,
    pub id: usize,
    pub num: String,
}

#[derive(Deserialize, Debug)]
pub struct SqlDependency {
    pub crate_id: usize,
    pub default_features: String,
    pub features: String,
    pub id: usize,
    pub kind: usize,
    pub optional: String,
    pub version_id: usize,
}

fn default_naive_date_time() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(0, 0)
}

impl Version {
    pub fn is_pre(&self) -> bool {
        !semver_version::parse(self.num.as_str())
            .expect(format!("{} does not adhere to SemVer", self.num).as_str())
            .pre
            .is_empty()
    }
}

pub trait Vertex {
    fn id(&self) -> usize;
}

impl Vertex for Category {
    fn id(&self) -> usize {
        self.id
    }
}

impl Vertex for Crate {
    fn id(&self) -> usize {
        self.id
    }
}

impl Vertex for Keyword {
    fn id(&self) -> usize {
        self.id
    }
}

mod custom_time {
    use chrono::{DateTime, NaiveDateTime};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT_1: &'static str = "%Y-%m-%d %H:%M:%S.%f";
    const FORMAT_2: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if let Ok(date_time) = DateTime::parse_from_rfc3339(&s) {
            return Ok(date_time.naive_utc());
        }
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&s, FORMAT_1) {
            return Ok(date_time);
        }

        NaiveDateTime::parse_from_str(&s, FORMAT_2).map_err(serde::de::Error::custom)
    }
}

pub struct Graph {
    categories: HashMap<usize, Category>,
    crates: HashMap<usize, Crate>,
    keywords: HashMap<usize, Keyword>,
}

impl Graph {
    pub fn new(
        categories: HashMap<usize, Category>,
        crates: HashMap<usize, Crate>,
        keywords: HashMap<usize, Keyword>,
    ) -> Graph {
        Graph {
            categories,
            crates,
            keywords,
        }
    }

    pub fn categories(&self) -> &HashMap<usize, Category> {
        &self.categories
    }

    pub fn crates(&self) -> &HashMap<usize, Crate> {
        &self.crates
    }

    pub fn keyword(&self) -> &HashMap<usize, Keyword> {
        &self.keywords
    }

    pub fn set_categories(&mut self, categories: HashMap<usize, Category>) {
        self.categories = categories;
    }

    pub fn set_crates(&mut self, crates: HashMap<usize, Crate>) {
        self.crates = crates;
    }

    pub fn set_keywords(&mut self, keywords: HashMap<usize, Keyword>) {
        self.keywords = keywords;
    }

    pub fn transitive_dependencies(&self, crate_id: usize) -> Option<Vec<&Crate>> {
        Some(
            self.transitive_dependency_ids(crate_id)
                .iter()
                .map(|crate_id| self.crates.get(crate_id).unwrap())
                .collect::<Vec<&Crate>>(),
        )
    }

    fn transitive_dependency_ids(&self, crate_id: usize) -> HashSet<usize> {
        let root_crate = self
            .crates
            .get(&crate_id)
            .expect(format!("Unable to find crate with id {}", crate_id).as_str());
        let mut dependency_ids: HashSet<usize> = HashSet::new();

        for dependency in &root_crate.dependencies {
            let dependency_id = dependency.to;
            if !dependency_ids.contains(&dependency_id) {
                dependency_ids.insert(dependency_id);
                dependency_ids.extend(self.transitive_dependency_ids(dependency_id));
            }
        }

        dependency_ids
    }
}
