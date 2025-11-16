use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};

use crate::{
    application::usecases::guild_commanders::GuildCommandersUsecase,
    domain::{
        repositories::guild_commanders::GuildCommandersRepository,
        value_objects::guild_commander_model::RegisterGuildCommanderModel,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad, repositories::guild_commanders::GuildCommandersPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let guild_commanders_repository = GuildCommandersPostgres::new(db_pool);
    let guild_commanders_usecase =
        GuildCommandersUsecase::new(Arc::new(guild_commanders_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(guild_commanders_usecase))
}

pub async fn register<T>(
    State(guild_commanders_usecase): State<Arc<GuildCommandersUsecase<T>>>,
    Json(register_guild_commander_model): Json<RegisterGuildCommanderModel>,
) -> impl IntoResponse
where
    T: GuildCommandersRepository + Send + Sync,
{
    match guild_commanders_usecase.register(register_guild_commander_model).await {
        Ok(guild_commander_id) => (
            StatusCode::CREATED,
            format!(
                "Guild Commander registered successfully with ID: {}",
                guild_commander_id
            )
            .into_response(),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", e.to_string()).into_response(),
        ),
    }
}
