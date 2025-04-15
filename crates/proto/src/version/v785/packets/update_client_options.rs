use bedrockrs_macros::{gamepacket, ProtoCodec};

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u8)]
#[repr(u8)]
pub enum GraphicsMode {
    Simple = 0,
    Fancy = 1,
    Advanced = 2,
    RayTraced = 3,
}

#[gamepacket(id = 323)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct UpdateClientOptionsPacket {
    pub graphics_mode: Option<GraphicsMode>,
}