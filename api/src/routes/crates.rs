use super::super::utils::{self, State};
use actix_web::{HttpRequest, HttpResponse};
use vault_graph::{Random, Search};

pub async fn get_crate(req: HttpRequest, data: State) -> HttpResponse {
    match req.match_info().get("crate_id") {
        None => HttpResponse::BadRequest().json("Crate id must be provided."),

        Some(crate_id) => match &data.read().await.crates().get(crate_id) {
            None => {
                HttpResponse::NotFound().json(format!("Crate with id {} does not exist.", crate_id))
            }

            Some(crate_res) => HttpResponse::Ok().json(crate_res),
        },
    }
}

pub async fn random(data: State) -> HttpResponse {
    HttpResponse::Ok().json(data.read().await.crates().random())
}

pub async fn search(req: HttpRequest, data: State) -> HttpResponse {
    match req.match_info().get("search_term") {
        None => HttpResponse::BadRequest().json("Search term must be provided."),

        Some(search_term) => {
            let graph = data.read().await;
            HttpResponse::Ok().json(graph.crate_names().search(search_term, graph.crates()))
        }
    }
}

pub async fn get_dependency_graph(req: HttpRequest, data: State) -> HttpResponse {
    match req.match_info().get("crate_id") {
        None => HttpResponse::BadRequest().json("Crate id must be provided."),

        Some(crate_id) => match utils::get_query_params(req.query_string()) {
            Err(_) => HttpResponse::BadRequest().json("Bad query string."),

            Ok(feature_map) => {
                match &data.read().await.get_dependency_graph(
                    crate_id,
                    match feature_map.get("features") {
                        Some(features) => {
                            if features.contains(',') {
                                features
                                    .split(',')
                                    .map(String::from)
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
