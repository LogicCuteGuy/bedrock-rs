use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u8)]
#[repr(u8)]
pub enum ModalFormCancelReason {
    UserClosed = 0,
    UserBusy = 1,
}
