use chrono::NaiveDateTime;
use semver_parser::version as semver_version;
use serde::Deserialize;
use std::cmp::PartialEq;
use std::collections::HashSet;

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
