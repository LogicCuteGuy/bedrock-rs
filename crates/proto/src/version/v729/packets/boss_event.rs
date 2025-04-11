use crate::version::v729::types::event_type::BossEventType;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::v662::types::ActorUniqueID;

#[gamepacket(id = 74)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct BossEventPacket {
    pub actor_id: ActorUniqueID,
    pub event_type: BossEventType,
}
