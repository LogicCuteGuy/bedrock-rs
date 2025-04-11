use bedrockrs_macros::{gamepacket, ProtoCodec};
use xuid::Xuid;
use crate::v662::types::ActorRuntimeID;

#[gamepacket(id = 138)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct EmotePacket {
    runtime_id: ActorRuntimeID,
    emote_id: String,
    /// Emote length measured in ticks.
    #[endianness(var)]
    emote_length: u32,
    xuid: Xuid,
    platform_id: String,
    // TODO: Turn this into an enum
    flags: i8,
}
