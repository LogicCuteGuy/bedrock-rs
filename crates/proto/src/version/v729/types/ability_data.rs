use bedrockrs_macros::ProtoCodec;
use crate::v662::enums::PermissionLevel;
use crate::v662::types::ActorUniqueID;

#[derive(ProtoCodec, Debug, Clone)]
pub struct AbilityData {
    /// This field is not necessary, 0 seems to work.
    pub target_player_id: ActorUniqueID,
    pub permission: PermissionLevel,
}
