use actix_web::web::Data;
use vault_api::utils::state::AppState;

pub async fn get_data() -> Data<AppState> {
    Data::new(AppState::test().await)
}
