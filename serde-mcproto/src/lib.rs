
pub mod de;
pub mod ser;
pub mod error;
pub mod types;

pub use de::read::read_varint;
pub use ser::write::write_varint;

