use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::v662::types::ActorUniqueID;

#[gamepacket(id = 14)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct RemoveEntityPacket {
    pub actor_id: ActorUniqueID,
}
