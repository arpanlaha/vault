#[macro_use]
extern crate lazy_static;

mod common;

use actix_web::{
    http::StatusCode,
    test::{self, TestRequest},
    web::{self, Data},
    App,
};
use serde::Deserialize;
use std::str;
use vault_api::{routes::keywords, utils::state::AppState};

lazy_static! {
    static ref DATA: Data<AppState> = common::get_data();
}

#[derive(Deserialize)]
struct TestKeyword {
    pub crates_cnt: usize,
    pub keyword: String,
}

#[actix_rt::test]
async fn test_get_keyword_no_id() {
    let mut app = test::init_service(
        App::new()
            .route("/keywords/", web::get().to(keywords::get_keyword))
            .app_data(DATA.clone()),
    )
    .await;

    let req = TestRequest::get().uri("/keywords/").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        common::get_body_as_string(resp).await,
        "\"Keyword id must be provided.\""
    );
}

#[actix_rt::test]
async fn test_get_keyword_nonexistent() {
    let mut app = test::init_service(
        App::new()
            .route(
                "/keywords/{keyword_id}",
                web::get().to(keywords::get_keyword),
            )
            .app_data(DATA.clone()),
    )
    .await;

    let req = TestRequest::get().uri("/keywords/nonexistent").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        common::get_body_as_string(resp).await,
        "\"Keyword with id nonexistent does not exist.\""
    );
}

#[actix_rt::test]
async fn test_get_keyword_ok() {
    let mut app = test::init_service(
        App::new()
            .route(
                "/keywords/{keyword_id}",
                web::get().to(keywords::get_keyword),
            )
            .app_data(DATA.clone()),
    )
    .await;

    let req = TestRequest::get().uri("/keywords/actix-web").to_request();
    let resp = test::call_service(&mut app, req).await;

    let graph = DATA.graph.read().await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(
        common::get_body_as_string(resp).await.as_str(),
        serde_json::to_string(graph.keywords().get("actix-web").unwrap()).unwrap()
    )
}

#[actix_rt::test]
async fn test_random_category() {
    let mut app = test::init_service(
        App::new()
            .route("/random/keywords", web::get().to(keywords::random))
            .app_data(DATA.clone()),
    )
    .await;

    let req = TestRequest::get().uri("/random/keywords").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert!(
        serde_json::from_str::<TestKeyword>(common::get_body_as_string(resp).await.as_str())
            .is_ok()
    );
}

#[actix_rt::test]
async fn test_search_keyword_no_search_term() {
    let mut app = test::init_service(
        App::new()
            .route("/search/keywords", web::get().to(keywords::search))
            .app_data(DATA.clone()),
    )
    .await;

    let req = TestRequest::get().uri("/search/keywords").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        common::get_body_as_string(resp).await,
        "\"Search term must be provided.\""
    );
}

#[actix_rt::test]
async fn test_search_keywords_ok() {
    let mut app = test::init_service(
        App::new()
            .route(
                "/search/keywords/{search_term}",
                web::get().to(keywords::search),
            )
            .app_data(DATA.clone()),
    )
    .await;

    let req = TestRequest::get()
        .uri("/search/keywords/actix")
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert!(serde_json::from_str::<Vec<TestKeyword>>(
        common::get_body_as_string(resp).await.as_str()
    )
    .is_ok());
}
