use std::sync::Arc;

use anyhow::Result;
use chrono::{Duration, Utc};

use crate::{
    config::config_loader::{get_adventurer_secret_env, get_guild_commanders_secret_env},
    domain::repositories::{
        adventures::AdventuresRepository, guild_commanders::GuildCommandersRepository,
    },
    infrastructure::{
        argon2_hashing,
        jwt_authentication::{
            self,
            authentication_model::LoginModel,
            jwt_model::{Claims, Passport, Roles},
        },
        postgres::schema::quests::guild_commander_id,
    },
};

pub struct AuthenticationUsecase<T1, T2>
where
    T1: AdventuresRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    pub adventures_repository: Arc<T1>,
    pub guild_commanders_repository: Arc<T2>,
}

impl<T1, T2> AuthenticationUsecase<T1, T2>
where
    T1: AdventuresRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    pub fn new(adventures_repository: Arc<T1>, guild_commanders_repository: Arc<T2>) -> Self {
        Self {
            adventures_repository,
            guild_commanders_repository,
        }
    }

    pub async fn adventurer_login(&self, login_model: LoginModel) -> Result<Passport> {
        let secret_env = get_adventurer_secret_env()?;

        let adventurer = self
            .adventures_repository
            .find_by_username(login_model.username.clone())
            .await?;

        let original_password = adventurer.password;

        let password = login_model.password;

        if !argon2_hashing::verify(password, original_password)? {
            return Err(anyhow::anyhow!("Invalid password"));
        }

        let access_token = jwt_authentication::generate_token(
            secret_env.secret,
            &Claims {
                sub: adventurer.id.to_string(),
                role: Roles::Adventurer,
                exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
                iat: (Utc::now()).timestamp() as usize,
            },
        )?;

        let refresh_token = jwt_authentication::generate_token(
            secret_env.refresh_secret,
            &Claims {
                sub: adventurer.id.to_string(),
                role: Roles::Adventurer,
                exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
                iat: (Utc::now()).timestamp() as usize,
            },
        )?;

        Ok(Passport {
            access_token,
            refresh_token,
        })
    }

    pub async fn adventurer_refresh_token(&self, refresh_token: String) -> Result<Passport> {
        let secret_env = get_adventurer_secret_env()?;

        let claims =
            jwt_authentication::verify_token(secret_env.refresh_secret.clone(), refresh_token)?;

        let access_token = jwt_authentication::generate_token(
            secret_env.secret,
            &Claims {
                sub: claims.sub.clone(),
                role: Roles::Adventurer,
                exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
                iat: (Utc::now()).timestamp() as usize,
            },
        )?;

        let refresh_token_new = jwt_authentication::generate_token(
            secret_env.refresh_secret,
            &Claims {
                sub: claims.sub.clone(),
                role: Roles::Adventurer,
                exp: claims.exp,
                iat: (Utc::now()).timestamp() as usize,
            },
        )?;

        Ok(Passport {
            access_token,
            refresh_token: refresh_token_new,
        })
    }

    pub async fn guild_commander_login(&self, login_model: LoginModel) -> Result<Passport> {
        let secret_env = get_guild_commanders_secret_env()?;

        let guild_commander = self
            .guild_commanders_repository
            .find_by_username(login_model.username.clone())
            .await?;

        let original_password = guild_commander.password;

        let password = login_model.password;

        if !argon2_hashing::verify(password, original_password)? {
            return Err(anyhow::anyhow!("Invalid password"));
        }

        let access_token = jwt_authentication::generate_token(
            secret_env.secret,
            &Claims {
                sub: guild_commander.id.to_string(),
                role: Roles::GuildCommander,
                exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
                iat: (Utc::now()).timestamp() as usize,
            },
        )?;

        let refresh_token = jwt_authentication::generate_token(
            secret_env.refresh_secret,
            &Claims {
                sub: guild_commander.id.to_string(),
                role: Roles::GuildCommander,
                exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
                iat: (Utc::now()).timestamp() as usize,
            },
        )?;

        Ok(Passport {
            access_token,
            refresh_token,
        })
    }

    pub async fn guild_commander_refresh_token(&self, refresh_token: String) -> Result<Passport> {
        let secret_env = get_guild_commanders_secret_env()?;

        let claims =
            jwt_authentication::verify_token(secret_env.refresh_secret.clone(), refresh_token)?;

        let access_token = jwt_authentication::generate_token(
            secret_env.secret,
            &Claims {
                sub: claims.sub.clone(),
                role: Roles::GuildCommander,
                exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
                iat: (Utc::now()).timestamp() as usize,
            },
        )?;

        let refresh_token_new = jwt_authentication::generate_token(
            secret_env.refresh_secret,
            &Claims {
                sub: claims.sub.clone(),
                role: Roles::GuildCommander,
                exp: claims.exp,
                iat: (Utc::now()).timestamp() as usize,
            },
        )?;

        Ok(Passport {
            access_token,
            refresh_token: refresh_token_new,
        })
    }
}
