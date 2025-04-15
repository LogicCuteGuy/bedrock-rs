use crate::helper::ProtoHelper;
use crate::version::v766::gamepackets::GamePackets;

pub struct ProtoHelperV776;

impl ProtoHelper for ProtoHelperV776 {
    type GamePacketType = GamePackets;
}
