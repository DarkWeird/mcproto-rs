use serde::{Serialize, Serializer, Deserialize, Deserializer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    //TODO implement normally
    msg: String
}
