use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::get,
};

use crate::{
    application::usecases::quest_viewing::QuestViewingUsecase,
    domain::{
        repositories::quest_viewing::QuestViewingRepository,
        value_objects::board_checking_filter::BoardCheckingFilter,
    },
    infrastructure::{
        axum_http::middlewares::adventures_authorization,
        postgres::{
            postgres_connection::PgPoolSquad, repositories::quest_viewing::QuestViewingPostgres,
        },
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let quest_viewing_repository = QuestViewingPostgres::new(db_pool);
    let quest_viewing_usecase = QuestViewingUsecase::new(Arc::new(quest_viewing_repository));

    Router::new()
        .route("/:quest_id", get(view_details))
        .route("/board-checking", get(board_checking))
        .route("/adventurers-count", get(adventurers_counting_by_quest_id))
        // .route_layer(middleware::from_fn(adventures_authorization))
        .with_state(Arc::new(quest_viewing_usecase))
}

pub async fn view_details<T>(
    State(quest_viewing_usecase): State<Arc<QuestViewingUsecase<T>>>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T: QuestViewingRepository + Send + Sync,
{
    match quest_viewing_usecase.view_details(quest_id).await {
        Ok(quest_model) => (StatusCode::OK, Json(quest_model).into_response()),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", e.to_string()).into_response(),
        ),
    }
}

pub async fn board_checking<T>(
    State(quest_viewing_usecase): State<Arc<QuestViewingUsecase<T>>>,
    Query(filter): Query<BoardCheckingFilter>,
) -> impl IntoResponse
where
    T: QuestViewingRepository + Send + Sync,
{
    match quest_viewing_usecase.board_checking(&filter).await {
        Ok(quest_models) => (StatusCode::OK, Json(quest_models).into_response()),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", e.to_string()).into_response(),
        ),
    }
}

pub async fn adventurers_counting_by_quest_id<T>(
    State(quest_viewing_usecase): State<Arc<QuestViewingUsecase<T>>>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T: QuestViewingRepository + Send + Sync,
{
    match quest_viewing_usecase
        .adventurers_counting_by_quest_id(quest_id)
        .await
    {
        Ok(adventurers_count) => (StatusCode::OK, Json(adventurers_count).into_response()),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", e.to_string()).into_response(),
        ),
    }
}
