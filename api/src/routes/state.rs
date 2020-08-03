use super::super::utils::State;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use vault_graph::Graph;

/// The minimum interval of time before a state update is permitted.
///
/// The crates.io database dump is updated daily, so this interval lies just under a day to permit some leeway.
const INTERVAL: u64 = 60 * (60 * 23 + 55);

/// A struct containing the time since the `Graph` was last updated.
#[derive(Deserialize, Serialize)]
pub struct LastUpdated {
    /// The time (in seconds) since the `Graph` was last updated.
    seconds: u64,
}

/// Returns the time (in seconds) since the `Graph` was last updated.
pub async fn time_since_last_update(data: State) -> HttpResponse {
    HttpResponse::Ok().json(LastUpdated {
        seconds: data.read().await.time_since_last_update(),
    })
}

/// A helper method to determine if the `Graph` can be updated.
///
/// # Arguments
/// * `data` - the Actix app data containing the `Graph`.
async fn can_update(data: &State) -> bool {
    let mut graph = data.write().await;
    if graph.time_since_last_update() >= INTERVAL {
        graph.update_time();
        true
    } else {
        false
    }
}

/// Updates the `Graph` so that it contains the latest crates.io data.
///
/// # Errors
/// * Returns a `403` error if not enough time has passed since the `Graph` was last updated.
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
