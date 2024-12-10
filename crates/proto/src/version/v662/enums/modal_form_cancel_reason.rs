use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i8)]
#[repr(i8)]
pub enum ModalFormCancelReason {
    UserClosed = 0,
    UserBusy = 1,
}
