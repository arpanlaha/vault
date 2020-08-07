#[macro_use]
extern crate lazy_static;

mod common;

use serde::{Deserialize, Serialize};
use std::str;
use vault_api::routes::{
    self,
    categories::CategoryResponse,
    utils::{self, State},
};
use vault_graph::{Category, Search};
use warp::Filter;

lazy_static! {
    static ref STATE: State = common::get_data();
}

#[derive(Deserialize, Serialize)]
struct TestCategory {
    pub category: String,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
struct TestCategoryResponse {
    pub category: TestCategory,
    pub children: Vec<TestCategory>,
}

#[tokio::test]
async fn test_get_categories() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .path("/categories")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 200);

    let graph = STATE.read();
    let mut categories: Vec<&Category> = graph.categories().values().collect();

    categories.sort_unstable_by_key(|category| category.category.as_str());

    assert_eq!(
        res.body(),
        serde_json::to_string::<Vec<&Category>>(&categories)
            .unwrap()
            .as_bytes()
    );
}

#[tokio::test]
async fn test_get_category_nonexistent() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .path("/categories/nonexistent")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 404);

    assert_eq!(
        res.body(),
        "\"Category with id nonexistent not found.\"".as_bytes()
    );
}

#[tokio::test]
async fn test_get_category_ok() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .path("/categories/Asynchronous")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 200);

    let graph = STATE.read();

    assert_eq!(
        res.body(),
        serde_json::to_string(&CategoryResponse::new(
            graph.categories().get("Asynchronous").unwrap(),
            &graph
        ))
        .unwrap()
        .as_bytes()
    );
}

#[tokio::test]
async fn test_get_random_category() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .path("/random/categories")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 200);

    assert!(
        serde_json::from_str::<TestCategoryResponse>(str::from_utf8(res.body()).unwrap()).is_ok(),
        format!("Did not work: {}", str::from_utf8(res.body()).unwrap())
    );
}

#[tokio::test]
async fn test_search_category() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .path("/search/categories/web")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 200);

    let graph = STATE.read();

    assert_eq!(
        res.body(),
        serde_json::to_string(&graph.category_names().search("web", graph.categories()))
            .unwrap()
            .as_bytes()
    )
}
