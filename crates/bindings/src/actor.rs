#![allow(non_snake_case, non_camel_case_types)]
use crate::types::*;

pub type Actor_Create_t =
    unsafe extern "C" fn(model: i32, x: f32, y: f32, z: f32, rot: f32, id: *mut i32) -> ActorPtr;

pub type Actor_Destroy_t = unsafe extern "C" fn(actor: ActorPtr) -> bool;
pub type Actor_FromID_t = unsafe extern "C" fn(actorid: i32) -> ActorPtr;
pub type Actor_GetID_t = unsafe extern "C" fn(actor: ActorPtr) -> i32;

pub type Actor_SetPos_t = unsafe extern "C" fn(actor: ActorPtr, x: f32, y: f32, z: f32) -> bool;

pub type Actor_GetPos_t =
    unsafe extern "C" fn(actor: ActorPtr, x: *mut f32, y: *mut f32, z: *mut f32) -> bool;

pub type Actor_SetHealth_t = unsafe extern "C" fn(actor: ActorPtr, hp: f32) -> bool;
pub type Actor_GetHealth_t = unsafe extern "C" fn(actor: ActorPtr) -> f32;

pub type Actor_SetInvulnerable_t = unsafe extern "C" fn(actor: ActorPtr, toggle: bool) -> bool;
pub type Actor_IsInvulnerable_t = unsafe extern "C" fn(actor: ActorPtr) -> bool;

pub type Actor_SetSkin_t = unsafe extern "C" fn(actor: ActorPtr, skin: i32) -> bool;
pub type Actor_GetSkin_t = unsafe extern "C" fn(actor: ActorPtr) -> i32;

pub type Actor_SetVirtualWorld_t = unsafe extern "C" fn(actor: ActorPtr, vw: i32) -> bool;
pub type Actor_GetVirtualWorld_t = unsafe extern "C" fn(actor: ActorPtr) -> i32;

pub type Actor_ApplyAnimation_t = unsafe extern "C" fn(
    actor: ActorPtr,
    name: *const u8,
    library: *const u8,
    delta: f32,
    loop_: bool,
    lockX: bool,
    lockY: bool,
    freeze: bool,
    time: i32,
) -> bool;

pub type Actor_ClearAnimations_t = unsafe extern "C" fn(actor: ActorPtr) -> bool;

pub struct ActorAPI {
    pub create: Option<Actor_Create_t>,
    pub destroy: Option<Actor_Destroy_t>,
    pub from_id: Option<Actor_FromID_t>,
    pub get_id: Option<Actor_GetID_t>,
    pub set_pos: Option<Actor_SetPos_t>,
    pub get_pos: Option<Actor_GetPos_t>,
    pub set_health: Option<Actor_SetHealth_t>,
    pub get_health: Option<Actor_GetHealth_t>,
    pub set_invulnerable: Option<Actor_SetInvulnerable_t>,
    pub is_invulnerable: Option<Actor_IsInvulnerable_t>,
    pub set_skin: Option<Actor_SetSkin_t>,
    pub get_skin: Option<Actor_GetSkin_t>,
    pub set_virtual_world: Option<Actor_SetVirtualWorld_t>,
    pub get_virtual_world: Option<Actor_GetVirtualWorld_t>,
    pub apply_animation: Option<Actor_ApplyAnimation_t>,
    pub clear_animations: Option<Actor_ClearAnimations_t>,
}

impl Default for ActorAPI {
    fn default() -> Self {
        Self {
            create: None,
            destroy: None,
            from_id: None,
            get_id: None,
            set_pos: None,
            get_pos: None,
            set_health: None,
            get_health: None,
            set_invulnerable: None,
            is_invulnerable: None,
            set_skin: None,
            get_skin: None,
            set_virtual_world: None,
            get_virtual_world: None,
            apply_animation: None,
            clear_animations: None,
        }
    }
}
