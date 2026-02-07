use std::sync::{Arc, Mutex};

pub trait Feature: Send + Sync {
    fn name(&self) -> &'static str;

    fn priority(&self) -> FeaturePriority {
        FeaturePriority::Normal
    }

    fn on_ready(&mut self, _data: &Arc<Mutex<dyn crate::GameData>>) {}

    fn on_reset(&mut self, _data: &Arc<Mutex<dyn crate::GameData>>) {}

    fn on_free(&mut self, _data: &Arc<Mutex<dyn crate::GameData>>) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FeaturePriority {
    Critical = 0,
    High = 100,
    Normal = 500,
    Low = 1000,
}

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
