use serde::{Deserialize, Serialize};

use crate::domain::value_objects::quest_statuses::QuestStatuses;

// Serialize,Deserialize แปลง json
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BoardCheckingFilter {
    pub name: Option<String>,
    pub status: Option<QuestStatuses>,
}
