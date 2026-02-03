use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

pub struct ChronoAsBson;

impl<'de> DeserializeAs<'de, chrono::DateTime<chrono::Utc>> for ChronoAsBson {
    fn deserialize_as<D>(deserializer: D) -> Result<chrono::DateTime<chrono::Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bson_dt = bson::DateTime::deserialize(deserializer)?;
        Ok(bson_dt.to_chrono())
    }
}

impl SerializeAs<chrono::DateTime<chrono::Utc>> for ChronoAsBson {
    fn serialize_as<S>(
        source: &chrono::DateTime<chrono::Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bson_dt = bson::DateTime::from_chrono(*source);
        bson_dt.serialize(serializer)
    }
}
