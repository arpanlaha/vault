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

impl Version {
    pub fn is_pre(&self) -> bool {
        semver_version::parse(self.num.as_str())
            .expect(format!("{:?}", self).as_str())
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
            r#"INSERT {{ category: "{}", description: "{}", id: {}, path: "{}", slug: "{}" }} INTO categories"#,
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
            r#"INSERT {{ description: "{}", id: {}, name: "{}" }} INTO crates"#,
            escape_quotes(description),
            id,
            name
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
            r#"INSERT {{ crates_cnt: {}, id: {}, keyword: "{}" }} INTO keywords"#,
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
            created_at,
            downloads,
            id,
            num,
        } = self;
        format!(
            r#"INSERT {{ crate_id: {}, created_at: "{}", downloads: {}, id: {}, num: "{}" }} INTO versions"#,
            crate_id, created_at, downloads, id, num
        )
    }
}

mod custom_time {
    use chrono::{DateTime, NaiveDateTime};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S.%f";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match DateTime::parse_from_rfc3339(&s) {
            Ok(date_time) => Ok(date_time.naive_utc()),
            Err(_) => NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom),
        }
    }
}
