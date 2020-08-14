#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use env_logger::Env;
use parking_lot::RwLock;
use std::env;
use std::sync::Arc;
use vault_api::routes::{self, utils};
use vault_graph::Graph;
use warp::Filter;

#[tokio::main]
async fn main() {
    let mut args = env::args();

    // port defaults to 8080 if not provided
    let port = {
        let port_string = args.nth(1).unwrap_or_else(|| String::from("8080"));

        port_string
            .parse::<u16>()
            .unwrap_or_else(|_| panic!("{} is not a valid port number", port_string))
    };

    // address defaults to `0.0.0.0`, unless the `-l` or `--local` argument is passed, in which case the address is `127.0.0.1`
    let address = args.next().map_or([0; 4], |arg| {
        if arg == "--local" || arg == "-l" {
            [127, 0, 0, 1]
        } else {
            [0; 4]
        }
    });

    let app_state = Arc::new(RwLock::new(Graph::new().await));

    // initialize logger at `info` level
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    warp::serve(
        routes::get(app_state.clone())
            .recover(utils::handle_rejection)
            .with(warp::cors().allow_any_origin())
            .with(warp::log("info"))
    )
    .run((address, port))
    .await;
}
