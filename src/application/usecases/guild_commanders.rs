use std::sync::Arc;

use anyhow::Result;

use crate::{domain::{
    repositories::guild_commanders::GuildCommandersRepository,
    value_objects::guild_commander_model::RegisterGuildCommanderModel,
}, infrastructure::argon2_hashing};

pub struct GuildCommandersUsecase<T>
where
    T: GuildCommandersRepository + Send + Sync,
{
    pub guild_commanders_repository: Arc<T>,
}

impl<T> GuildCommandersUsecase<T>
where
    T: GuildCommandersRepository + Send + Sync,
{
    pub fn new(guild_commanders_repository: Arc<T>) -> Self {
        Self {
            guild_commanders_repository,
        }
    }

    pub async fn register(
        &self,
        mut register_guild_commander_model: RegisterGuildCommanderModel,
    ) -> Result<i32> {
        let hashed_password = argon2_hashing::hash(register_guild_commander_model.password.clone())?;

        register_guild_commander_model.password = hashed_password;

        let register_adventurer_entity = register_guild_commander_model.to_entity();
        let result = self
            .guild_commanders_repository
            .register(register_adventurer_entity)
            .await?;

        Ok(result)
    }
}
