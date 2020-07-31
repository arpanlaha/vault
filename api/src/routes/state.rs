use super::super::utils::State;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use vault_graph::Graph;

const INTERVAL: u64 = 60 * (60 * 23 + 55);

#[derive(Deserialize, Serialize)]
pub struct LastUpdated {
    time_since_last_updated: u64,
}

pub async fn time_since_last_update(data: State) -> HttpResponse {
    HttpResponse::Ok().json(LastUpdated {
        time_since_last_updated: data.read().await.time_since_last_update(),
    })
}

pub async fn reset(data: State) -> HttpResponse {
    if data.read().await.time_since_last_update() >= INTERVAL {
        let new_graph = Graph::new().await;
        data.write().await.replace(new_graph);

        HttpResponse::Ok().json("Successfully updated application state.")
    } else {
        HttpResponse::Forbidden()
            .json("Updating application state can only occur in 24-hour intervals.")
    }
}
