#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use actix_cors::Cors;
use actix_web::{
    middleware::{Compress, Logger},
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use env_logger::Env;
use std::env;
use std::io::Result as IoResult;
use tokio::sync::RwLock;
use vault_api::routes::{categories, crates, keywords, state};
use vault_graph::Graph;

#[actix_web::main]
async fn main() -> IoResult<()> {
    let mut args = env::args();

    // port defaults to 8080 if not provided
    let port = args.nth(1).unwrap_or_else(|| String::from("8080"));
    port.parse::<u16>()
        .unwrap_or_else(|_| panic!("{} is not a valid port number", port));

    // address defaults to `0.0.0.0`, unless the `-l` or `--local` argument is passed, in which case the address is `127.0.0.1`
    let address = args.next().map_or("0.0.0.0", |arg| {
        if arg == "--local" || arg == "-l" {
            "127.0.0.1"
        } else {
            "0.0.0.0"
        }
    });

    let app_state = Data::new(RwLock::new(Graph::new().await));

    // initialize logger at `info` level
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(Cors::default())
            .app_data(app_state.clone())
            .route(
                "graph/{crate_id}",
                web::get().to(crates::get_dependency_graph),
            )
            .route("crates/{crate_id}", web::get().to(crates::get_crate))
            .route("categories", web::get().to(categories::get_categories))
            .route(
                "categories/{category_id}",
                web::get().to(categories::get_category),
            )
            .route(
                "keywords/{keyword_id}",
                web::get().to(keywords::get_keyword),
            )
            .service(
                web::scope("random")
                    .route("crates", web::get().to(crates::random))
                    .route("categories", web::get().to(categories::random))
                    .route("keywords", web::get().to(keywords::random)),
            )
            .service(
                web::scope("search")
                    .route("crates/{search_term}", web::get().to(crates::search))
                    .route(
                        "categories/{search_term}",
                        web::get().to(categories::search),
                    )
                    .route("keywords/{search_term}", web::get().to(keywords::search)),
            )
            .service(
                web::scope("state")
                    .route("last-updated", web::get().to(state::time_since_last_update))
                    .route("reset", web::put().to(state::reset)),
            )
            .default_service(web::route().to(|| HttpResponse::NotFound().json("Route not found.")))
    })
    .bind(format!("{}:{}", address, port))?
    .run()
    .await
}
