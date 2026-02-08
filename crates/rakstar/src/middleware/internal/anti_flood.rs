use crate::{EventMiddleware, EventResult, GameData, Middleware, MiddlewarePriority, Player};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct AntiFloodMiddleware<T: GameData> {
    last_messages: Arc<Mutex<HashMap<i32, Instant>>>,
    min_interval: Duration,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: GameData> AntiFloodMiddleware<T> {
    pub fn new() -> Self {
        Self {
            last_messages: Arc::new(Mutex::new(HashMap::new())),
            min_interval: Duration::from_millis(500),
            _phantom: std::marker::PhantomData,
        }
    }

    fn check_flood(&self, player_id: i32) -> bool {
        let mut last_messages = self.last_messages.lock().unwrap();
        let now = Instant::now();

        if let Some(&last_time) = last_messages.get(&player_id) {
            if now.duration_since(last_time) < self.min_interval {
                return true;
            }
        }

        last_messages.insert(player_id, now);
        false
    }
}

impl<T: GameData> Middleware<T> for AntiFloodMiddleware<T> {
    fn name(&self) -> &'static str {
        "RakStar::AntiFlood"
    }

    fn priority(&self) -> MiddlewarePriority {
        MiddlewarePriority::Critical
    }
}

impl<T: GameData> EventMiddleware<T> for AntiFloodMiddleware<T> {
    fn on_player_text(&mut self, player: Player, _text: String, _data: Arc<T>) -> EventResult {
        if self.check_flood(player.get_id()) {
            println!("[AntiFlood] Blocked flood from {}", player.get_name());
            return EventResult::Block;
        }
        EventResult::Continue
    }

    fn on_player_command_text(
        &mut self,
        player: Player,
        _command: String,
        _data: Arc<T>,
    ) -> EventResult {
        if self.check_flood(player.get_id()) {
            println!(
                "[AntiFlood] Blocked command flood from {}",
                player.get_name()
            );
            return EventResult::Block;
        }
        EventResult::Continue
    }
}

impl<T: GameData> Default for AntiFloodMiddleware<T> {
    fn default() -> Self {
        Self::new()
    }
}
