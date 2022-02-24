use serde::{Deserialize, Serialize};
use struct_deser::{FromBytesOrdered, IntoBytesOrdered, SerializedByteLen};

pub mod area;
pub mod belt;
pub mod blueprint;
pub mod building;
pub mod enums;
pub mod station;
pub mod vec;
pub mod traits;

// Because StructDeser isn't perfect
#[derive(Serialize, Deserialize)]
struct F32(f32);

impl SerializedByteLen for F32 {
    const BYTE_LEN: usize = 4;
}

impl FromBytesOrdered for F32 {
    fn from_bytes<BO: struct_deser::byteorder::ByteOrder>(bytes: &[u8]) -> Self {
        Self(BO::read_f32(bytes))
    }
}

impl IntoBytesOrdered for F32 {
    fn into_bytes<BO: struct_deser::byteorder::ByteOrder>(&self, bytes: &mut [u8]) {
        BO::write_f32(bytes, self.0);
    }
}
