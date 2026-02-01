use rakstar::{Component, EventHandler, Player, Vehicle};

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
    fn on_player_connect(&mut self, player: Player) {
        println!("Player {} connected!", player.get_name().unwrap());
    }

    fn on_player_command_text(&mut self, player: Player, command: String) -> bool {
        if command.starts_with("/veh") {
            let (x, y, z) = player.get_pos();
            let rotation = player.get_facing_angle();

            let Some(vehicle) = Vehicle::create(415, x, y, z, rotation, -1, -1, -1, false) else {
                return false;
            };

            player.put_in_vehicle(&vehicle, 0);
            return true;
        }

        true
    }
}

rakstar::entrypoint!(MyComponent, "RakStar Gamemode", (1, 0, 0, 0));
