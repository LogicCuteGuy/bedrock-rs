use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u8)]
#[repr(u8)]
pub enum AnimationMode {
    None = 0,
    Layers = 1,
    Blocks = 2,
}