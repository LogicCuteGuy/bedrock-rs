use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::v662::types::ActorRuntimeID;

#[gamepacket(id = 113)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct SetLocalPlayerAsInitializedPacket {
    pub target_actor_id: ActorRuntimeID,
}
