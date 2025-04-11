use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::v662::enums::Difficulty;

#[gamepacket(id = 60)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct UpdateDifficultyPacket {
    pub difficulty: Difficulty,
}
