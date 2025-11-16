use std::sync::Arc;

use axum::{
    Extension, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, post},
};

use crate::{
    application::usecases::crew_swithboard::CrewSwithboardUsecase,
    domain::repositories::{
        crew_swithboard::CrewSwithboardRepository, quest_viewing::QuestViewingRepository,
    },
    infrastructure::{
        axum_http::middlewares::adventures_authorization,
        postgres::{
            postgres_connection::PgPoolSquad,
            repositories::{
                crew_swithboard::CrewSwithboardPostgres, quest_viewing::QuestViewingPostgres,
            },
        },
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let crew_swithboard_repository = CrewSwithboardPostgres::new(db_pool.clone());
    let quest_viewing_repository = QuestViewingPostgres::new(db_pool);
    let crew_swithboard_usecase = CrewSwithboardUsecase::new(
        Arc::new(crew_swithboard_repository),
        Arc::new(quest_viewing_repository),
    );

    Router::new()
        .route("/join/:quest_id", post(join))
        .route("/leave/:quest_id", delete(leave))
        .route_layer(middleware::from_fn(adventures_authorization))
        .with_state(Arc::new(crew_swithboard_usecase))
}

pub async fn join<T1, T2>(
    State(crew_swithboard_usecase): State<Arc<CrewSwithboardUsecase<T1, T2>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewSwithboardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    match crew_swithboard_usecase.join(quest_id, adventurer_id).await {
        Ok(()) => (StatusCode::OK, "Quest joined successfully".into_response()),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", e.to_string()).into_response(),
        ),
    }
}

pub async fn leave<T1, T2>(
    State(crew_swithboard_usecase): State<Arc<CrewSwithboardUsecase<T1, T2>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewSwithboardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    match crew_swithboard_usecase.leave(quest_id, adventurer_id).await {
        Ok(()) => (StatusCode::OK, "Quest left successfully".into_response()),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", e.to_string()).into_response(),
        ),
    }
}
