use bedrockrs_macros::{gamepacket, ProtoCodec};
use vek::Vec2;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u8)]
#[repr(u8)]
pub enum Action {
    Set = 0,
    Clear = 1,
}

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u8)]
#[repr(u8)]
pub enum TargetMode {
    Angle = 0,
    Distance = 1,
}

#[gamepacket(id = 316)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct CameraAimAssistPacket {
    pub preset_id: String,
    #[endianness(le)]
    pub view_angle: Vec2<f32>,
    #[endianness(le)]
    pub distance: f32,
    pub target_mode: TargetMode,
    pub action: Action,
}