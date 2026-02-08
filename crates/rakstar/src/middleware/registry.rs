use super::traits::{EventMiddleware, EventResult};
use crate::{GameData, Player};
use std::sync::Arc;

pub struct MiddlewareRegistry<T: GameData> {
    middlewares: Vec<Box<dyn EventMiddleware<T>>>,
    sorted: bool,
}

impl<T: GameData> MiddlewareRegistry<T> {
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
            sorted: false,
        }
    }

    pub fn register<M: EventMiddleware<T> + 'static>(&mut self, middleware: M) {
        self.middlewares.push(Box::new(middleware));
        self.sorted = false;
    }

    fn ensure_sorted(&mut self) {
        if !self.sorted {
            self.middlewares.sort_by_key(|m| m.priority());
            self.sorted = true;
        }
    }

    pub fn dispatch_player_connect(&mut self, player: Player, data: Arc<T>) -> bool {
        self.ensure_sorted();
        for middleware in &mut self.middlewares {
            match middleware.on_player_connect(player, data.clone()) {
                EventResult::Continue => continue,
                EventResult::Stop | EventResult::Block => return false,
            }
        }
        true
    }

    pub fn dispatch_player_disconnect(
        &mut self,
        player: Player,
        reason: i32,
        data: Arc<T>,
    ) -> bool {
        for middleware in &mut self.middlewares {
            match middleware.on_player_disconnect(player, reason, data.clone()) {
                EventResult::Continue => continue,
                EventResult::Stop | EventResult::Block => return false,
            }
        }
        true
    }

    pub fn dispatch_player_spawn(&mut self, player: Player, data: Arc<T>) -> bool {
        for middleware in &mut self.middlewares {
            match middleware.on_player_spawn(player, data.clone()) {
                EventResult::Continue => continue,
                EventResult::Stop | EventResult::Block => return false,
            }
        }
        true
    }

    pub fn dispatch_player_text(&mut self, player: Player, text: String, data: Arc<T>) -> bool {
        for middleware in &mut self.middlewares {
            match middleware.on_player_text(player, text.clone(), data.clone()) {
                EventResult::Continue => continue,
                EventResult::Stop | EventResult::Block => return false,
            }
        }
        true
    }

    pub fn dispatch_player_command_text(
        &mut self,
        player: Player,
        command: String,
        data: Arc<T>,
    ) -> bool {
        for middleware in &mut self.middlewares {
            match middleware.on_player_command_text(player, command.clone(), data.clone()) {
                EventResult::Continue => continue,
                EventResult::Stop | EventResult::Block => return false,
            }
        }
        true
    }

    pub fn dispatch_dialog_response(
        &mut self,
        player: Player,
        dialog_id: i32,
        response: i32,
        list_item: i32,
        input_text: String,
        data: Arc<T>,
    ) -> bool {
        for middleware in &mut self.middlewares {
            match middleware.on_dialog_response(
                player,
                dialog_id,
                response,
                list_item,
                input_text.clone(),
                data.clone(),
            ) {
                EventResult::Continue => continue,
                EventResult::Stop | EventResult::Block => return false,
            }
        }
        true
    }
}

impl<T: GameData> Default for MiddlewareRegistry<T> {
    fn default() -> Self {
        Self::new()
    }
}
