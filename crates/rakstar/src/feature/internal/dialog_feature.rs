use crate::{Feature, FeatureEvents, FeaturePriority, GameData, Player};
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
        _player: crate::Player,
        _dialog_id: i32,
        _response: i32,
        _list_item: i32,
        _input_text: String,
        _data: &Arc<Mutex<dyn crate::GameData>>,
    ) {
          println!(
            "[DialogFeature] Player {} response: {} {} {} {}",
                       _player.get_name(), 
                      _dialog_id, 
                      _response, 
                       _list_item, 
                       _input_text

        );
    }
}

impl Default for DialogFeature {
    fn default() -> Self {
        Self::new()
    }
}
