use crate::helper::ProtoHelper;
use crate::v776::gamepackets::GamePackets;

pub struct ProtoHelperV785;

impl ProtoHelper for ProtoHelperV785 {
    type GamePacketType = GamePackets;
}
