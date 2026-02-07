use crate::{Feature, FeatureEvents, FeaturePriority, GameData, Player};
use std::sync::{Arc, Mutex};

pub struct CommandFeature {
    enabled: bool,
}

impl CommandFeature {
    pub fn new() -> Self {
        Self { enabled: true }
    }
}

impl Feature for CommandFeature {
    fn name(&self) -> &'static str {
        "RakStar::Commands"
    }

    fn priority(&self) -> FeaturePriority {
        FeaturePriority::Critical
    }

    fn on_ready(&mut self, _data: &Arc<Mutex<dyn GameData>>) {
        println!("[CommandFeature] Command system ready");
    }
}

impl FeatureEvents for CommandFeature {
    fn on_player_command_text(
        &mut self,
        player: Player,
        command: String,
        data: &Arc<Mutex<dyn GameData>>,
    ) {
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

impl Default for CommandFeature {
    fn default() -> Self {
        Self::new()
    }
}
