//mod chat;
extern crate serde_mcproto;

pub mod packet;

pub use serde_mcproto::de::deserialize;
pub use serde_mcproto::ser::serialize;
pub use serde_mcproto::types;
pub use serde_mcproto::{de::MCProtoDeserializer, ser::MCProtoSerializer};

pub use serde_mcproto::read_varint;
pub use serde_mcproto::write_varint;

pub use serde_mcproto::error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
