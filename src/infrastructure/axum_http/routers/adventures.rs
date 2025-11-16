use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};

use crate::{
    application::usecases::adventures::AdventuresUsecase,
    domain::{
        repositories::adventures::AdventuresRepository,
        value_objects::adventurer_model::RegisterAdventurerModel,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad, repositories::adventures::AdventurerPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    // ในสุด ไปนอกสุด
    let adventurer_repository = AdventurerPostgres::new(db_pool);
    let adventurer_usecase = AdventuresUsecase::new(Arc::new(adventurer_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(adventurer_usecase))
}

pub async fn register<T>(
    State(adventurer_usecase): State<Arc<AdventuresUsecase<T>>>,
    Json(register_adventurer_model): Json<RegisterAdventurerModel>,
) -> impl IntoResponse
where
    T: AdventuresRepository + Send + Sync,
{
    match adventurer_usecase.register(register_adventurer_model).await {
        Ok(adventurer_id) => (
            StatusCode::CREATED,
            format!(
                "Adventurer registered successfully with ID: {}",
                adventurer_id
            ).into_response(),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!(
                "Error: {}",
                e.to_string()
            ).into_response(),
        ),
    }
}
