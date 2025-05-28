use mongodb::bson::{Document, DateTime};
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

#[derive(Debug, Deserialize)]
pub struct Event {
    pub name: String,
    pub previous_version: Document,
    pub current_version: Document,
    pub occurred_at: DateTime,
    pub created_at: DateTime
}

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 5 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Event", 5)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("previous_version", &self.previous_version)?;
        state.serialize_field("current_version", &self.current_version)?;
        state.serialize_field("occurred_at", &self.occurred_at.to_string())?;
        state.serialize_field("created_at", &self.created_at.to_string())?;
        state.end()
    }
}