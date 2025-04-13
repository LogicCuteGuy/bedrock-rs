use std::str::FromStr;
use serde_json::Value;
use crate::version::v662::enums::{AnimatedTextureType, AnimationExpression};
use bedrockrs_macros::ProtoCodec;
use bedrockrs_proto_core::error::ProtoCodecError;

#[derive(ProtoCodec, Clone, Debug)]
pub struct SerializedSkinAnimationFrame {
    #[endianness(le)]
    pub image_width: u32,
    #[endianness(le)]
    pub image_height: u32,
    pub image_bytes: String,
    pub animation_type: AnimatedTextureType,
    #[endianness(le)]
    pub frame_count: f32,
    pub animation_expression: AnimationExpression,
}

impl SerializedSkinAnimationFrame {
    pub fn decode(json: &Value) -> Result<Self, ProtoCodecError> {
        let image_width = json.get("ImageWidth")
            .and_then(|v| v.as_u64())
            .unwrap_or_default() as u32;

        let image_height = json.get("ImageHeight")
            .and_then(|v| v.as_u64())
            .unwrap_or_default() as u32;

        let image_bytes = json.get("ImageBytes")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let animation_type = json.get("AnimationType")
            .and_then(|v| v.as_str())
            .and_then(|s| AnimatedTextureType::from_str(s).ok())
            .unwrap_or(AnimatedTextureType::None);

        let frame_count = json.get("FrameCount")
            .and_then(|v| v.as_f64())
            .unwrap_or_default() as f32;

        let animation_expression = json.get("AnimationExpression")
            .and_then(|v| v.as_str())
            .and_then(|s| AnimationExpression::from_str(s).ok())
            .unwrap_or(AnimationExpression::Linear);

        Ok(Self {
            image_width,
            image_height,
            image_bytes,
            animation_type,
            frame_count,
            animation_expression,
        })
    }
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct PersonaPiecesEntry {
    pub piece_id: String,
    pub piece_type: String,
    pub pack_id: String,
    pub is_default_piece: bool,
    pub product_id: String,
}

impl PersonaPiecesEntry {
    pub fn decode(json: &Value) -> Result<Self, ProtoCodecError> {
        Ok(Self {
            piece_id: json.get("piece_id")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            piece_type: json.get("piece_type")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            pack_id: json.get("pack_id")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            is_default_piece: json.get("is_default_piece")
                .and_then(|v| v.as_bool())
                .unwrap_or_default(),
            product_id: json.get("product_id")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
        })
    }
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct PieceTintColorsEntry {
    pub piece_type: String,
    pub piece_tint_color: String,
}

impl PieceTintColorsEntry {
    pub fn decode(json: &Value) -> Result<Self, ProtoCodecError> {
        Ok(Self {
            piece_type: json.get("piece_type")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            piece_tint_color: json.get("piece_tint_color")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
        })
    }
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct SerializedSkin {
    pub skin_id: String,
    pub play_fab_id: String,
    pub skin_resource_patch: String,
    #[endianness(le)]
    pub skin_image_width: u32,
    #[endianness(le)]
    pub skin_image_height: u32,
    pub skin_image_bytes: String,
    #[vec_repr(u32)]
    #[vec_endianness(le)]
    pub animations: Vec<SerializedSkinAnimationFrame>,
    #[endianness(le)]
    pub cape_image_width: u32,
    #[endianness(le)]
    pub cape_image_height: u32,
    pub cape_image_bytes: String,
    pub geometry_data: String,
    pub geometry_data_engine_version: String,
    pub animation_data: String,
    pub cape_id: String,
    pub full_id: String,
    pub arm_size: String,
    pub skin_color: String,
    #[vec_repr(u32)]
    #[vec_endianness(le)]
    pub persona_pieces: Vec<PersonaPiecesEntry>,
    #[vec_repr(u32)]
    #[vec_endianness(le)]
    pub piece_tint_colors: Vec<PieceTintColorsEntry>,
    pub is_premium_skin: bool,
    pub is_persona_skin: bool,
    pub is_persona_cape_on_classic_skin: bool,
    pub is_primary_user: bool,
    pub overrides_player_appearance: bool,
}

impl SerializedSkin {
    pub fn decode(json: &Value) -> Result<Self, ProtoCodecError> {
        let skin_id = json.get("SkinId")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let play_fab_id = json.get("PlayFabId")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let skin_resource_patch = json.get("SkinResourcePatch")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let skin_image_width = json.get("SkinImageWidth")
            .and_then(|v| v.as_u64())
            .unwrap_or_default() as u32;

        let skin_image_height = json.get("SkinImageHeight")
            .and_then(|v| v.as_u64())
            .unwrap_or_default() as u32;

        let skin_image_bytes = json.get("SkinImageBytes")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let animations = json.get("Animations")
            .and_then(|v| v.as_array())
            .unwrap_or(&vec![])
            .iter()
            .map(|a| SerializedSkinAnimationFrame::decode(a))
            .collect::<Result<Vec<_>, ProtoCodecError>>()?;

        let cape_image_width = json.get("CapeImageWidth")
            .and_then(|v| v.as_u64())
            .unwrap_or_default() as u32;

        let cape_image_height = json.get("CapeImageHeight")
            .and_then(|v| v.as_u64())
            .unwrap_or_default() as u32;

        let cape_image_bytes = json.get("CapeImageBytes")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let geometry_data = json.get("GeometryData")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let geometry_data_engine_version = json.get("GeometryDataEngineVersion")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let animation_data = json.get("AnimationData")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let cape_id = json.get("CapeId")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let full_id = json.get("FullId")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let arm_size = json.get("ArmSize")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let skin_color = json.get("SkinColor")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let persona_pieces = json.get("PersonaPieces")
            .and_then(|v| v.as_array())
            .unwrap_or(&vec![])
            .iter()
            .map(|p| PersonaPiecesEntry::decode(p))
            .collect::<Result<Vec<_>, ProtoCodecError>>()?;

        let piece_tint_colors = json.get("PieceTintColors")
            .and_then(|v| v.as_array())
            .unwrap_or(&vec![])
            .iter()
            .map(|p| PieceTintColorsEntry::decode(p))
            .collect::<Result<Vec<_>, ProtoCodecError>>()?;

        let is_premium_skin = json.get("IsPremiumSkin")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let is_persona_skin = json.get("IsPersonaSkin")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let is_persona_cape_on_classic_skin = json.get("IsPersonaCapeOnClassicSkin")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let is_primary_user = json.get("IsPrimaryUser")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let overrides_player_appearance = json.get("OverridesPlayerAppearance")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        Ok(Self {
            skin_id,
            play_fab_id,
            skin_resource_patch,
            skin_image_width,
            skin_image_height,
            skin_image_bytes,
            animations,
            cape_image_width,
            cape_image_height,
            cape_image_bytes,
            geometry_data,
            geometry_data_engine_version,
            animation_data,
            cape_id,
            full_id,
            arm_size,
            skin_color,
            persona_pieces,
            piece_tint_colors,
            is_premium_skin,
            is_persona_skin,
            is_persona_cape_on_classic_skin,
            is_primary_user,
            overrides_player_appearance,
        })
    }
}