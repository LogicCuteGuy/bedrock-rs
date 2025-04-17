use uuid::Uuid;

pub trait LoginChainData {
    fn get_username(&self) -> String;
    fn get_client_uuid(&self) -> Uuid;
    fn get_identity_public_key(&self) -> String;
    fn get_client_id(&self) -> u128;
    fn get_server_address(&self) -> String;
    fn get_device_model(&self) -> String;
    fn get_device_os(&self) -> u8;
    fn get_device_id(&self) -> String;
    fn get_game_version(&self) -> String;
    fn get_gui_scale(&self) -> i8;
    fn get_language_code(&self) -> String;
    fn get_xuid(&self) -> String;
    fn is_xbox_authed(&self) -> bool;
    fn get_current_input_mode(&self) -> i8;
    fn get_default_input_mode(&self) -> i8;
    fn get_cape_data(&self) -> String;
    fn get_ui_profile(&self) -> i8;
    fn get_max_view_distance(&self) -> i8;
    fn get_memory_tier(&self) -> i16;

    fn get_raw_data(&self) -> String;

}

#[derive(Clone, Debug)]
pub struct ClientChainData {
    username: String,
    client_uuid: Uuid,
    xuid: String,
    title_id: String,
}

impl LoginChainData for ClientChainData {

    fn get_username(&self) -> String {
        self.username.clone()
    }

    fn get_client_uuid(&self) -> Uuid {
        self.client_uuid.clone()
    }

    fn get_identity_public_key(&self) -> String {
        todo!()
    }

    fn get_client_id(&self) -> u128 {
        todo!()
    }

    fn get_server_address(&self) -> String {
        todo!()
    }

    fn get_device_model(&self) -> String {
        todo!()
    }

    fn get_device_os(&self) -> u8 {
        todo!()
    }

    fn get_device_id(&self) -> String {
        todo!()
    }

    fn get_game_version(&self) -> String {
        todo!()
    }

    fn get_gui_scale(&self) -> i8 {
        todo!()
    }

    fn get_language_code(&self) -> String {
        todo!()
    }

    fn get_xuid(&self) -> String {
        todo!()
    }

    fn is_xbox_authed(&self) -> bool {
        todo!()
    }

    fn get_current_input_mode(&self) -> i8 {
        todo!()
    }

    fn get_default_input_mode(&self) -> i8 {
        todo!()
    }

    fn get_cape_data(&self) -> String {
        todo!()
    }

    fn get_ui_profile(&self) -> i8 {
        todo!()
    }

    fn get_max_view_distance(&self) -> i8 {
        todo!()
    }

    fn get_memory_tier(&self) -> i16 {
        todo!()
    }

    fn get_raw_data(&self) -> String {
        todo!()
    }
}