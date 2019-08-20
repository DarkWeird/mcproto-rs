use serde::{Serialize, Serializer, Deserialize, Deserializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    //TODO implement normally
    msg: String
}
