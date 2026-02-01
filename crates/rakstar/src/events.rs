pub mod internal;

pub use internal::{
    PRIORITY_HIGHEST, PRIORITY_FAIRLY_HIGH, PRIORITY_DEFAULT, 
    PRIORITY_FAIRLY_LOW, PRIORITY_LOWEST,
    PRIORITY_INTERNAL, PRIORITY_USER, PRIORITY_LOW
};

pub trait EventHandler {
    fn on_player_connect(&mut self, _player_id: i32) {}
    fn on_player_disconnect(&mut self, _player_id: i32, _reason: i32) {}
    fn on_player_spawn(&mut self, _player_id: i32) {}
    fn on_player_death(&mut self, _player_id: i32, _killer_id: i32, _weapon: i32) {}
    fn on_player_text(&mut self, _player_id: i32, _text: &str) -> bool { true }
    fn on_player_command_text(&mut self, _player_id: i32, _command: &str) -> bool { true }
    fn on_player_request_class(&mut self, _player_id: i32, _class_id: i32) -> bool { true }
    fn on_player_request_spawn(&mut self, _player_id: i32) -> bool { true }
    fn on_player_key_state_change(&mut self, _player_id: i32, _new_keys: i32, _old_keys: i32) {}
    fn on_player_stream_in(&mut self, _player_id: i32, _for_player_id: i32) {}
    fn on_player_stream_out(&mut self, _player_id: i32, _for_player_id: i32) {}
    fn on_player_take_damage(&mut self, _player_id: i32, _issuer_id: i32, _amount: f32, _weapon: i32, _bodypart: i32) {}
    fn on_player_give_damage(&mut self, _player_id: i32, _damaged_id: i32, _amount: f32, _weapon: i32, _bodypart: i32) {}
    fn on_player_enter_vehicle(&mut self, _player_id: i32, _vehicle_id: i32, _is_passenger: bool) {}
    fn on_player_exit_vehicle(&mut self, _player_id: i32, _vehicle_id: i32) {}
}
