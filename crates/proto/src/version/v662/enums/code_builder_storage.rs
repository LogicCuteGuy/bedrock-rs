use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u8)]
#[repr(u8)]
pub enum CodeBuilderStorageCategory {
    None = 0,
    CodeStatus = 1,
    Instantiation = 2,
}
#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u8)]
#[repr(u8)]
pub enum CodeBuilderStorageOperation {
    None = 0,
    Get = 1,
    Set = 2,
    Reset = 3,
}