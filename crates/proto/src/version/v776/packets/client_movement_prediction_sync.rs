use vek::Vec3;
use bedrockrs_macros::{gamepacket, ProtoCodec};
use bedrockrs_proto_core::types::bitset::BitSet;
use crate::v662::types::ActorUniqueID;

#[derive(ProtoCodec, Clone, Debug)]
pub struct MovementAttributesComponent {
    #[endianness(le)]
    pub movement_speed: f32,
    #[endianness(le)]
    pub underwater_movement_speed: f32,
    #[endianness(le)]
    pub lava_movement_speed: f32,
    #[endianness(le)]
    pub jump_strength: f32,
    #[endianness(le)]
    pub health: f32,
    #[endianness(le)]
    pub hunger: f32,
}

#[gamepacket(id = 322)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct ClientMovementPredictionSyncPacket {
    pub actor_data_flag: BitSet<123>,
    #[endianness(le)]
    pub actor_data_bounding_box: Vec3<f32>,
    pub movement_attributes_component: MovementAttributesComponent,
    pub actor_unique_id: ActorUniqueID,
    pub actor_flying_state: bool,
}