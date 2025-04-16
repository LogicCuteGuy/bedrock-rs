macro_rules! export {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
}

export!(client_camera_aim_assist);
export!(client_movement_prediction_sync);
export!(creative_content);
export!(item_registry);
export!(camera_aim_assist_presets);
export!(command_block_update);
export!(start_game);
export!(camera_instruction);