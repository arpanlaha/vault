#[macro_use]
extern crate lazy_static;

mod common;

use actix_web::{
    http::StatusCode,
    test::{self, TestRequest},
    web::{self, Data},
    App,
};
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::str;
use vault_api::{routes::crates, utils::state::AppState};

lazy_static! {
    static ref DATA: Data<AppState> = common::get_data();
}

#[derive(Deserialize)]
struct TestCrate {
    pub created_at: NaiveDateTime,
    pub description: String,
    pub downloads: usize,
    pub name: String,
    pub version: String,
}

#[actix_rt::test]
async fn test_get_crate_no_id() {
    let mut app = test::init_service(
        App::new()
            .route("/crates/", web::get().to(crates::get_crate))
            .app_data(DATA.clone()),
    )
    .await;

    let req = TestRequest::get().uri("/crates/").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        common::get_body_as_string(resp).await,
        "\"Crate id must be provided.\""
    );
}

#[actix_rt::test]
async fn test_get_crate_nonexistent() {
    let mut app = test::init_service(
        App::new()
            .route("/crates/{crate_id}", web::get().to(crates::get_crate))
            .app_data(DATA.clone()),
    )
    .await;

    let req = TestRequest::get().uri("/crates/nonexistent").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        common::get_body_as_string(resp).await,
        "\"Crate with id nonexistent does not exist.\""
    );
}

#[actix_rt::test]
async fn test_get_crate_ok() {
    let mut app = test::init_service(
        App::new()
            .route("/crates/{crate_id}", web::get().to(crates::get_crate))
            .app_data(DATA.clone()),
    )
    .await;

    let req = TestRequest::get().uri("/crates/actix-web").to_request();
    let resp = test::call_service(&mut app, req).await;

    let graph = DATA.graph.read().await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(
        common::get_body_as_string(resp).await.as_str(),
        serde_json::to_string(graph.crates().get("actix-web").unwrap()).unwrap()
    )
}

#[actix_rt::test]
async fn test_random_category() {
    let mut app = test::init_service(
        App::new()
            .route("/random/crates", web::get().to(crates::random))
            .app_data(DATA.clone()),
    )
    .await;

    let req = TestRequest::get().uri("/random/crates").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert!(
        serde_json::from_str::<TestCrate>(common::get_body_as_string(resp).await.as_str()).is_ok()
    );
}

#[actix_rt::test]
async fn test_search_crate_no_search_term() {
    let mut app = test::init_service(
        App::new()
            .route("/search/crates", web::get().to(crates::search))
            .app_data(DATA.clone()),
    )
    .await;

    let req = TestRequest::get().uri("/search/crates").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        common::get_body_as_string(resp).await,
        "\"Search term must be provided.\""
    );
}

#[actix_rt::test]
async fn test_search_crates_ok() {
    let mut app = test::init_service(
        App::new()
            .route(
                "/search/crates/{search_term}",
                web::get().to(crates::search),
            )
            .app_data(DATA.clone()),
    )
    .await;

    let req = TestRequest::get().uri("/search/crates/actix").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert!(serde_json::from_str::<Vec<TestCrate>>(
        common::get_body_as_string(resp).await.as_str()
    )
    .is_ok());
}

#[actix_rt::test]
async fn test_graph_no_id() {
    let mut app = test::init_service(
        App::new()
            .route("/graph/", web::get().to(crates::get_dependency_graph))
            .app_data(DATA.clone()),
    )
    .await;

    let req = TestRequest::get().uri("/graph/").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        common::get_body_as_string(resp).await,
        "\"Crate id must be provided.\""
    );
}

#[actix_rt::test]
async fn test_graph_bad_query_string() {
    let mut app = test::init_service(
        App::new()
            .route(
                "/graph/{crate_id}",
                web::get().to(crates::get_dependency_graph),
            )
            .app_data(DATA.clone()),
    )
    .await;

    let req = TestRequest::get()
        .uri("/graph/actix-web?features")
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        common::get_body_as_string(resp).await,
        "\"Bad query string.\""
    );
}

#[actix_rt::test]
async fn test_graph_ok() {
    let mut app = test::init_service(
        App::new()
            .route(
                "/graph/{crate_id}",
                web::get().to(crates::get_dependency_graph),
            )
            .app_data(DATA.clone()),
    )
    .await;

    let req = TestRequest::get().uri("/graph/actix-web").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(
        common::get_body_as_string(resp).await,
        serde_json::to_string(
            &DATA
                .graph
                .read()
                .await
                .get_dependency_graph("actix-web", vec![])
                .unwrap()
        )
        .unwrap()
    );
}

#[actix_rt::test]
async fn test_graph_ok_features() {
    let mut app = test::init_service(
        App::new()
            .route(
                "/graph/{crate_id}",
                web::get().to(crates::get_dependency_graph),
            )
            .app_data(DATA.clone()),
    )
    .await;

    let req = TestRequest::get()
        .uri("/graph/actix-web?features=tls")
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(
        common::get_body_as_string(resp).await,
        serde_json::to_string(
            &DATA
                .graph
                .read()
                .await
                .get_dependency_graph("actix-web", vec![String::from("tls")])
                .unwrap()
        )
        .unwrap()
    );
}
