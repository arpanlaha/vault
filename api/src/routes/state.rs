use super::super::utils::State;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use vault_graph::Graph;

const INTERVAL: u64 = 60 * (60 * 23 + 55);

#[derive(Deserialize, Serialize)]
pub struct LastUpdated {
    seconds: u64,
}

pub async fn time_since_last_update(data: State) -> HttpResponse {
    HttpResponse::Ok().json(LastUpdated {
        seconds: data.read().await.time_since_last_update(),
    })
}

async fn can_update(data: &State) -> bool {
    let mut graph = data.write().await;
    if graph.time_since_last_update() >= INTERVAL {
        graph.update_time();
        true
    } else {
        false
    }
}

pub async fn reset(data: State) -> HttpResponse {
    if can_update(&data).await {
        let new_graph = Graph::new().await;
        data.write().await.replace(new_graph);

        HttpResponse::Ok().json("Successfully updated application state.")
    } else {
        HttpResponse::Forbidden()
            .json("Updating application state can only occur in 24-hour intervals.")
    }
}
