use crate::domain::user::{User, UserId};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub created_at: bson::DateTime,
    pub updated_at: bson::DateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<bson::DateTime>,
}

impl From<User> for UserDocument {
    fn from(entity: User) -> Self {
        Self {
            id: entity
                .id
                .as_ref()
                .and_then(|id| ObjectId::parse_str(&**id).ok()),
            name: entity.name,
            email: entity.email,
            created_at: bson::DateTime::from_chrono(entity.created_at),
            updated_at: bson::DateTime::from_chrono(entity.updated_at),
            deleted_at: entity.deleted_at.map(bson::DateTime::from_chrono),
        }
    }
}

impl From<UserDocument> for User {
    fn from(doc: UserDocument) -> Self {
        Self {
            id: doc.id.map(|oid| UserId::new(oid.to_hex())),
            name: doc.name,
            email: doc.email,
            created_at: doc.created_at.to_chrono(),
            updated_at: doc.updated_at.to_chrono(),
            deleted_at: doc.deleted_at.map(|dt| dt.to_chrono()),
        }
    }
}
