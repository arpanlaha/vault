use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct Category {
    category: String,
    description: String,
    id: usize,
    path: String,
    slug: String,
}

#[derive(Deserialize, Debug)]
pub struct Crate {
    description: String,
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Keyword {
    crates_cnt: usize,
    id: usize,
    keyword: String,
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
