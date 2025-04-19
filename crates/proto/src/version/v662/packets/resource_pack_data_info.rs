use crate::version::v662::enums::PackType;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 82)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct ResourcePackDataInfoPacket {
    pub resource_name: String, // packid
    #[endianness(le)]
    pub chunk_size: u32,
    #[endianness(le)]
    pub chunk_amount: u32,
    #[endianness(le)]
    pub file_size: u64, // compressedPackSize
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub file_hash: Vec<u8>, // compressedPackHash
    pub is_premium: bool, // isPremium
    pub pack_type: PackType,
}