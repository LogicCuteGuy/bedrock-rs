use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u8)]
#[repr(u8)]
pub enum ChatRestrictionLevel {
    None = 0,
    Dropped = 1,
    Disabled = 2,
}
