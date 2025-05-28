use chrono::Utc;
use serde::{ser::SerializeStruct, Serialize, Serializer};

use super::User;

pub struct UserCreated {
    pub previous_version: Option<User>,
    pub current_version: User,
    pub actor: String
}

impl Serialize for UserCreated {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 6 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("UserCreated", 6)?;
        state.serialize_field("collection", "user")?;
        state.serialize_field("name", "created")?;
        state.serialize_field("previous_version", &self.previous_version)?;
        state.serialize_field("current_version", &self.current_version)?;
        state.serialize_field("occurred_at", &Utc::now().to_string())?;
        state.serialize_field("actor", &self.actor)?;
        state.end()
    }
}