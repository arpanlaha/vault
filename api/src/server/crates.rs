use super::{state::AppState, util};
use actix_web::{web::Data, HttpRequest, HttpResponse, Responder};

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

pub async fn random(data: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(util::random(data.graph.read().await.crates()))
}

pub async fn search(req: HttpRequest, data: Data<AppState>) -> impl Responder {
    match req.match_info().get("search_term") {
        None => HttpResponse::BadRequest().json("Search term must be provided."),

        Some(search_term) => {
            HttpResponse::Ok().json(util::search(search_term, data.graph.read().await.crates()))
        }
    }
}

pub async fn get_dependency_graph(req: HttpRequest, data: Data<AppState>) -> impl Responder {
    match req.match_info().get("crate_id") {
        None => HttpResponse::BadRequest().json("Crate id must be provided."),

        Some(crate_id) => match util::get_query_params(req.query_string()) {
            Err(_) => HttpResponse::BadRequest().json("Bad query string."),

            Ok(feature_map) => {
                match &data.graph.read().await.get_dependency_graph(
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
