use bedrockrs_core::int::VAR;
use bedrockrs_proto_derive::ProtoCodec;
use uuid::Uuid;
#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(VAR::<u32>)]
pub enum CommandOriginType {
    Player = 0,
    CommandBlock = 1,
    MinecartCommandBlock = 2,
    DevConsole = 3,
    Test = 4,
    AutomationPlayer = 5,
    ClientAutomation = 6,
    DedicatedServer = 7,
    Entity = 8,
    Virtual = 9,
    GameArgument = 10,
    EntityServer = 11,
    Precompiled = 12,
    GameDirectorEntityServer = 13,
    Scripting = 14,
    ExecuteContext = 15,
}

#[derive(ProtoCodec, Debug, Clone)]
pub struct CommandOriginData {
    command_type: CommandOriginType,
    command_uuid: Uuid,
    request_id: String,
}
