use bedrockrs_macros::ProtoCodec;

use crate::v662::enums::Dimension;
use crate::version::v729::types::spawn_biome_type::SpawnBiomeType;

#[derive(ProtoCodec, Debug, Clone)]
pub struct SpawnSettings {
    pub biome_type: SpawnBiomeType,
    pub user_defined_biome_name: String,
    pub dimension: Dimension,
}
