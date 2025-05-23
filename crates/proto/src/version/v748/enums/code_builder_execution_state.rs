use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u8)]
#[repr(u8)]
pub enum CodeBuilderExecutionState {
    None = 0,
    NotStarted = 1,
    InProgress = 2,
    Paused = 3,
    Error = 4,
    Succeeded = 5,
}