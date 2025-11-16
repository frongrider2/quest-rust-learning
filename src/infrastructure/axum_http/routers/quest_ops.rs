use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, patch, post},
};

use crate::{
    application::usecases::quest_ops::QuestOpsUsecase,
    domain::{
        repositories::{quest_ops::QuestOpsRepository, quest_viewing::QuestViewingRepository},
        value_objects::quest_model::{AddQuestModel, EditQuestModel},
    },
    infrastructure::{
        axum_http::middlewares::guild_commanders_authorization,
        postgres::{
            postgres_connection::PgPoolSquad,
            repositories::{quest_ops::QuestOpsPostgres, quest_viewing::QuestViewingPostgres},
        },
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let quest_ops_repository = QuestOpsPostgres::new(db_pool.clone());
    let quest_viewing_repository = QuestViewingPostgres::new(db_pool);
    let quest_ops_usecase = QuestOpsUsecase::new(
        Arc::new(quest_ops_repository),
        Arc::new(quest_viewing_repository),
    );

    Router::new()
        .route("/", post(quest_add))
        .route("/:quest_id", patch(quest_edit))
        .route("/:quest_id", delete(quest_remove))
        .route_layer(middleware::from_fn(guild_commanders_authorization))
        .with_state(Arc::new(quest_ops_usecase))
}

pub async fn quest_add<T1, T2>(
    State(quest_ops_usecase): State<Arc<QuestOpsUsecase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Json(add_quest_model): Json<AddQuestModel>,
) -> impl IntoResponse
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    match quest_ops_usecase
        .add(guild_commander_id, add_quest_model)
        .await
    {
        Ok(quest_id) => (
            StatusCode::CREATED,
            format!("Quest Add successfully with ID: {}", quest_id).into_response(),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", e.to_string()).into_response(),
        ),
    }
}

pub async fn quest_edit<T1, T2>(
    State(quest_ops_usecase): State<Arc<QuestOpsUsecase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
    Json(edit_quest_model): Json<EditQuestModel>,
) -> impl IntoResponse
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    match quest_ops_usecase
        .edit(quest_id, guild_commander_id, edit_quest_model)
        .await
    {
        Ok(quest_id) => (
            StatusCode::CREATED,
            format!("Quest Edit successfully with ID: {}", quest_id).into_response(),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", e.to_string()).into_response(),
        ),
    }
}

pub async fn quest_remove<T1, T2>(
    State(quest_ops_usecase): State<Arc<QuestOpsUsecase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    match quest_ops_usecase.remove(quest_id, guild_commander_id).await {
        Ok(_) => (
            StatusCode::CREATED,
            format!("Quest Delete successfully with ID: {}", quest_id).into_response(),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", e.to_string()).into_response(),
        ),
    }
}
