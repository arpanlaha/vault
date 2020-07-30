use actix_web::{dev::ServiceResponse, test, web::Data};
use futures::executor;
use vault_api::utils::state::AppState;

pub fn get_data() -> Data<AppState> {
    Data::new(executor::block_on(AppState::test()))
}

pub async fn get_body_as_string(resp: ServiceResponse) -> String {
    String::from_utf8(test::read_body(resp).await.to_vec()).unwrap()
}
