use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::{ActorRuntimeID, DataItem, PropertySyncData};

#[gamepacket(id = 39)]
#[derive(ProtoCodec)]
pub struct SetActorDataPacket {
    pub target_runtime_id: ActorRuntimeID,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub actor_data: Vec<DataItem>, // VERIFY: vec_repr & vec_endianness
    pub synced_properties: PropertySyncData,
    #[endianness(var)]
    pub tick: u64,
}