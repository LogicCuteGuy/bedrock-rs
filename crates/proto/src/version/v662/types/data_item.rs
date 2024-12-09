use crate::version::v662::enums::DataItemType;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct DataItem {
    #[endianness(var)]
    pub data_item_id: u32,
    pub data_item_type: DataItemType,
}