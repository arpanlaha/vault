use super::super::utils::State;
use actix_web::HttpResponse;
use vault_graph::Graph;

const INTERVAL: u64 = 60 * (60 * 23 + 55);

pub async fn reset_state(data: State) -> HttpResponse {
    let graph = data.read().await;
    if graph.time_since_last_update() >= INTERVAL {
        let new_graph = Graph::new().await;
        data.write().await.replace(new_graph);

        HttpResponse::Ok().json("Successfully updated application state.")
    } else {
        HttpResponse::Forbidden()
            .json("Updating application state can only occur in 24-hour intervals.")
    }
}
