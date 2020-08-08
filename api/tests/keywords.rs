#[macro_use]
extern crate lazy_static;

mod common;

use serde::Deserialize;
use std::str;
use vault_api::routes::{
    self,
    utils::{self, State},
};
use vault_graph::Search;
use warp::Filter;

lazy_static! {
    static ref STATE: State = common::get_data();
}

#[derive(Deserialize)]
struct TestKeyword {
    pub crates_cnt: usize,
    pub keyword: String,
}

#[tokio::test]
async fn test_get_keyword_nonexistent() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .path("/keywords/nonexistent")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 404);

    assert_eq!(
        res.body(),
        "\"Keyword with id nonexistent not found.\"".as_bytes()
    );
}

#[tokio::test]
async fn test_get_keyword_ok() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .path("/keywords/web")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 200);

    assert_eq!(
        res.body(),
        serde_json::to_string(STATE.read().keywords().get("web").unwrap(),)
            .unwrap()
            .as_bytes()
    );
}

#[tokio::test]
async fn test_get_random_keyword() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .path("/random/keywords")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 200);

    assert!(serde_json::from_str::<TestKeyword>(str::from_utf8(res.body()).unwrap()).is_ok(),);
}

#[tokio::test]
async fn test_search_keyword() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .path("/search/keywords/web")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 200);

    let graph = STATE.read();

    assert_eq!(
        res.body(),
        serde_json::to_string(&graph.keyword_names().search("web", graph.keywords()))
            .unwrap()
            .as_bytes()
    )
}
