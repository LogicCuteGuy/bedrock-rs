use bedrockrs_macros::{gamepacket, ProtoCodec};

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u8)]
#[repr(u8)]
enum ClientCameraAimAssistPacketAction {
    SetFromCameraPreset = 0,
    Clear = 1,
}

#[gamepacket(id = 321)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct ClientCameraAimAssistPacket {
    pub cameraPresetId: String,
    pub action: ClientCameraAimAssistPacketAction,
    pub allowAimAssist: bool,
}