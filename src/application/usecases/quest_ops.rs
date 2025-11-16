use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::{quest_ops::QuestOpsRepository, quest_viewing::QuestViewingRepository},
    value_objects::quest_model::{AddQuestModel, EditQuestModel},
};

pub struct QuestOpsUsecase<T1, T2>
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    pub quest_ops_repository: Arc<T1>,
    pub quest_viewing_repository: Arc<T2>,
}

impl<T1, T2> QuestOpsUsecase<T1, T2>
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    pub fn new(quest_ops_repository: Arc<T1>, quest_viewing_repository: Arc<T2>) -> Self {
        Self {
            quest_ops_repository,
            quest_viewing_repository,
        }
    }

    pub async fn add(
        &self,
        guild_commander_id: i32,
        add_quest_model: AddQuestModel,
    ) -> Result<i32> {
        let add_quest_entity = add_quest_model.to_entity(guild_commander_id);
        let result = self.quest_ops_repository.add(add_quest_entity).await?;
        Ok(result)
    }

    pub async fn edit(
        &self,
        quest_id: i32,
        guild_commander_id: i32,
        edit_quest_model: EditQuestModel,
    ) -> Result<i32> {
        let adventurers_count = self
            .quest_viewing_repository
            .adventurers_counting_by_quest_id(quest_id)
            .await?;

        if adventurers_count > 0 {
            return Err(anyhow::anyhow!("Quest has adventurers, cannot delete"));
        }

        let edit_quest_entity = edit_quest_model.to_entity(guild_commander_id);
        let result = self
            .quest_ops_repository
            .edit(quest_id, edit_quest_entity)
            .await?;
        Ok(result)
    }

    pub async fn remove(&self, quest_id: i32, guild_commander_id: i32) -> Result<()> {
        let adventurers_count = self
            .quest_viewing_repository
            .adventurers_counting_by_quest_id(quest_id)
            .await?;

        if adventurers_count > 0 {
            return Err(anyhow::anyhow!("Quest has adventurers, cannot delete"));
        }

        let result = self
            .quest_ops_repository
            .remove(quest_id, guild_commander_id)
            .await?;
        Ok(result)
    }
}
