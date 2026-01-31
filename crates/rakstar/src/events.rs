pub mod internal;

pub use internal::{
    PRIORITY_HIGHEST, PRIORITY_FAIRLY_HIGH, PRIORITY_DEFAULT, 
    PRIORITY_FAIRLY_LOW, PRIORITY_LOWEST,
    PRIORITY_INTERNAL, PRIORITY_USER, PRIORITY_LOW
};

pub trait EventHandler {
    fn on_player_connect(&mut self, _player_id: i32) {}
}
