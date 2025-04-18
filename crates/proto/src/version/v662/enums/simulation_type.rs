use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u8)]
#[repr(u8)]
pub enum SimulationType {
    Game = 0,
    Editor = 1,
    Test = 2,
    Invalid = 3,
}