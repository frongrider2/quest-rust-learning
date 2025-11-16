use anyhow::Result;
use axum::async_trait;
use diesel::{delete, dsl::insert_into, prelude::*};
use std::sync::Arc;

use crate::{
    domain::{
        repositories::crew_swithboard::CrewSwithboardRepository,
        value_objects::quest_adventurer_juntion::QuestAdventurerJunction,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad, schema::quest_adventurer_junction,
    },
};

pub struct CrewSwithboardPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl CrewSwithboardPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl CrewSwithboardRepository for CrewSwithboardPostgres {
    async fn join(&self, junction_body: QuestAdventurerJunction) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        insert_into(quest_adventurer_junction::table)
            .values(junction_body)
            .execute(&mut conn)?;

        Ok(())
    }
    async fn leave(&self, junction_body: QuestAdventurerJunction) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        delete(quest_adventurer_junction::table)
            .filter(quest_adventurer_junction::quest_id.eq(junction_body.quest_id))
            .filter(quest_adventurer_junction::adventurer_id.eq(junction_body.adventurer_id))
            .execute(&mut conn)?;

        Ok(())
    }
}
