use bedrockrs_macros::ProtoCodec;


#[derive(ProtoCodec)]
pub enum ContainerID {
    None = -1,
    Inventory = 0,
    First = 1,
    Last = 100,
    Offhand = 119,
    Armor = 120,
    SelectionSlots = 122,
    PlayerOnlyUI = 124,
}
