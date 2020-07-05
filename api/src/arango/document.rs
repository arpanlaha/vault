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
    pub id: String,
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
    pub id: usize,
    pub kind: usize,
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
pub struct SqlDependency {
    pub crate_id: usize,
    pub id: usize,
    pub kind: usize,
    pub version_id: usize,
}

impl Version {
    pub fn is_pre(&self) -> bool {
        !semver_version::parse(self.num.as_str())
            .unwrap()
            .pre
            .is_empty()
    }
}

pub trait ArangoDocument {
    fn get_insert_query(&self) -> String;
}

fn escape_quotes(input: &String) -> String {
    input.replace("\"", "\\\"")
}

impl ArangoDocument for Category {
    fn get_insert_query(&self) -> String {
        let Category {
            category,
            description,
            id,
            path,
            slug,
        } = self;
        format!(
            r#"INSERT {{ _key: "{}", category: "{}", description: "{}", id: {}, path: "{}", slug: "{}" }} INTO categories"#,
            id,
            category,
            // TODO: fix appearance in db
            escape_quotes(description),
            id,
            path,
            slug
        )
    }
}

impl ArangoDocument for Crate {
    fn get_insert_query(&self) -> String {
        let Crate {
            description,
            id,
            name,
        } = self;
        format!(
            r#"INSERT {{ _key: "{}", description: "{}", id: {}, name: "{}" }} INTO crates"#,
            id,
            escape_quotes(description),
            id,
            name
        )
    }
}

impl ArangoDocument for CrateCategory {
    fn get_insert_query(&self) -> String {
        let CrateCategory {
            category_id,
            crate_id,
        } = self;
        format!(
            r#"INSERT {{ category_id: {}, crate_id: {}, _from: "crates/{}", _to: "categories/{}" }} INTO crates_categories"#,
            category_id, crate_id, crate_id, category_id
        )
    }
}

impl ArangoDocument for CrateKeyword {
    fn get_insert_query(&self) -> String {
        let CrateKeyword {
            crate_id,
            keyword_id,
        } = self;
        format!(
            r#"INSERT {{ crate_id: {}, _from: "crates/{}", keyword_id: {}, _to: "keywords/{}" }} INTO crates_keywords"#,
            crate_id, crate_id, keyword_id, keyword_id
        )
    }
}

impl ArangoDocument for Dependency {
    fn get_insert_query(&self) -> String {
        let Dependency { from, id, kind, to } = self;
        format!(
            r#"INSERT {{ _key: "{}", _from: "crates/{}", id: {}, kind: {}, _to: "crates/{}" }} INTO dependencies"#,
            id, from, id, kind, to
        )
    }
}

impl ArangoDocument for Keyword {
    fn get_insert_query(&self) -> String {
        let Keyword {
            crates_cnt,
            id,
            keyword,
        } = self;
        format!(
            r#"INSERT {{ _key: "{}", crates_cnt: {}, id: {}, keyword: "{}" }} INTO keywords"#,
            id,
            crates_cnt,
            id,
            escape_quotes(keyword),
        )
    }
}

impl ArangoDocument for Version {
    fn get_insert_query(&self) -> String {
        let Version {
            crate_id,
            downloads,
            id,
            num,
            ..
        } = self;
        format!(
            r#"UPDATE "{}" WITH {{ current_version: "{}", current_version_id: {}, downloads: {} }} INTO crates"#,
            crate_id, num, id, downloads
        )
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
