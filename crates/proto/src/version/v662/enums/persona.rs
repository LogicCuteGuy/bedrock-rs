use std::str::FromStr;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u32)]
#[enum_endianness(le)]
#[repr(u32)]
pub enum AnimatedTextureType {
    None = 0,
    Face = 1,
    Body32x32 = 2,
    Body128x128 = 3,
}

impl FromStr for AnimatedTextureType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(AnimatedTextureType::None),
            "Face" => Ok(AnimatedTextureType::Face),
            "Body32x32" => Ok(AnimatedTextureType::Body32x32),
            "Body128x128" => Ok(AnimatedTextureType::Body128x128),
            _ => Err(()),
        }
    }
}

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(u32)]
#[enum_endianness(le)]
#[repr(u32)]
pub enum AnimationExpression {
    Linear = 0,
    Blinking = 1,
}

impl FromStr for AnimationExpression {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Linear" => Ok(AnimationExpression::Linear),
            "Blinking" => Ok(AnimationExpression::Blinking),
            _ => Err(()),
        }
    }
}