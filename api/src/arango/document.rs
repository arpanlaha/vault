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
            num,
            ..
        } = self;
        format!(
            r#"UPDATE "{}" WITH {{ current_version: "{}", downloads: {} }} IN crates"#,
            crate_id, num, downloads
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
