use std::sync::Arc;

use crate::{
    domain::{
        repositories::adventures::AdventuresRepository,
        value_objects::adventurer_model::RegisterAdventurerModel,
    },
    infrastructure::argon2_hashing,
};
use anyhow::Result;

pub struct AdventuresUsecase<T>
where
    T: AdventuresRepository + Send + Sync,
{
    pub adventures_repository: Arc<T>,
}

impl<T> AdventuresUsecase<T>
where
    T: AdventuresRepository + Send + Sync,
{
    pub fn new(adventures_repository: Arc<T>) -> Self {
        Self {
            adventures_repository,
        }
    }

    pub async fn register(
        &self,
        mut register_adventurer_model: RegisterAdventurerModel,
    ) -> Result<i32> {
        let hashed_password = argon2_hashing::hash(register_adventurer_model.password.clone())?;

        register_adventurer_model.password = hashed_password;
        let register_adventurer_entity = register_adventurer_model.to_entity();
        let result = self
            .adventures_repository
            .register(register_adventurer_entity)
            .await?;
        
        Ok(result)
    }
}
