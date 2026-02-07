use super::traits::{EventMiddleware, EventResult};
use crate::{GameData, Player};
use std::sync::{Arc, Mutex};

pub struct MiddlewareRegistry {
    middlewares: Vec<Box<dyn EventMiddleware>>,
    sorted: bool,
}

impl MiddlewareRegistry {
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
            sorted: false,
        }
    }

    pub fn register<M: EventMiddleware + 'static>(&mut self, middleware: M) {
        self.middlewares.push(Box::new(middleware));
        self.sorted = false;
    }

    fn ensure_sorted(&mut self) {
        if !self.sorted {
            self.middlewares.sort_by_key(|m| m.priority());
            self.sorted = true;
        }
    }

    pub fn dispatch_player_connect(
        &mut self,
        player: Player,
        data: &Arc<Mutex<dyn GameData>>,
    ) -> bool {
        self.ensure_sorted();
        for middleware in &mut self.middlewares {
            match middleware.on_player_connect(player, data) {
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
        data: &Arc<Mutex<dyn GameData>>,
    ) -> bool {
        for middleware in &mut self.middlewares {
            match middleware.on_player_disconnect(player, reason, data) {
                EventResult::Continue => continue,
                EventResult::Stop | EventResult::Block => return false,
            }
        }
        true
    }

    pub fn dispatch_player_spawn(
        &mut self,
        player: Player,
        data: &Arc<Mutex<dyn GameData>>,
    ) -> bool {
        for middleware in &mut self.middlewares {
            match middleware.on_player_spawn(player, data) {
                EventResult::Continue => continue,
                EventResult::Stop | EventResult::Block => return false,
            }
        }
        true
    }

    pub fn dispatch_player_text(
        &mut self,
        player: Player,
        text: &mut String,
        data: &Arc<Mutex<dyn GameData>>,
    ) -> bool {
        println!("dispatch player text");
        for middleware in &mut self.middlewares {
            match middleware.on_player_text(player, text, data) {
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
        data: &Arc<Mutex<dyn GameData>>,
    ) -> bool {
        for middleware in &mut self.middlewares {
            match middleware.on_player_command_text(player, command.clone(), data) {
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
        data: &Arc<Mutex<dyn GameData>>,
    ) -> bool {
        for middleware in &mut self.middlewares {
            match middleware.on_dialog_response(
                player,
                dialog_id,
                response,
                list_item,
                input_text.clone(),
                data,
            ) {
                EventResult::Continue => continue,
                EventResult::Stop | EventResult::Block => return false,
            }
        }
        true
    }
}

impl Default for MiddlewareRegistry {
    fn default() -> Self {
        Self::new()
    }
}
