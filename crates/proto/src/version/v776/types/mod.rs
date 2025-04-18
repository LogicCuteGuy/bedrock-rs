macro_rules! export {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
}

export!(structure_editor_data);
export!(camera_instruction);
