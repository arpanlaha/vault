#[macro_use]
extern crate lazy_static;

mod common;

use chrono::NaiveDateTime;
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
struct TestCrate {
    pub created_at: NaiveDateTime,
    pub description: String,
    pub downloads: usize,
    pub name: String,
    pub version: String,
}

#[tokio::test]
async fn test_get_crate_nonexistent() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .path("/crates/nonexistent")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 404);

    assert_eq!(
        res.body(),
        "\"Crate with id nonexistent not found.\"".as_bytes()
    );
}

#[tokio::test]
async fn test_get_crate_ok() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .path("/crates/warp")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 200);

    assert_eq!(
        res.body(),
        serde_json::to_string(STATE.crates().get("warp").unwrap(),)
            .unwrap()
            .as_bytes()
    );
}

#[tokio::test]
async fn test_get_random_crate() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .path("/random/crates")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 200);

    assert!(serde_json::from_str::<TestCrate>(str::from_utf8(res.body()).unwrap()).is_ok(),);
}

#[tokio::test]
async fn test_search_crate() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .path("/search/crates/warp")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 200);

    assert_eq!(
        res.body(),
        serde_json::to_string(&STATE.crate_names().search("warp", STATE.crates()))
            .unwrap()
            .as_bytes()
    )
}

#[tokio::test]
async fn test_graph() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .path("/graph/warp")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 200);

    assert_eq!(
        res.body(),
        serde_json::to_string(&STATE.get_dependency_graph("warp", vec![], &None, &None,))
            .unwrap()
            .as_bytes()
    )
}

#[tokio::test]
async fn test_graph_features() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .path("/graph/warp?features=tls,websocket,compression")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 200);

    assert_eq!(
        res.body(),
        serde_json::to_string(&STATE.get_dependency_graph(
            "warp",
            vec![
                String::from("tls"),
                String::from("websocket"),
                String::from("compression")
            ],
            &None,
            &None,
        ))
        .unwrap()
        .as_bytes()
    )
}

#[tokio::test]
async fn test_graph_features_platform() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .path("/graph/chrono?features=wasmbind&target=x86_64-unknown-linux-gnu")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 200);

    assert_eq!(
        res.body(),
        serde_json::to_string(&STATE.get_dependency_graph(
            "chrono",
            vec![String::from("wasmbind")],
            &Some(String::from("x86_64-unknown-linux-gnu")),
            &None,
        ))
        .unwrap()
        .as_bytes()
    )
}

#[tokio::test]
async fn test_graph_cfg_name() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .path("/graph/time?cfg_name=cargo_web")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 200);

    assert_eq!(
        res.body(),
        serde_json::to_string(&STATE.get_dependency_graph(
            "time",
            vec![],
            &None,
            &Some(String::from("cargo_web")),
        ))
        .unwrap()
        .as_bytes()
    )
}
