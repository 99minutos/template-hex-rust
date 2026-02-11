use crate::domain::users::{User, UserId};
use crate::infrastructure::serde::chrono_bson::ChronoAsBson;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{IfIsHumanReadable, serde_as};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<IfIsHumanReadable<serde_with::DisplayFromStr>>")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    #[serde_as(as = "ChronoAsBson")]
    pub created_at: DateTime<Utc>,
    #[serde_as(as = "ChronoAsBson")]
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<ChronoAsBson>")]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<User> for UserDocument {
    fn from(user: User) -> Self {
        Self {
            id: user.id.and_then(|id| ObjectId::parse_str(&*id).ok()),
            name: user.name,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
        }
    }
}

impl From<UserDocument> for User {
    fn from(doc: UserDocument) -> Self {
        Self {
            id: doc.id.map(|oid| UserId::new(oid.to_hex())),
            name: doc.name,
            email: doc.email,
            created_at: doc.created_at,
            updated_at: doc.updated_at,
            deleted_at: doc.deleted_at,
        }
    }
}
