use bedrockrs_macros::{gamepacket, ProtoCodec};

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u8)]
#[repr(u8)]
pub enum PlayerVideoCaptureAction {
    Stop = 0,
    Start {
        #[endianness(le)]
        frame_rate: i32,
        file_prefix: String,
    } = 1,
}

#[gamepacket(id = 324)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct PlayerVideoCapturePacket {
    pub player_video_capture_action: PlayerVideoCaptureAction,
}