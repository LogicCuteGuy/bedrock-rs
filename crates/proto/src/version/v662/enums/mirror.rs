use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u8)]
#[repr(u8)]
pub enum Mirror {
    None = 0,
    X = 1,
    Z = 2,
    XZ = 3,
}