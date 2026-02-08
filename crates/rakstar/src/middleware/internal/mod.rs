mod anti_flood;

pub use anti_flood::AntiFloodMiddleware;

use crate::{GameData, MiddlewareRegistry};

pub fn register_internal_middlewares<T: GameData>(registry: &mut MiddlewareRegistry<T>) {
    registry.register(AntiFloodMiddleware::<T>::new());
    println!("[RakStar] Registered internal middleware: AntiFloodMiddleware");
}
