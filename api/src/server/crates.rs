use super::{super::ingest::schema::Crate, state::AppState, util::get_query_params};
use actix_web::{web::Data, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct CrateListResponse<'a> {
    count: usize,
    crates: &'a Vec<&'a Crate>,
}

pub async fn get_transitive_dependencies_by_crate_id(
    req: HttpRequest,
    data: Data<AppState>,
) -> impl Responder {
    match req.match_info().get("crate_id") {
        None => HttpResponse::BadRequest().json("Crate id must be provided."),

        Some(crate_id) => match get_query_params(req.query_string()) {
            Err(_) => HttpResponse::BadRequest().json("Bad query string."),

            Ok(feature_map) => {
                match &data.graph.read().await.transitive_dependencies(
                    crate_id,
                    match feature_map.get("features") {
                        Some(features) => {
                            if features.contains(',') {
                                features
                                    .split(',')
                                    .map(|feature| String::from(feature))
                                    .collect::<Vec<String>>()
                            } else {
                                vec![features.to_owned()]
                            }
                        }
                        None => vec![],
                    },
                ) {
                    None => HttpResponse::NotFound()
                        .json(format!("Crate with id {} does not exist.", crate_id)),

                    Some(dependency_graph) => HttpResponse::Ok().json(dependency_graph),
                }
            }
        },
    }
}

pub async fn get_crate(req: HttpRequest, data: Data<AppState>) -> impl Responder {
    match req.match_info().get("crate_id") {
        None => HttpResponse::BadRequest().json("Crate id must be provided."),

        Some(crate_id) => match &data.graph.read().await.crates().get(crate_id) {
            None => {
                HttpResponse::NotFound().json(format!("Crate with id {} does not exist.", crate_id))
            }

            Some(crate_res) => HttpResponse::Ok().json(crate_res),
        },
    }
}

pub async fn search(req: HttpRequest, data: Data<AppState>) -> impl Responder {
    match req.match_info().get("search_term") {
        None => HttpResponse::BadRequest().json("Search term must be provided."),

        Some(search_term) => {
            let graph = data.graph.read().await;
            let search_results = graph.search(search_term);
            HttpResponse::Ok().json(CrateListResponse {
                count: search_results.len(),
                crates: &search_results,
            })
        }
    }
}
