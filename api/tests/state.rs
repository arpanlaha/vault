#[macro_use]
extern crate lazy_static;

mod common;

use actix_web::{
    http::StatusCode,
    test::{self, TestRequest},
    web::{self, Data},
    App,
};
use tokio::sync::RwLock;
use vault_api::{
    routes::state::{self, LastUpdated},
    utils::State,
};
use vault_graph::Graph;

lazy_static! {
    static ref DATA: State = common::get_data();
}

#[actix_rt::test]
async fn test_time_since_last_update() {
    let mut app = test::init_service(
        App::new()
            .route(
                "/state/last-updated",
                web::get().to(state::time_since_last_update),
            )
            .app_data(DATA.clone()),
    )
    .await;

    let req = TestRequest::get().uri("/state/last-updated").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert!(
        serde_json::from_str::<LastUpdated>(common::get_body_as_string(resp).await.as_str())
            .is_ok()
    );
}

#[actix_rt::test]
async fn test_reset_too_soon() {
    let mut app = test::init_service(
        App::new()
            .route("/state/reset", web::put().to(state::reset))
            .app_data(DATA.clone()),
    )
    .await;

    let req = TestRequest::put().uri("/state/reset").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    assert_eq!(
        common::get_body_as_string(resp).await,
        "\"Updating application state can only occur in 24-hour intervals.\""
    );
}

#[actix_rt::test]
async fn test_reset_ok() {
    let data = Data::new(RwLock::new(Graph::yesterday().await));
    let mut app = test::init_service(
        App::new()
            .route("/state/reset", web::put().to(state::reset))
            .app_data(data.clone()),
    )
    .await;

    let req = TestRequest::put().uri("/state/reset").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(
        common::get_body_as_string(resp).await,
        "\"Successfully updated application state.\""
    );
}
