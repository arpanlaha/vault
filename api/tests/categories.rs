#[macro_use]
extern crate lazy_static;

mod common;

use serde::{Deserialize, Serialize};
use std::str;
use vault_api::routes::{self, utils::State};
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
    let filters = routes::categories::routes(STATE.clone());

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

// #[actix_rt::test]
// async fn test_get_category_no_id() {
//     let mut app = test::init_service(
//         App::new()
//             .route("/categories/", web::get().to(categories::get_category))
//             .app_data(DATA.clone()),
//     )
//     .await;

//     let req = TestRequest::get().uri("/categories/").to_request();
//     let resp = test::call_service(&mut app, req).await;

//     assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
//     assert_eq!(
//         common::get_body_as_string(resp).await,
//         "\"Category id must be provided.\""
//     );
// }

// #[actix_rt::test]
// async fn test_get_category_nonexistent() {
//     let mut app = test::init_service(
//         App::new()
//             .route(
//                 "/categories/{category_id}",
//                 web::get().to(categories::get_category),
//             )
//             .app_data(DATA.clone()),
//     )
//     .await;

//     let req = TestRequest::get()
//         .uri("/categories/nonexistent")
//         .to_request();
//     let resp = test::call_service(&mut app, req).await;

//     assert_eq!(resp.status(), StatusCode::NOT_FOUND);
//     assert_eq!(
//         common::get_body_as_string(resp).await,
//         "\"Category with id nonexistent does not exist.\""
//     );
// }

// #[actix_rt::test]
// async fn test_get_category_ok() {
//     let mut app = test::init_service(
//         App::new()
//             .route(
//                 "/categories/{category_id}",
//                 web::get().to(categories::get_category),
//             )
//             .app_data(DATA.clone()),
//     )
//     .await;

//     let req = TestRequest::get()
//         .uri("/categories/WebAssembly")
//         .to_request();
//     let resp = test::call_service(&mut app, req).await;

//     let graph = DATA.read().await;

//     assert_eq!(resp.status(), StatusCode::OK);
//     assert_eq!(
//         common::get_body_as_string(resp).await.as_str(),
//         serde_json::to_string(&CategoryResponse::new(
//             graph.categories().get("WebAssembly").unwrap(),
//             &graph
//         ))
//         .unwrap()
//     )
// }

// #[actix_rt::test]
// async fn test_random_category() {
//     let mut app = test::init_service(
//         App::new()
//             .route("/random/categories", web::get().to(categories::random))
//             .app_data(DATA.clone()),
//     )
//     .await;

//     let req = TestRequest::get().uri("/random/categories").to_request();
//     let resp = test::call_service(&mut app, req).await;

//     assert_eq!(resp.status(), StatusCode::OK);
//     assert!(serde_json::from_str::<TestCategoryResponse>(
//         common::get_body_as_string(resp).await.as_str()
//     )
//     .is_ok());
// }

// #[actix_rt::test]
// async fn test_search_category_no_search_term() {
//     let mut app = test::init_service(
//         App::new()
//             .route("/search/categories", web::get().to(categories::search))
//             .app_data(DATA.clone()),
//     )
//     .await;

//     let req = TestRequest::get().uri("/search/categories").to_request();
//     let resp = test::call_service(&mut app, req).await;

//     assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
//     assert_eq!(
//         common::get_body_as_string(resp).await,
//         "\"Search term must be provided.\""
//     );
// }

// #[actix_rt::test]
// async fn test_search_category_ok() {
//     let mut app = test::init_service(
//         App::new()
//             .route(
//                 "/search/categories/{search_term}",
//                 web::get().to(categories::search),
//             )
//             .app_data(DATA.clone()),
//     )
//     .await;

//     let req = TestRequest::get()
//         .uri("/search/categories/web")
//         .to_request();
//     let resp = test::call_service(&mut app, req).await;

//     assert_eq!(resp.status(), StatusCode::OK);

//     let graph = DATA.read().await;
//     assert_eq!(
//         common::get_body_as_string(resp).await.as_str(),
//         serde_json::to_string(&graph.category_names().search("web", graph.categories())).unwrap()
//     )
// }
