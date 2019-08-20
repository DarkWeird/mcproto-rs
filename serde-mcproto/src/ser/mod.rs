use serde::{Serialize, Serializer};

pub use ser::MCProtoSerializer;

mod ser;
pub(crate) mod write;


pub fn serialize<T, S>(data: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: ?Sized + Serialize,
        S: Serializer,
{
    Serialize::serialize(data, serializer)
}
