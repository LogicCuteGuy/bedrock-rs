use crate::error::ProtoCodecError;
use crate::ProtoCodec;
use std::io::Cursor;
use crate::byteorder::{ProtoCodecBE, ProtoCodecLE, ProtoCodecVAR};

macro_rules! impl_proto_slice {
    ($name:ident, $size:literal) => {
        impl<T: $name> $name for [T; $size] {
            fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
                for i in 0..$size {
                    self[i].proto_serialize(stream)?;
                }

                Ok(())
            }

            fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
                let mut buf = [0; $size];

                for i in 0..$size {
                    buf[i] = T::proto_deserialize(stream)?;
                }

                Ok(buf)
            }
        }
    };
}


impl_proto_slice!(ProtoCodec, 0);
impl_proto_slice!(ProtoCodec, 1);
impl_proto_slice!(ProtoCodec, 2);
impl_proto_slice!(ProtoCodec, 3);
impl_proto_slice!(ProtoCodec, 4);
impl_proto_slice!(ProtoCodec, 5);
impl_proto_slice!(ProtoCodec, 6);
impl_proto_slice!(ProtoCodec, 7);
impl_proto_slice!(ProtoCodec, 8);
impl_proto_slice!(ProtoCodec, 9);
impl_proto_slice!(ProtoCodec, 10);
impl_proto_slice!(ProtoCodec, 11);
impl_proto_slice!(ProtoCodec, 12);

impl_proto_slice!(ProtoCodecLE, 0);
impl_proto_slice!(ProtoCodecLE, 1);
impl_proto_slice!(ProtoCodecLE, 2);
impl_proto_slice!(ProtoCodecLE, 3);
impl_proto_slice!(ProtoCodecLE, 4);
impl_proto_slice!(ProtoCodecLE, 5);
impl_proto_slice!(ProtoCodecLE, 6);
impl_proto_slice!(ProtoCodecLE, 7);
impl_proto_slice!(ProtoCodecLE, 8);
impl_proto_slice!(ProtoCodecLE, 9);
impl_proto_slice!(ProtoCodecLE, 10);
impl_proto_slice!(ProtoCodecLE, 11);
impl_proto_slice!(ProtoCodecLE, 12);

impl_proto_slice!(ProtoCodecBE, 0);
impl_proto_slice!(ProtoCodecBE, 1);
impl_proto_slice!(ProtoCodecBE, 2);
impl_proto_slice!(ProtoCodecBE, 3);
impl_proto_slice!(ProtoCodecBE, 4);
impl_proto_slice!(ProtoCodecBE, 5);
impl_proto_slice!(ProtoCodecBE, 6);
impl_proto_slice!(ProtoCodecBE, 7);
impl_proto_slice!(ProtoCodecBE, 8);
impl_proto_slice!(ProtoCodecBE, 9);
impl_proto_slice!(ProtoCodecBE, 10);
impl_proto_slice!(ProtoCodecBE, 11);
impl_proto_slice!(ProtoCodecBE, 12);

impl_proto_slice!(ProtoCodecVAR, 0);
impl_proto_slice!(ProtoCodecVAR, 1);
impl_proto_slice!(ProtoCodecVAR, 2);
impl_proto_slice!(ProtoCodecVAR, 3);
impl_proto_slice!(ProtoCodecVAR, 4);
impl_proto_slice!(ProtoCodecVAR, 5);
impl_proto_slice!(ProtoCodecVAR, 6);
impl_proto_slice!(ProtoCodecVAR, 7);
impl_proto_slice!(ProtoCodecVAR, 8);
impl_proto_slice!(ProtoCodecVAR, 9);
impl_proto_slice!(ProtoCodecVAR, 10);
impl_proto_slice!(ProtoCodecVAR, 11);
impl_proto_slice!(ProtoCodecVAR, 12);
