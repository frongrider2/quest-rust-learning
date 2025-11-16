use std::sync::Arc;

use ::cookie::time::Duration;
use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::IntoResponse,
    routing::post,
};
use axum_extra::extract::{CookieJar, cookie};
use cookie::Cookie;

use crate::{
    application::usecases::authentication::AuthenticationUsecase,
    config::{config_loader::get_stage, stage::Stage},
    domain::repositories::{
        adventures::AdventuresRepository, guild_commanders::GuildCommandersRepository,
    },
    infrastructure::{
        jwt_authentication::authentication_model::LoginModel,
        postgres::{
            postgres_connection::PgPoolSquad,
            repositories::{
                adventures::AdventurerPostgres, guild_commanders::GuildCommandersPostgres,
            },
        },
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let adventurer_repository = AdventurerPostgres::new(db_pool.clone());
    let guild_commanders_repository = GuildCommandersPostgres::new(db_pool);
    let authentication_usecase = AuthenticationUsecase::new(
        Arc::new(adventurer_repository),
        Arc::new(guild_commanders_repository),
    );

    Router::new()
        .route("/adventurers/login", post(adventurer_login))
        .route("/adventurers/refresh-token", post(adventurer_refresh_token))
        .route("/guild-commanders/login", post(guild_commander_login))
        .route(
            "/guild_commanders/refresh-token",
            post(guild_commander_refresh_token),
        )
        .with_state(Arc::new(authentication_usecase))
}

pub async fn adventurer_login<T1, T2>(
    State(authentication_usecase): State<Arc<AuthenticationUsecase<T1, T2>>>,
    Json(login_model): Json<LoginModel>,
) -> impl IntoResponse
where
    T1: AdventuresRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    match authentication_usecase.adventurer_login(login_model).await {
        Ok(passport) => {
            let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            if get_stage() == Stage::Production {
                act_cookie = act_cookie.secure(true);
                rft_cookie = rft_cookie.secure(true);
            }

            let mut headers = HeaderMap::new();

            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
            );

            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
            );

            (StatusCode::OK, headers, "Login successful").into_response()
        }
        Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    }
}

pub async fn adventurer_refresh_token<T1, T2>(
    State(authentication_usecase): State<Arc<AuthenticationUsecase<T1, T2>>>,
    jar: CookieJar,
) -> impl IntoResponse
where
    T1: AdventuresRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    if let Some(rtf) = jar.get("rft") {
        let refresh_token = rtf.value().to_string();

        let response = match authentication_usecase
            .adventurer_refresh_token(refresh_token)
            .await
        {
            Ok(passport) => {
                let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                if get_stage() == Stage::Production {
                    act_cookie = act_cookie.secure(true);
                    rft_cookie = rft_cookie.secure(true);
                }

                let mut headers = HeaderMap::new();

                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
                );

                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
                );

                (StatusCode::OK, headers, "Login successful").into_response()
            }
            Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
        };

        return response;
    }

    (StatusCode::UNAUTHORIZED, "Refresh token not found").into_response()
}

pub async fn guild_commander_login<T1, T2>(
    State(authentication_usecase): State<Arc<AuthenticationUsecase<T1, T2>>>,
    Json(login_model): Json<LoginModel>,
) -> impl IntoResponse
where
    T1: AdventuresRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    
    match authentication_usecase
        .guild_commander_login(login_model)
        .await
    {
        Ok(passport) => {
            let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            if get_stage() == Stage::Production {
                act_cookie = act_cookie.secure(true);
                rft_cookie = rft_cookie.secure(true);
            }

            let mut headers = HeaderMap::new();

            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
            );

            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
            );

            (StatusCode::OK, headers, "Login successful").into_response()
        }
        Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    }
}

pub async fn guild_commander_refresh_token<T1, T2>(
    State(authentication_usecase): State<Arc<AuthenticationUsecase<T1, T2>>>,
    jar: CookieJar,
) -> impl IntoResponse
where
    T1: AdventuresRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    if let Some(rtf) = jar.get("rft") {
        let refresh_token = rtf.value().to_string();

        let response = match authentication_usecase
            .guild_commander_refresh_token(refresh_token)
            .await
        {
            Ok(passport) => {
                let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                if get_stage() == Stage::Production {
                    act_cookie = act_cookie.secure(true);
                    rft_cookie = rft_cookie.secure(true);
                }

                let mut headers = HeaderMap::new();

                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
                );

                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
                );

                (StatusCode::OK, headers, "Login successful").into_response()
            }
            Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
        };

        return response;
    }

    (StatusCode::UNAUTHORIZED, "Refresh token not found").into_response()
}
