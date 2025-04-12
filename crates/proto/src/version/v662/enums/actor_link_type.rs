use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u8)]
#[repr(u8)]
pub enum ActorLinkType {
    None = 0,
    Riding = 1,
    Passenger = 2,
}
