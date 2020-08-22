#[macro_use]
extern crate lazy_static;

mod common;

use vault_api::routes::{
    self,
    state::LastUpdated,
    utils::{self, State},
};
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
            seconds: STATE.time_since_last_update()
        })
        .unwrap()
        .as_bytes()
    );
}
