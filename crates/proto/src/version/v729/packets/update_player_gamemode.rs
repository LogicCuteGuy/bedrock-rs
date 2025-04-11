use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::v662::enums::Gamemode;

#[gamepacket(id = 62)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct UpdatePlayerGamemodePacket {
    pub gamemode: Gamemode,
}
