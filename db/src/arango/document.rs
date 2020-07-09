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

pub trait RedisGraphDocument {
    fn get_insert_query(&self) -> String;
}

pub trait RedisGraphNode {
    fn create_constraint() -> String;
}

fn escape_quotes(input: &String) -> String {
    input.replace("\"", "\\\"")
}

impl RedisGraphDocument for Category {
    fn get_insert_query(&self) -> String {
        let Category {
            category,
            description,
            id,
            path,
            slug,
        } = self;
        format!(
            r#"CREATE (n:Category {{ category: "{}", description: "{}", id: {}, path: "{}", slug: "{}" }})"#,
            category,
            escape_quotes(description),
            id,
            path,
            slug
        )
    }
}

impl RedisGraphNode for Category {
    fn create_constraint() -> String {
        String::from("CREATE CONSTRAINT unique_category_id ON (n:Category) ASSERT n.id IS UNIQUE")
    }
}

impl RedisGraphDocument for Crate {
    fn get_insert_query(&self) -> String {
        let Crate {
            description,
            id,
            name,
        } = self;
        format!(
            r#"CREATE (n:Crate {{ description: "{}", id: {}, name: "{}" }})"#,
            escape_quotes(description),
            id,
            name
        )
    }
}

impl RedisGraphNode for Crate {
    fn create_constraint() -> String {
        String::from("CREATE CONSTRAINT unique_crate_id ON (n:Crate) ASSERT n.id IS UNIQUE")
    }
}

impl RedisGraphDocument for CrateCategory {
    fn get_insert_query(&self) -> String {
        let CrateCategory {
            category_id,
            crate_id,
        } = self;
        format!(
            r#"
            MATCH (crate:Crate), (category:Category)
            WHERE crate.id = {} AND category.id = {}
            CREATE (crate)-[r:HAS_CATEGORY]->(category)
            "#,
            crate_id, category_id
        )
    }
}

impl RedisGraphDocument for CrateKeyword {
    fn get_insert_query(&self) -> String {
        let CrateKeyword {
            crate_id,
            keyword_id,
        } = self;
        format!(
            r#"
            MATCH (crate:Crate), (keyword:Keyword)
            WHERE crate.id = {} AND keyword.id = {}
            CREATE (crate)-[r:HAS_KEYWORD]->(keyword)
            "#,
            crate_id, keyword_id
        )
    }
}

impl RedisGraphDocument for Dependency {
    fn get_insert_query(&self) -> String {
        let Dependency {
            from,
            kind,
            optional,
            to,
        } = self;
        format!(
            r#"
            MATCH (from:Crate), (to:Crate)
            WHERE from.id = {} AND to.id = {}
            CREATE (from)-[r:DEPENDS_ON {{ kind: {}, optional: {} }}]->(to)
            "#,
            from, to, kind, optional
        )
    }
}

impl RedisGraphDocument for Keyword {
    fn get_insert_query(&self) -> String {
        let Keyword {
            crates_cnt,
            id,
            keyword,
        } = self;
        format!(
            r#"CREATE (n:Keyword {{ crates_cnt: {}, id: {}, keyword: "{}" }})"#,
            id, crates_cnt, keyword,
        )
    }
}

impl RedisGraphNode for Keyword {
    fn create_constraint() -> String {
        String::from("CREATE CONSTRAINT unique_keyword_id ON (n:Keyword) ASSERT n.id IS UNIQUE")
    }
}

impl RedisGraphDocument for Version {
    fn get_insert_query(&self) -> String {
        let Version {
            crate_id,
            downloads,
            id,
            num,
            ..
        } = self;
        format!(
            r#"
            MATCH (n:Crate)
            WHERE n.id = {}
            SET n.downloads = {}, n.num = "{}", n.version_id = {}
            "#,
            crate_id, downloads, num, id
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
