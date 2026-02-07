mod anti_flood;

pub use anti_flood::AntiFloodMiddleware;

use crate::MiddlewareRegistry;

pub fn register_internal_middlewares(registry: &mut MiddlewareRegistry) {
    registry.register(AntiFloodMiddleware::new());
    println!("[RakStar] Registered internal middleware: AntiFloodMiddleware");
}
