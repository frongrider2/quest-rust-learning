use std::sync::Arc;

use crate::domain::{
    repositories::{
        crew_swithboard::CrewSwithboardRepository, quest_viewing::QuestViewingRepository,
    },
    value_objects::{
        quest_adventurer_juntion::QuestAdventurerJunction, quest_statuses::QuestStatuses,
    },
};
use anyhow::Result;

pub struct CrewSwithboardUsecase<T1, T2>
where
    T1: CrewSwithboardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    pub crew_swithboard_repository: Arc<T1>,
    pub quest_viewing_repository: Arc<T2>,
}

impl<T1, T2> CrewSwithboardUsecase<T1, T2>
where
    T1: CrewSwithboardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    pub fn new(crew_swithboard_repository: Arc<T1>, quest_viewing_repository: Arc<T2>) -> Self {
        Self {
            crew_swithboard_repository,
            quest_viewing_repository,
        }
    }

    pub async fn join(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        let quest = self.quest_viewing_repository.view_details(quest_id).await?;

        let quest_status_condition = quest.status.to_string() == QuestStatuses::Open.to_string()
            || quest.status.to_string() == QuestStatuses::Failed.to_string();

        if !quest_status_condition {
            return Err(anyhow::anyhow!("Quest is not join "));
        }

        let junction_body = QuestAdventurerJunction {
            adventurer_id,
            quest_id,
        };

        self.crew_swithboard_repository.join(junction_body).await?;

        Ok(())
    }

    pub async fn leave(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        let quest = self.quest_viewing_repository.view_details(quest_id).await?;

        let quest_status_condition = quest.status.to_string() == QuestStatuses::Open.to_string()
            || quest.status.to_string() == QuestStatuses::Failed.to_string();

        if !quest_status_condition {
            return Err(anyhow::anyhow!("Quest is not leave "));
        }

        let junction_body = QuestAdventurerJunction {
            adventurer_id,
            quest_id,
        };

        self.crew_swithboard_repository.leave(junction_body).await?;

        Ok(())
    }
}
