use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u8)]
#[repr(u8)]
pub enum ItemDescriptorInternalType {
    Invalid = 0,
    Default = 1,
    Molang = 2,
    ItemTag = 3,
    Deferred = 4,
    ComplexAlias = 5,
}
