use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u8)]
#[repr(u8)]
pub enum ShowStoreOfferRedirectType {
    MarketplaceOffer = 0,
    DressingRoomOffer = 1,
    ThirdPartyServerPage = 2,
    Count = 3,
}