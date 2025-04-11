use bedrockrs_macros::{gamepacket, ProtoCodec};
use vek::Vec3;
use crate::v662::enums::Dimension;

#[gamepacket(id = 61)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ChangeDimensionPacket {
    pub dimension: Dimension,
    #[endianness(le)]
    pub pos: Vec3<f32>,
    pub respawn: bool,
    #[endianness(le)]
    pub loading_screen: Option<u32>,
}
