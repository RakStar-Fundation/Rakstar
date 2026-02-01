use crate::types::PlayerPtr;
use std::ffi::c_char;

pub type DialogShowT = unsafe extern "C" fn(
    player: PlayerPtr,
    dialog_id: i32,
    style: i32,
    title: *const c_char,
    body: *const c_char,
    button1: *const c_char,
    button2: *const c_char,
) -> bool;

pub type DialogHideT = unsafe extern "C" fn(player: PlayerPtr) -> bool;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DialogAPI {
    pub show: Option<DialogShowT>,
    pub hide: Option<DialogHideT>,
}
