#[macro_use]
extern crate lazy_static;

mod common;

use parking_lot::RwLock;
use std::sync::Arc;
use vault_api::routes::{
    self,
    state::LastUpdated,
    utils::{self, State},
};
use vault_graph::Graph;
use warp::Filter;

lazy_static! {
    static ref STATE: State = common::get_data();
}

#[tokio::test]
async fn test_time_since_last_update() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .path("/state/last-updated")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 200);

    assert_eq!(
        res.body(),
        serde_json::to_string(&LastUpdated {
            seconds: STATE.read().time_since_last_update()
        })
        .unwrap()
        .as_bytes()
    );
}

#[tokio::test]
async fn test_reset_too_early() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .method("PUT")
        .path("/state/reset")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 403);

    assert_eq!(
        res.body(),
        "\"Updating application state can only occur in 24-hour intervals.\"".as_bytes()
    );
}

#[tokio::test]
async fn test_reset_ok() {
    let yesterday_state = Arc::new(RwLock::new(Graph::yesterday().await));

    let filters = routes::get(yesterday_state.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .method("PUT")
        .path("/state/reset")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 200);

    assert_eq!(
        res.body(),
        "\"Successfully updated application state.\"".as_bytes()
    );
}
