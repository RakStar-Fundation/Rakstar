use std::sync::Arc;

use crate::GameData;

pub trait Middleware<T: GameData>: Send + Sync {
    fn name(&self) -> &'static str;

    fn priority(&self) -> MiddlewarePriority {
        MiddlewarePriority::Normal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MiddlewarePriority {
    Critical = 0,
    High = 100,
    Normal = 500,
    Low = 1000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventResult {
    Continue,
    Stop,
    Block,
}

pub trait EventMiddleware<T: GameData>: Middleware<T> {
    fn on_player_connect(&mut self, _player: crate::Player, _data: Arc<T>) -> EventResult {
        EventResult::Continue
    }

    fn on_player_disconnect(
        &mut self,
        _player: crate::Player,
        _reason: i32,
        _data: Arc<T>,
    ) -> EventResult {
        EventResult::Continue
    }

    fn on_player_spawn(&mut self, _player: crate::Player, _data: Arc<T>) -> EventResult {
        EventResult::Continue
    }

    fn on_player_text(
        &mut self,
        _player: crate::Player,
        _text: String,
        _data: Arc<T>,
    ) -> EventResult {
        EventResult::Continue
    }

    fn on_player_command_text(
        &mut self,
        _player: crate::Player,
        _command: String,
        _data: Arc<T>,
    ) -> EventResult {
        EventResult::Continue
    }

    fn on_dialog_response(
        &mut self,
        _player: crate::Player,
        _dialog_id: i32,
        _response: i32,
        _list_item: i32,
        _input_text: String,
        _data: Arc<T>,
    ) -> EventResult {
        EventResult::Continue
    }
}
