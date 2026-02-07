use crate::{
    Feature, FeatureEvents, FeaturePriority, GameData, Player,
    dialog::{self, notify},
};
use std::sync::{Arc, Mutex};

pub struct DialogFeature {
    enabled: bool,
}

impl DialogFeature {
    pub fn new() -> Self {
        Self { enabled: true }
    }
}

impl Feature for DialogFeature {
    fn name(&self) -> &'static str {
        "RakStar::Dialog"
    }

    fn priority(&self) -> FeaturePriority {
        FeaturePriority::Critical
    }

    fn on_ready(&mut self, _data: &Arc<Mutex<dyn GameData>>) {
        println!("[DialogFeature] Command system ready");
    }
}

impl FeatureEvents for DialogFeature {
    fn on_dialog_response(
        &mut self,
        player: crate::Player,
        dialog_id: i32,
        response: i32,
        list_item: i32,
        input_text: String,
        _data: &Arc<Mutex<dyn crate::GameData>>,
    ) {
        println!(
            "[DialogFeature] Player {} response: {} {} {} {}",
            player.get_name(),
            dialog_id,
            response,
            list_item,
            input_text
        );

        dialog::handler::notify_dialog(
            dialog_id as u32,
            player.get_id() as u32,
            response as u8,
            list_item as i8,
            input_text,
        );
    }
}

impl Default for DialogFeature {
    fn default() -> Self {
        Self::new()
    }
}
