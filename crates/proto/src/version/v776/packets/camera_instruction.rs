use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::v776::types::CameraInstruction;

#[gamepacket(id = 300)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct CameraInstructionPacket {
    pub camera_instruction: CameraInstruction,
}