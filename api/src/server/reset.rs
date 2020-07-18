use super::state::{AppState, Graph};
use actix_web::{web::Data, HttpResponse, Responder};

pub async fn reset_state(data: Data<AppState>) -> impl Responder {
    let graph = Graph::new().await;
    data.graph.write().await.replace(graph);

    HttpResponse::Ok()
}
