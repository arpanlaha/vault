use chrono::NaiveDateTime;
use semver_parser::version as semver_version;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};

/// A category in the crates.io registry.
#[derive(Deserialize, Debug, Serialize)]
pub struct Category {
    /// The name of the category.
    ///
    /// This becomes the key of the category for easier search/access by the API.
    pub category: String,

    /// A set of crates belonging to the category.
    ///
    /// This is not set on deserialization and instead must be populated later when processing crate-category relationships.
    #[serde(skip_deserializing, default, skip_serializing)]
    pub crates: HashSet<String>,

    // /The description of the category.
    pub description: String,

    /// The id of the Category in the SQL database.
    ///
    /// This is disregarded by the API.
    #[serde(skip_serializing)]
    pub id: usize,
}

/// A crate in the crates.io registry.
#[derive(Deserialize, Debug, Serialize)]
pub struct Crate {
    /// A list of categories the crate belongs to.
    ///
    /// This is not set on deserialization and instead must be populated later when processing crate-category relationships.
    #[serde(skip_deserializing, default)]
    pub categories: HashSet<String>,

    /// The time at which the most recent stable version (if available) of the crate was created.
    ///
    /// This is not set on deserialization and must be populated later when assigning versions to crates.
    ///
    /// As NaiveDateTime does not provide a default constructor, the default is defined using the default_naive_date_time module provided below.
    #[serde(skip_deserializing, default = "default_naive_date_time")]
    pub created_at: NaiveDateTime,

    /// The dependencies of the crate.
    ///
    /// This is not set on deserialization and instead must be populated later when processing dependencies.
    #[serde(skip_deserializing, default, skip_serializing)]
    pub dependencies: HashSet<Dependency>,

    /// The description of the crate.
    pub description: String,

    /// The number of downloads of the crate.
    ///
    /// This is not set on deserialization and instead must be populated later when assigning versions to crates.
    #[serde(skip_deserializing, default)]
    pub downloads: usize,

    /// The features exposed by the crate.
    ///
    /// This is not set on deserialization and instead must be populated later when assigning versions to crates.
    #[serde(skip_deserializing, default)]
    pub features: HashMap<String, Vec<String>>,

    /// The keywords belonging to the crate.
    ///
    /// This is not set on deserialization and instead must be populated later when processing crate-keyword relationships.
    #[serde(skip_deserializing, default)]
    pub keywords: HashSet<String>,

    /// The SQL id of the crate.
    ///
    /// This is disregarded by the API.
    #[serde(skip_serializing)]
    pub id: usize,

    /// The name of the crate.
    ///
    /// This becomes the key of the crate for easier search/access by the API.
    pub name: String,

    /// The most recent stable version (if available) of the crate.
    ///
    /// This is not set on deserialization and instead must be populated later when assigning versions to crates.
    #[serde(skip_deserializing, default)]
    pub version: String,
}

/// A relationship between a crate and a category.
#[derive(Deserialize, Debug)]
pub struct CrateCategory {
    /// The category this relationship belongs to.
    pub category_id: usize,

    /// The craet this relationship belongs to.
    pub crate_id: usize,
}

/// A relationship between a crate and a keyword.
#[derive(Deserialize, Debug)]
pub struct CrateKeyword {
    /// The category this relationship belongs to.
    pub crate_id: usize,

    /// The keyword this relationship belongs to.
    pub keyword_id: usize,
}

/// A dependency between crates.
///
/// This is not directly obtained from the SQL dump - for that, see the SqlDependency struct.
#[derive(Deserialize, Debug, Hash, Eq, PartialEq, Serialize)]
pub struct Dependency {
    /// If the dependency relies on default features.
    ///
    /// TODO: confirm this is the case.
    #[serde(skip_serializing)]
    pub default_features: bool,

    /// A list of features this dependency uses.
    #[serde(skip_serializing)]
    pub features: Vec<String>,

    /// The source crate of this dependency.
    pub from: String,

    /// The kind of the dependency.
    ///
    /// 0: standard dependency, 1: dev dependency, 2: build dependency.
    pub kind: usize,

    /// If the dependency is optional.
    #[serde(skip_serializing)]
    pub optional: bool,

    /// The destination version of the dependency.
    pub to: String,
}

/// A keyword in the crates.io registry.
#[derive(Deserialize, Debug, Serialize)]
pub struct Keyword {
    /// The crates possessing the keyword.
    ///
    /// This is not set on deserialization and instead must be populated later when processing crate-keyword relationships.
    #[serde(skip_deserializing, default, skip_serializing)]
    pub crates: Vec<String>,

    /// The number of crates possessing the keyword.
    pub crates_cnt: usize,

    /// The SQL id of the keyword.
    ///
    /// This is disregarded by the API.
    #[serde(skip_serializing)]
    pub id: usize,

    /// The name of the keyword.
    ///
    /// This becomes the key of the version for easier search/access by the API.
    pub keyword: String,
}

/// A version in the crates.io repository.
#[derive(Deserialize, Debug)]
pub struct Version {
    /// The id of the crate the version belongs to.
    pub crate_id: usize,

    /// The time at which the crate was created.
    #[serde(with = "custom_time")]
    pub created_at: NaiveDateTime,

    /// The number of downloads of the version.
    pub downloads: usize,

    /// The features the version exposes.
    pub features: String,

    /// The id of the version.
    pub id: usize,

    /// The number of the version.
    ///
    /// This will likely be SemVer-compliant; however some versions are not.
    pub num: String,
}

/// A representation of a dependency in the crates.io registry obtained from the SQL database dump.
#[derive(Deserialize, Debug)]
pub struct SqlDependency {
    /// The source crate of this dependency.
    pub crate_id: usize,

    /// If the dependency relies on default features.
    ///
    /// TODO: confirm this is the case.
    pub default_features: String,

    /// A list of features this dependency uses.
    pub features: String,

    // The kind of the dependency.
    ///
    /// 0: standard dependency, 1: dev dependency, 2: build dependency.
    pub kind: usize,

    /// If the dependency is optional.
    pub optional: String,

    /// The destination version of the dependency.
    pub version_id: usize,
}

/// An implementation of a default constructor for the NaiveDateTime struct.
///
/// This creates a NaiveDateTime with 0 seconds and 0 nanoseconds since January 1, 1970.
fn default_naive_date_time() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(0, 0)
}

impl Version {
    /// Determines if a version is stable or in preview.
    ///
    /// Returns true of the version is a preview version.
    ///
    /// This function will panic if the version's num is not SemVer-compliant.
    pub fn is_pre(&self) -> bool {
        !semver_version::parse(self.num.as_str())
            .expect(format!("{} does not adhere to SemVer", self.num).as_str())
            .pre
            .is_empty()
    }
}

/// A trait intended for vertices in the API graph.
pub trait Vertex {
    /// Returns a unique identifier for use by the API graph.
    fn id(&self) -> &str;

    /// Returns the unique identifier from the SQL representation.
    fn sql_id(&self) -> usize;

    /// Returns a number corresponding to the popularity of the Vertex.
    fn popularity(&self) -> f64;
}

impl Vertex for Category {
    /// Returns a unique identifier for use by the API graph.
    fn id(&self) -> &str {
        self.category.as_str()
    }

    /// Returns the unique identifier from the SQL representation.
    fn sql_id(&self) -> usize {
        self.id
    }

    /// Returns a number corresponding to the popularity of the Vertex.
    fn popularity(&self) -> f64 {
        (self.crates.len() as f64).log10().log10()
    }
}

impl Vertex for Crate {
    /// Returns a unique identifier for use by the API graph.
    fn id(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the unique identifier from the SQL representation.
    fn sql_id(&self) -> usize {
        self.id
    }

    /// Returns a number corresponding to the popularity of the Vertex.
    fn popularity(&self) -> f64 {
        (self.downloads as f64).log10().sqrt()
    }
}

impl Vertex for Keyword {
    /// Returns a unique identifier for use by the API graph.
    fn id(&self) -> &str {
        self.keyword.as_str()
    }
    /// Returns the unique identifier from the SQL representation.
    fn sql_id(&self) -> usize {
        self.id
    }

    /// Returns a number corresponding to the popularity of the Vertex.
    fn popularity(&self) -> f64 {
        (self.crates.len() as f64).log10().sqrt()
    }
}

/// Implements a custom time deserializer for versions in the crates.io registry.
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

        // Attempt parsing using RFC 3339
        if let Ok(date_time) = DateTime::parse_from_rfc3339(&s) {
            return Ok(date_time.naive_utc());
        }

        // Attempt using format string 1
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&s, FORMAT_1) {
            return Ok(date_time);
        }

        // Use format string 2
        NaiveDateTime::parse_from_str(&s, FORMAT_2).map_err(serde::de::Error::custom)
    }
}
