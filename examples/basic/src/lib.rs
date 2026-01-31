use rakstar::{Component, EventHandler, Player};

#[derive(Default)]
struct MyComponent;

impl Component for MyComponent {
    fn on_ready(&mut self) {
        println!("Component ready!");
    }

    fn on_reset(&mut self) {
        println!("Component reset!");
    }

    fn on_free(&mut self) {
        println!("Component freed!");
    }
}

impl EventHandler for MyComponent {
    fn on_player_connect(&mut self, player_id: i32) {
        println!("Player {} connected!", Player::from_id(player_id).unwrap().get_name().unwrap());
    }
}

rakstar::entrypoint!(MyComponent, "RakStar Gamemode", (1, 0, 0, 0));

