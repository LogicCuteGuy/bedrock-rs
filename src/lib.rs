#[cfg(feature = "level")]
pub mod level {
    pub use ::bedrockrs_level::*;
}

#[cfg(feature = "addon")]
pub mod addon {
    pub use ::bedrockrs_addon::*;
}

#[cfg(feature = "proto")]
pub mod proto {
    pub use ::bedrockrs_proto::*;
    pub use ::bedrockrs_proto_core::*;

    pub mod error {
        pub use ::bedrockrs_proto::error::*;
        pub use ::bedrockrs_proto_core::error::*;
    }
}

#[cfg(feature = "form")]
pub mod form {
    pub use ::bedrockrs_form::*;
}
