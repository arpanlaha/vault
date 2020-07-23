use super::state::{AppState, Graph};
use actix_web::{web::Data, HttpResponse, Responder};

const INTERVAL: u64 = 60 * (60 * 23 + 55);

pub async fn reset_state(data: Data<AppState>) -> impl Responder {
    let mut last_updated = data.last_updated.lock().await;
    if last_updated.elapsed().as_secs() >= INTERVAL {
        let graph = Graph::new().await;
        data.graph.write().await.replace(graph);
        let to_add = last_updated.elapsed();
        *last_updated += to_add;

        HttpResponse::Ok().json("Successfully updated application state.")
    } else {
        HttpResponse::Forbidden()
            .json("Updating application state can only occur in 24-hour intervals.")
    }
}
