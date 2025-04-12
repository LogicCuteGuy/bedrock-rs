use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::v662::types::NetworkItemInstanceDescriptor;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i32)]
#[enum_endianness(le)]
pub enum CreativeContentGroupCategory {
    All = 0,
    Construction = 1,
    Nature = 2,
    Equipment = 3,
    Items = 4,
    ItemCommandOnly = 5
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct CreativeContentGroups {
    pub category: CreativeContentGroupCategory,
    pub name: String,
    pub icon_item: NetworkItemInstanceDescriptor,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct CreativeContentItems {
    #[endianness(var)]
    pub entry_id : u32,
    pub item: NetworkItemInstanceDescriptor,
    #[endianness(var)]
    pub group_index : u32,
}


#[gamepacket(id = 145)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct CreativeContentPacket {
    #[vec_repr(i32)]
    #[vec_endianness(le)]
    pub groups: Vec<CreativeContentGroups>,
    #[vec_repr(i32)]
    #[vec_endianness(le)]
    pub items: Vec<CreativeContentItems>,
}