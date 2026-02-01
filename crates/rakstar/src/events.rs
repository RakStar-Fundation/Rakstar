pub mod internal;

pub use internal::{
    PRIORITY_DEFAULT, PRIORITY_FAIRLY_HIGH, PRIORITY_FAIRLY_LOW, PRIORITY_HIGHEST,
    PRIORITY_INTERNAL, PRIORITY_LOW, PRIORITY_LOWEST, PRIORITY_USER,
};

use crate::{Player, Vehicle};

pub trait EventHandler {
    fn on_player_connect(&mut self, _player: Player) {}
    fn on_player_disconnect(&mut self, _player: Player, _reason: i32) {}
    fn on_player_spawn(&mut self, _player: Player) {}
    fn on_player_death(&mut self, _player: Player, _killer: Player, _weapon: i32) {}
    fn on_player_text(&mut self, _player: Player, _text: String) -> bool {
        true
    }
    fn on_player_command_text(&mut self, _player: Player, _command: String) -> bool {
        true
    }
    fn on_player_request_class(&mut self, _player: Player, _class_id: i32) -> bool {
        true
    }
    fn on_player_request_spawn(&mut self, _player: Player) -> bool {
        true
    }
    fn on_player_key_state_change(&mut self, _player: Player, _new_keys: i32, _old_keys: i32) {}
    fn on_player_stream_in(&mut self, _player: Player, _for_player: Player) {}
    fn on_player_stream_out(&mut self, _player: Player, _for_player: Player) {}
    fn on_player_take_damage(
        &mut self,
        _player: Player,
        _issuer: Player,
        _amount: f32,
        _weapon: i32,
        _bodypart: i32,
    ) {
    }
    fn on_player_give_damage(
        &mut self,
        _player: Player,
        _damaged: Player,
        _amount: f32,
        _weapon: i32,
        _bodypart: i32,
    ) {
    }
    fn on_player_enter_vehicle(&mut self, _player: Player, _vehicle: Vehicle, _is_passenger: bool) {
    }
    fn on_player_exit_vehicle(&mut self, _player: Player, _vehicle: Vehicle) {}
}
