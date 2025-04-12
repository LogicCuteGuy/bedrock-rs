macro_rules! export {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
}

export!(client_camera_aim_assist);
export!(client_movement_prediction_sync);
export!(creative_content);
export!(item_registry_packet);
