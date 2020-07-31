use actix_web::{dev::ServiceResponse, test, web::Data};
use futures::executor;
use tokio::sync::RwLock;
use vault_api::utils::State;
use vault_graph::Graph;

pub fn get_data() -> State {
    Data::new(RwLock::new(executor::block_on(Graph::test())))
}

pub async fn get_body_as_string(resp: ServiceResponse) -> String {
    String::from_utf8(test::read_body(resp).await.to_vec()).unwrap()
}
