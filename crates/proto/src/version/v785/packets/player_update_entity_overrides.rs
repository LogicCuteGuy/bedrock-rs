use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::v662::types::ActorUniqueID;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u8)]
#[repr(u8)]
pub enum PlayerUpdateEntityOverridesType {
    ClearAll = 0,
    Remove = 1,
    SetInt {
        #[endianness(le)]
        value: i32,
    } = 2,
    SetFloat {
        #[endianness(le)]
        value: f32,
    } = 3,
}

#[gamepacket(id = 325)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct PlayerUpdateEntityOverridesPacket {
    pub target_id: ActorUniqueID,
    #[endianness(var)]
    pub property_index: u32,
    pub property_type: PlayerUpdateEntityOverridesType,
}