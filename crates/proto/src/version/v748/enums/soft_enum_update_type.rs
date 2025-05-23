use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u8)]
#[repr(u8)]
pub enum SoftEnumUpdateType {
    Add = 0,
    Remove = 1,
    Replace = 2,
}