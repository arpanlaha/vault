#[macro_use]
extern crate lazy_static;

mod common;

use vault_api::routes::{
    self,
    compiler::{CfgNameList, TargetList},
    utils::{self, State},
};
use warp::Filter;

lazy_static! {
    static ref STATE: State = common::get_data();
}

#[tokio::test]
async fn test_get_targets() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .path("/compiler/targets")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 200);

    assert_eq!(
        res.body(),
        serde_json::to_string(&TargetList {
            targets: STATE.targets().keys().collect(),
        })
        .unwrap()
        .as_bytes()
    );
}

#[tokio::test]
async fn test_get_cfg_names() {
    let filters = routes::get(STATE.clone()).recover(utils::handle_rejection);

    let res = warp::test::request()
        .path("/compiler/cfg_names")
        .reply(&filters)
        .await;

    assert_eq!(res.status(), 200);

    assert_eq!(
        res.body(),
        serde_json::to_string(&CfgNameList {
            cfg_names: STATE.cfg_names().iter().collect()
        })
        .unwrap()
        .as_bytes()
    );
}
