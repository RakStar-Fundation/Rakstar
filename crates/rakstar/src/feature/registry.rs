use super::traits::FeatureEvents;
use crate::{GameData, Player};
use std::sync::Arc;

pub struct FeatureRegistry<T: GameData> {
    features: Vec<Box<dyn FeatureEvents<T>>>,
    sorted: bool,
}

impl<T: GameData> FeatureRegistry<T> {
    pub fn new() -> Self {
        Self {
            features: Vec::new(),
            sorted: false,
        }
    }

    pub fn register<F: FeatureEvents<T> + 'static>(&mut self, feature: F) {
        self.features.push(Box::new(feature));
        self.sorted = false;
    }

    fn ensure_sorted(&mut self) {
        if !self.sorted {
            self.features.sort_by_key(|f| f.priority());
            self.sorted = true;
        }
    }

    pub fn on_ready(&mut self, data: Arc<T>) {
        self.ensure_sorted();
        for feature in &mut self.features {
            feature.on_ready(data.clone());
        }
    }

    pub fn on_reset(&mut self, data: Arc<T>) {
        for feature in &mut self.features {
            feature.on_reset(data.clone());
        }
    }

    pub fn on_free(&mut self, data: Arc<T>) {
        for feature in &mut self.features {
            feature.on_free(data.clone());
        }
    }

    pub fn dispatch_player_connect(&mut self, player: Player, data: Arc<T>) {
        self.ensure_sorted();
        for feature in &mut self.features {
            feature.on_player_connect(player, data.clone());
        }
    }

    pub fn dispatch_player_disconnect(&mut self, player: Player, reason: i32, data: Arc<T>) {
        for feature in &mut self.features {
            feature.on_player_disconnect(player, reason, data.clone());
        }
    }

    pub fn dispatch_player_spawn(&mut self, player: Player, data: Arc<T>) {
        for feature in &mut self.features {
            feature.on_player_spawn(player, data.clone());
        }
    }

    pub fn dispatch_player_text(&mut self, player: Player, text: String, data: Arc<T>) {
        for feature in &mut self.features {
            feature.on_player_text(player, text.clone(), data.clone());
        }
    }

    pub fn dispatch_player_command_text(&mut self, player: Player, command: String, data: Arc<T>) {
        println!(
            "[FeatureRegistry] Dispatching command '{}' to {} features",
            command,
            self.features.len()
        );
        for feature in &mut self.features {
            println!("[FeatureRegistry] Calling feature: {}", feature.name());
            feature.on_player_command_text(player, command.clone(), data.clone());
        }
    }

    pub fn dispatch_dialog_response(
        &mut self,
        player: Player,
        dialog_id: i32,
        response: i32,
        list_item: i32,
        input_text: String,
        data: Arc<T>,
    ) {
        for feature in &mut self.features {
            feature.on_dialog_response(
                player,
                dialog_id,
                response,
                list_item,
                input_text.clone(),
                data.clone(),
            );
        }
    }
}

impl<T: GameData> Default for FeatureRegistry<T> {
    fn default() -> Self {
        Self::new()
    }
}
