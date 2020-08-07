use futures::executor;
use parking_lot::RwLock;
use std::sync::Arc;
use vault_api::routes::utils::State;
use vault_graph::Graph;

pub fn get_data() -> State {
    Arc::new(RwLock::new(executor::block_on(Graph::test())))
}

// pub async fn get_body_as_string(resp: ServiceResponse) -> String {
//     String::from_utf8(test::read_body(resp).await.to_vec()).unwrap()
// }
