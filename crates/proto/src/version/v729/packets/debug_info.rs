use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::v662::types::ActorUniqueID;

#[gamepacket(id = 155)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct DebugInfoPacket {
    pub actor_id: ActorUniqueID,
    pub data: String,
}
