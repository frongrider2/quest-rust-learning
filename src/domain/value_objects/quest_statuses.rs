use std::fmt;
use std::io::Write;

use diesel::{
    deserialize::{self, FromSql},
    pg::Pg,
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Varchar,
    AsExpression, FromSqlRow,
};
use serde::{Deserialize, Serialize};

// Serialize,Deserialize แปลง json
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Varchar)]
pub enum QuestStatuses {
    #[default]
    Open,
    InJourney,
    Completed,
    Failed,
}

impl fmt::Display for QuestStatuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuestStatuses::Open => write!(f, "Open"),
            QuestStatuses::InJourney => write!(f, "InJourney"),
            QuestStatuses::Completed => write!(f, "Completed"),
            QuestStatuses::Failed => write!(f, "Failed"),
        }
    }
}

impl FromSql<Varchar, Pg> for QuestStatuses {
    fn from_sql(bytes: <Pg as diesel::backend::Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <String as FromSql<Varchar, Pg>>::from_sql(bytes)?;
        match value.as_str() {
            "Open" => Ok(QuestStatuses::Open),
            "InJourney" => Ok(QuestStatuses::InJourney),
            "Completed" => Ok(QuestStatuses::Completed),
            "Failed" => Ok(QuestStatuses::Failed),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl ToSql<Varchar, Pg> for QuestStatuses {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let value = self.to_string();
        out.write_all(value.as_bytes())?;
        Ok(IsNull::No)
    }
}
