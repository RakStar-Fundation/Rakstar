use rakstar::*;
use std::sync::{Arc, RwLock};

pub struct Basic {
    pub server_name: String,
    pub authenticated_players: RwLock<Vec<i32>>,
}

impl GameData for Basic {
    fn new() -> Self {
        Self {
            server_name: String::from("Basic Gamemode"),
            authenticated_players: RwLock::new(Vec::new()),
        }
    }

    fn on_ready(&self) {
        println!("[GameData] {} ready!", self.server_name);
    }
}

pub struct AuthFeature;

impl Feature<Basic> for AuthFeature {
    fn name(&self) -> &'static str {
        "AuthFeature"
    }

    fn priority(&self) -> FeaturePriority {
        FeaturePriority::High
    }
}

impl FeatureEvents<Basic> for AuthFeature {
    fn on_player_connect(&mut self, player: Player, data: Arc<Basic>) {
        println!(
            "[Auth] {} connected to {} - showing login dialog",
            player.get_name(),
            data.server_name
        );
    }

    fn on_player_command_text(&mut self, player: Player, command: String, data: Arc<Basic>) {
        if command.starts_with("/login") {
            let mut auth_players = data.authenticated_players.write().unwrap();

            if auth_players.contains(&player.get_id()) {
                player.send_client_message(0xffffffff, "You're already logged in.");
                return;
            }

            auth_players.push(player.get_id());
            player.send_client_message(0x00ff00ff, "Logged in successfully!");
        }
    }
}

pub struct AuthenticatedMiddleware;

impl Middleware<Basic> for AuthenticatedMiddleware {
    fn name(&self) -> &'static str {
        "AuthenticatedMiddleware"
    }

    fn priority(&self) -> MiddlewarePriority {
        MiddlewarePriority::High
    }
}

impl EventMiddleware<Basic> for AuthenticatedMiddleware {
    fn on_player_connect(&mut self, player: Player, data: Arc<Basic>) -> EventResult {
        println!(
            "[Auth] {} connected to {} - showing login dialog",
            player.get_name(),
            data.server_name
        );

        EventResult::Continue
    }

    fn on_player_command_text(
        &mut self,
        player: rakstar::Player,
        command: String,
        data: Arc<Basic>,
    ) -> EventResult {
        player.send_client_message(0xFFFFFFFF, "authenticated middleware");

        if command.starts_with("/login")
            || data
                .authenticated_players
                .read()
                .map(|v| v.contains(&player.get_id()))
                .unwrap_or(false)
        {
            return EventResult::Continue;
        }

        player.send_client_message(
            0xFFFFFFFF,
            "You can't use commands without being logged in.",
        );

        EventResult::Block
    }

    fn on_player_text(
        &mut self,
        player: rakstar::Player,
        _text: String,
        _data: Arc<Basic>,
    ) -> EventResult {
        player.send_client_message(0xFFFFFFFF, "You can't talk without being logged in.");

        EventResult::Block
    }
}

rakstar::entrypoint_new! {
    data: Basic,
    name: "Basic Gamemode",
    version: (1, 0, 0, 0),
    features: [
        AuthFeature,
    ],
    middlewares: [
        AuthenticatedMiddleware,
    ]
}
