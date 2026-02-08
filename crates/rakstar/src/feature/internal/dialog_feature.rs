use crate::{Feature, FeatureEvents, FeaturePriority, GameData, Player, dialog};
use std::sync::Arc;

pub struct DialogFeature<T: GameData> {
    enabled: bool,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: GameData> DialogFeature<T> {
    pub fn new() -> Self {
        Self {
            enabled: true,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: GameData> Feature<T> for DialogFeature<T> {
    fn name(&self) -> &'static str {
        "RakStar::Dialog"
    }

    fn priority(&self) -> FeaturePriority {
        FeaturePriority::Critical
    }

    fn on_ready(&mut self, _data: Arc<T>) {
        println!("[DialogFeature] Command system ready");
    }
}

impl<T: GameData> FeatureEvents<T> for DialogFeature<T> {
    fn on_dialog_response(
        &mut self,
        player: crate::Player,
        dialog_id: i32,
        response: i32,
        list_item: i32,
        input_text: String,
        _data: Arc<T>,
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

impl<T: GameData> Default for DialogFeature<T> {
    fn default() -> Self {
        Self::new()
    }
}
