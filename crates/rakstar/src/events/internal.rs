pub const PRIORITY_HIGHEST: i32 = 0;
pub const PRIORITY_FAIRLY_HIGH: i32 = 1;
pub const PRIORITY_DEFAULT: i32 = 2;
pub const PRIORITY_FAIRLY_LOW: i32 = 3;
pub const PRIORITY_LOWEST: i32 = 4;

pub const PRIORITY_INTERNAL: i32 = PRIORITY_FAIRLY_HIGH;
pub const PRIORITY_USER: i32 = PRIORITY_DEFAULT;
pub const PRIORITY_LOW: i32 = PRIORITY_FAIRLY_LOW;

pub unsafe fn register_internal_handlers() {
    let api = match crate::macros::get_api() {
        Some(api) => api,
        None => return,
    };
    
    let _add_handler = match api.event.add_handler {
        Some(handler) => handler,
        None => return,
    };
}
