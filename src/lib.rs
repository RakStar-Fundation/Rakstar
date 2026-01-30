use omp::{events::Events, main, register, types::colour::Colour};

struct MyGM;

impl Events for MyGM {
    fn on_player_connect(&mut self, player: omp::players::Player) {
        player.send_client_message(Colour::from_rgba(0xFFFFFFFF), "Welcome to my server!");
    }
}

#[main]
pub fn game_main() {
    register!(MyGM);
}
