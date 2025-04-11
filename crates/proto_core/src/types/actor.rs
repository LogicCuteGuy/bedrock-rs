use std::io::Cursor;
use varint_rs::{VarintReader, VarintWriter};
use crate::error::ProtoCodecError;
use crate::ProtoCodec;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ActorRuntimeID(pub u64);

impl ProtoCodec for ActorRuntimeID {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        Ok(stream.write_u64_varint(self.0)?)
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        Ok(Self(stream.read_u64_varint()?))
    }

    fn get_size_prediction(&self) -> usize {
        size_of::<u64>()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ActorUniqueID(pub i64);

impl ProtoCodec for ActorUniqueID {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        Ok(stream.write_i64_varint(self.0)?)
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        Ok(Self(stream.read_i64_varint()?))
    }

    fn get_size_prediction(&self) -> usize {
        size_of::<i64>()
    }
}
