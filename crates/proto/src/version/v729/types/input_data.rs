use crate::version::v729::types::block_actions::BlockActions;
use vek::Vec2;
use crate::v662::types::ActorUniqueID;

#[derive(Debug, Clone)]
pub struct InputData {
    pub ascend: bool,
    pub descend: bool,
    pub north_jump_deprecated: bool,
    pub jump_down: bool,
    pub sprint_down: bool,
    pub change_height: bool,
    pub jumping: bool,
    pub auto_jumping_in_water: bool,
    pub sneaking: bool,
    pub sneak_down: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub up_left: bool,
    pub up_right: bool,
    pub want_up: bool,
    pub want_down: bool,
    pub want_down_slow: bool,
    pub want_up_slow: bool,
    pub sprinting: bool,
    pub ascend_block: bool,
    pub descend_block: bool,
    pub sneak_toggle_down: bool,
    pub persist_sneak: bool,
    pub start_sprinting: bool,
    pub stop_sprinting: bool,
    pub start_sneaking: bool,
    pub stop_sneaking: bool,
    pub start_swimming: bool,
    pub stop_swimming: bool,
    pub start_jumping: bool,
    pub start_gliding: bool,
    pub stop_gliding: bool,
    pub perform_item_interaction: bool,
    pub perform_block_actions: Option<BlockActions>,
    pub perform_item_stack_request: bool,
    pub handled_teleport: bool,
    pub emoting: bool,
    pub missed_swing: bool,
    pub start_crawling: bool,
    pub stop_crawling: bool,
    pub start_flying: bool,
    pub stop_flying: bool,
    pub client_ack_server_data: bool,
    pub is_in_client_predicted_vehicle: Option<(Vec2<f32>, ActorUniqueID)>,
    pub paddling_left: bool,
    pub paddling_right: bool,
    pub block_breaking_delay_enabled: bool,
    pub input_num: bool,
}
