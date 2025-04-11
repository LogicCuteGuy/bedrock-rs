use crate::version::v729::types::credits_state::CreditsState;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::v662::types::ActorRuntimeID;

#[gamepacket(id = 75)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ShowCreditsPacket {
    pub target_actor_id: ActorRuntimeID,
    pub credits_state: CreditsState,
}
