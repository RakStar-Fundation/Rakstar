use crate::{Feature, FeatureEvents, FeaturePriority, GameData, Player};
use std::sync::Arc;

pub struct CommandFeature<T: GameData> {
    enabled: bool,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: GameData> CommandFeature<T> {
    pub fn new() -> Self {
        Self {
            enabled: true,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: GameData> Feature<T> for CommandFeature<T> {
    fn name(&self) -> &'static str {
        "RakStar::Commands"
    }

    fn priority(&self) -> FeaturePriority {
        FeaturePriority::Critical
    }

    fn on_ready(&mut self, _data: Arc<T>) {
        println!("[CommandFeature] Command system ready");
    }
}

impl<T: GameData> FeatureEvents<T> for CommandFeature<T> {
    fn on_player_command_text(&mut self, player: Player, command: String, _data: Arc<T>) {
        if !self.enabled {
            return;
        }

        println!(
            "[CommandFeature] Player {} executed: {}",
            player.get_name(),
            command
        );
    }
}

impl<T: GameData> Default for CommandFeature<T> {
    fn default() -> Self {
        Self::new()
    }
}
