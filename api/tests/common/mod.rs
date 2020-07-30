use actix_web::web::Data;
use futures::executor;
use vault_api::utils::state::AppState;

pub fn get_data() -> Data<AppState> {
    Data::new(executor::block_on(AppState::test()))
}
