use chrono::NaiveDateTime;
use semver_parser::version as semver_version;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Category {
    pub category: String,
    pub description: String,
    pub id: usize,
    pub path: String,
    pub slug: String,
}

#[derive(Deserialize, Debug)]
pub struct Crate {
    pub description: String,
    pub id: usize,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct CrateCategory {
    category_id: usize,
    crate_id: usize,
}

#[derive(Deserialize, Debug)]
pub struct CrateKeyword {
    crate_id: usize,
    keyword_id: usize,
}

#[derive(Deserialize, Debug)]
pub struct Dependency {
    pub from: usize,
    pub kind: usize,
    pub optional: bool,
    pub to: usize,
}

#[derive(Deserialize, Debug)]
pub struct Keyword {
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
pub struct VersionedCrate {
    #[serde(with = "custom_time")]
    pub created_at: NaiveDateTime,
    pub description: String,
    pub downloads: usize,
    pub id: usize,
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct SqlDependency {
    pub crate_id: usize,
    pub id: usize,
    pub kind: usize,
    pub optional: String,
    pub version_id: usize,
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
