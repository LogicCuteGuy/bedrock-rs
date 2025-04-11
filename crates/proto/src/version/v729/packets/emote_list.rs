use bedrockrs_macros::{gamepacket, ProtoCodec};
use uuid::Uuid;
use crate::v662::types::ActorRuntimeID;

#[gamepacket(id = 152)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct EmoteListPacket {
    pub runtime_id: ActorRuntimeID,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub emote_piece_ids: Vec<Uuid>,
}
