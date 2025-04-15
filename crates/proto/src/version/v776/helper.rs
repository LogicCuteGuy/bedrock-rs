use crate::helper::ProtoHelper;
use crate::v776::gamepackets::GamePackets;

pub struct ProtoHelperV776;

impl ProtoHelper for ProtoHelperV776 {
    type GamePacketType = GamePackets;
}
