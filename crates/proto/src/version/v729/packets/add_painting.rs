use bedrockrs_macros::{gamepacket, ProtoCodec};
use vek::Vec3;
use crate::v662::types::{ActorRuntimeID, ActorUniqueID};

#[gamepacket(id = 22)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct AddPaintingPacket {
    pub target_actor_id: ActorUniqueID,
    pub target_runtime_id: ActorRuntimeID,
    #[endianness(le)]
    pub position: Vec3<f32>,
    #[endianness(var)]
    pub direction: i32,
    pub motif: String,
}
