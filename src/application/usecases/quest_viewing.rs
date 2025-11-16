use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::quest_viewing::QuestViewingRepository,
    value_objects::{board_checking_filter::BoardCheckingFilter, quest_model::QuestModel},
};

pub struct QuestViewingUsecase<T>
where
    T: QuestViewingRepository + Send + Sync,
{
    pub quest_viewing_repository: Arc<T>,
}

impl<T> QuestViewingUsecase<T>
where
    T: QuestViewingRepository + Send + Sync,
{
    pub fn new(quest_viewing_repository: Arc<T>) -> Self {
        Self {
            quest_viewing_repository,
        }
    }

    pub async fn view_details(&self, quest_id: i32) -> Result<QuestModel> {
        let result = self.quest_viewing_repository.view_details(quest_id).await?;

        let adventurers_count = self
            .quest_viewing_repository
            .adventurers_counting_by_quest_id(quest_id)
            .await?;

        Ok(result.to_model(adventurers_count))
    }

    pub async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestModel>> {
        let results = self.quest_viewing_repository.board_checking(filter).await?;

        let mut quests_model = Vec::<QuestModel>::new();

        for result in results {
            let adventurers_count = self
                .quest_viewing_repository
                .adventurers_counting_by_quest_id(result.id)
                .await?;
            quests_model.push(result.to_model(adventurers_count));
        }

        Ok(quests_model)
    }

    pub async fn adventurers_counting_by_quest_id(&self, quest_id: i32) -> Result<i64> {
        let result = self
            .quest_viewing_repository
            .adventurers_counting_by_quest_id(quest_id)
            .await?;
        Ok(result)
    }
}
