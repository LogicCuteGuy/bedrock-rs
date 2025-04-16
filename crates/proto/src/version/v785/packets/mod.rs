macro_rules! export {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
}

export!(update_client_options);
export!(player_video_capture);
export!(player_update_entity_overrides);