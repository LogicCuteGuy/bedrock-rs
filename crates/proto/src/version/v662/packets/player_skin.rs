use crate::version::v662::types::SerializedSkin;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use uuid::Uuid;

#[gamepacket(id = 93)]
#[derive(ProtoCodec, Clone, Debug)]
#[allow(proto_gen)]
pub struct PlayerSkinPacket {
    pub uuid: Uuid,
    pub serialized_skin: SerializedSkin,
    pub new_skin_name: String,
    pub old_skin_name: String,
    pub trusted_marketplace_skin: bool,
}