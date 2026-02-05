/// Feature system for modular gamemode logic
use std::sync::{Arc, Mutex};

/// Base trait for all features
pub trait Feature: Send + Sync {
    /// Unique name for this feature
    fn name(&self) -> &'static str;

    /// Execution priority (lower value = higher priority)
    fn priority(&self) -> FeaturePriority {
        FeaturePriority::Normal
    }

    /// Called when the game data is ready
    fn on_ready(&mut self, _data: &Arc<Mutex<dyn crate::GameData>>) {}

    /// Called when the server is reset
    fn on_reset(&mut self, _data: &Arc<Mutex<dyn crate::GameData>>) {}

    /// Called when the game data is being destroyed
    fn on_free(&mut self, _data: &Arc<Mutex<dyn crate::GameData>>) {}
}

/// Priority levels for feature execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FeaturePriority {
    Critical = 0, // RakStar internal systems
    High = 100,   // Auth, critical user systems
    Normal = 500, // Regular user features
    Low = 1000,   // Logging, analytics
}

/// Trait for features that handle events
pub trait FeatureEvents: Feature {
    fn on_player_connect(
        &mut self,
        _player: crate::Player,
        _data: &Arc<Mutex<dyn crate::GameData>>,
    ) {
    }

    fn on_player_disconnect(
        &mut self,
        _player: crate::Player,
        _reason: i32,
        _data: &Arc<Mutex<dyn crate::GameData>>,
    ) {
    }

    fn on_player_spawn(&mut self, _player: crate::Player, _data: &Arc<Mutex<dyn crate::GameData>>) {
    }

    fn on_player_text(
        &mut self,
        _player: crate::Player,
        _text: String,
        _data: &Arc<Mutex<dyn crate::GameData>>,
    ) {
    }

    fn on_player_command_text(
        &mut self,
        _player: crate::Player,
        _command: String,
        _data: &Arc<Mutex<dyn crate::GameData>>,
    ) {
    }

    fn on_dialog_response(
        &mut self,
        _player: crate::Player,
        _dialog_id: i32,
        _response: i32,
        _list_item: i32,
        _input_text: String,
        _data: &Arc<Mutex<dyn crate::GameData>>,
    ) {
    }
}
