use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::v662::types::ActorRuntimeID;

#[gamepacket(id = 17)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct TakeItemActorPacket {
    pub item_runtime_id: ActorRuntimeID,
    pub actor_runtime_id: ActorRuntimeID,
}