use serde::{Deserialize, Serialize};
use serde_with::{IfIsHumanReadable, serde_as};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<IfIsHumanReadable<serde_with::DisplayFromStr>>")]
    pub id: Option<bson::oid::ObjectId>,
    pub name: String,
    pub email: String,
    #[serde_as(as = "crate::infrastructure::serde::chrono_bson::ChronoAsBson")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde_as(as = "crate::infrastructure::serde::chrono_bson::ChronoAsBson")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
