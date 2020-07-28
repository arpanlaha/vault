use std::time::Instant;
use tokio::sync::{Mutex, RwLock};
use vault_graph::Graph;

pub struct AppState {
    pub graph: RwLock<Graph>,
    pub last_updated: Mutex<Instant>,
}

impl AppState {
    pub async fn new() -> AppState {
        AppState {
            graph: RwLock::new(Graph::new().await),
            last_updated: Mutex::new(Instant::now()),
        }
    }

    pub async fn test() -> AppState {
        AppState {
            graph: RwLock::new(Graph::test().await),
            last_updated: Mutex::new(Instant::now()),
        }
    }
}
