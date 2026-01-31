use rakstar::Component;

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

rakstar::entrypoint!(MyComponent, "RakStar Gamemode", (1, 0, 0, 0));

