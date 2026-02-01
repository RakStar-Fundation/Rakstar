use std::ffi::{c_void, CString};
use super::types::{ComponentVersion, CAPIStringView, PlayerPtr};

pub type ComponentCreateFn = unsafe extern "C" fn(
    uid: u64,
    name: *const i8,
    version: ComponentVersion,
    on_ready: *const c_void,
    on_reset: *const c_void,
    on_free: *const c_void,
) -> *mut c_void;

pub type PlayerGetNameFn = unsafe extern "C" fn(
    player: PlayerPtr,
    name: *mut CAPIStringView,
) -> i32;

pub type PlayerFromIdFn = unsafe extern "C" fn(playerid: i32) -> PlayerPtr;
pub type PlayerGetIdFn = unsafe extern "C" fn(player: PlayerPtr) -> i32;
pub type PlayerKickFn = unsafe extern "C" fn(player: PlayerPtr) -> bool;
pub type PlayerBanFn = unsafe extern "C" fn(player: PlayerPtr) -> bool;
pub type PlayerSpawnFn = unsafe extern "C" fn(player: PlayerPtr) -> bool;
pub type PlayerGetHealthFn = unsafe extern "C" fn(player: PlayerPtr, health: *mut f32);
pub type PlayerSetHealthFn = unsafe extern "C" fn(player: PlayerPtr, health: f32) -> bool;
pub type PlayerGetArmourFn = unsafe extern "C" fn(player: PlayerPtr, armour: *mut f32);
pub type PlayerSetArmourFn = unsafe extern "C" fn(player: PlayerPtr, armour: f32) -> bool;
pub type PlayerGetPosFn = unsafe extern "C" fn(player: PlayerPtr, x: *mut f32, y: *mut f32, z: *mut f32);
pub type PlayerSetPosFn = unsafe extern "C" fn(player: PlayerPtr, x: f32, y: f32, z: f32) -> bool;
pub type PlayerGetInteriorFn = unsafe extern "C" fn(player: PlayerPtr) -> i32;
pub type PlayerSetInteriorFn = unsafe extern "C" fn(player: PlayerPtr, interior: i32) -> bool;
pub type PlayerGetVirtualWorldFn = unsafe extern "C" fn(player: PlayerPtr) -> i32;
pub type PlayerSetVirtualWorldFn = unsafe extern "C" fn(player: PlayerPtr, world: i32) -> bool;
pub type PlayerResetWeaponsFn = unsafe extern "C" fn(player: PlayerPtr) -> bool;
pub type PlayerGiveWeaponFn = unsafe extern "C" fn(player: PlayerPtr, weapon: i32, ammo: i32) -> bool;

pub type EventCallback = unsafe extern "C" fn() -> bool;

pub type EventAddHandlerFn = unsafe extern "C" fn(
    name: *const i8,
    priority: i32,
    callback: EventCallback,
) -> bool;

#[repr(C)]
pub struct ComponentApi {
    pub create: Option<ComponentCreateFn>,
}

#[repr(C)]
pub struct PlayerApi {
    pub get_name: Option<PlayerGetNameFn>,
    pub from_id: Option<PlayerFromIdFn>,
    pub get_id: Option<PlayerGetIdFn>,
    pub kick: Option<PlayerKickFn>,
    pub ban: Option<PlayerBanFn>,
    pub spawn: Option<PlayerSpawnFn>,
    pub get_health: Option<PlayerGetHealthFn>,
    pub set_health: Option<PlayerSetHealthFn>,
    pub get_armour: Option<PlayerGetArmourFn>,
    pub set_armour: Option<PlayerSetArmourFn>,
    pub get_pos: Option<PlayerGetPosFn>,
    pub set_pos: Option<PlayerSetPosFn>,
    pub get_interior: Option<PlayerGetInteriorFn>,
    pub set_interior: Option<PlayerSetInteriorFn>,
    pub get_virtual_world: Option<PlayerGetVirtualWorldFn>,
    pub set_virtual_world: Option<PlayerSetVirtualWorldFn>,
    pub reset_weapons: Option<PlayerResetWeaponsFn>,
    pub give_weapon: Option<PlayerGiveWeaponFn>,
}

#[repr(C)]
pub struct EventApi {
    pub add_handler: Option<EventAddHandlerFn>,
}

#[repr(C)]
pub struct OmpApi {
    pub actor: *const c_void,
    pub checkpoint: *const c_void,
    pub race_checkpoint: *const c_void,
    pub class: *const c_void,
    pub player: PlayerApi,
    pub component: ComponentApi,
    pub config: *const c_void,
    pub core: *const c_void,
    pub npc: *const c_void,
    pub custom_model: *const c_void,
    pub dialog: *const c_void,
    pub event: EventApi,
    pub gang_zone: *const c_void,
    pub menu: *const c_void,
    pub object: *const c_void,
    pub player_object: *const c_void,
    pub pickup: *const c_void,
    pub all: *const c_void,
    pub recording: *const c_void,
    pub text_draw: *const c_void,
    pub player_text_draw: *const c_void,
    pub text_label: *const c_void,
    pub player_text_label: *const c_void,
    pub vehicle: *const c_void,
}

unsafe impl Send for OmpApi {}
unsafe impl Sync for OmpApi {}

macro_rules! load_fn {
    ($lib:expr, $api:expr, $field:expr, $name:expr) => {
        let fn_name = CString::new($name).unwrap();
        let fn_ptr = libc::dlsym($lib, fn_name.as_ptr());
        if !fn_ptr.is_null() {
            $field = Some(std::mem::transmute(fn_ptr));
        }
    };
}

#[cfg(unix)]
pub unsafe fn initialize_capi(api: *mut OmpApi) -> bool {
    let lib_path = CString::new("./components/$CAPI.so").unwrap();
    let lib = libc::dlopen(lib_path.as_ptr(), libc::RTLD_LAZY | libc::RTLD_LOCAL);
    
    if lib.is_null() {
        return false;
    }

    let create_fn_name = CString::new("Component_Create").unwrap();
    let create_fn_ptr = libc::dlsym(lib, create_fn_name.as_ptr());
    
    if create_fn_ptr.is_null() {
        libc::dlclose(lib);
        return false;
    }

    (*api).component.create = Some(std::mem::transmute(create_fn_ptr));
    
    load_fn!(lib, api, (*api).player.get_name, "Player_GetName");
    load_fn!(lib, api, (*api).player.from_id, "Player_FromID");
    load_fn!(lib, api, (*api).player.get_id, "Player_GetID");
    load_fn!(lib, api, (*api).player.kick, "Player_Kick");
    load_fn!(lib, api, (*api).player.ban, "Player_Ban");
    load_fn!(lib, api, (*api).player.spawn, "Player_Spawn");
    load_fn!(lib, api, (*api).player.get_health, "Player_GetHealth");
    load_fn!(lib, api, (*api).player.set_health, "Player_SetHealth");
    load_fn!(lib, api, (*api).player.get_armour, "Player_GetArmour");
    load_fn!(lib, api, (*api).player.set_armour, "Player_SetArmour");
    load_fn!(lib, api, (*api).player.get_pos, "Player_GetPos");
    load_fn!(lib, api, (*api).player.set_pos, "Player_SetPos");
    load_fn!(lib, api, (*api).player.get_interior, "Player_GetInterior");
    load_fn!(lib, api, (*api).player.set_interior, "Player_SetInterior");
    load_fn!(lib, api, (*api).player.get_virtual_world, "Player_GetVirtualWorld");
    load_fn!(lib, api, (*api).player.set_virtual_world, "Player_SetVirtualWorld");
    load_fn!(lib, api, (*api).player.reset_weapons, "Player_ResetWeapons");
    load_fn!(lib, api, (*api).player.give_weapon, "Player_GiveWeapon");
    load_fn!(lib, api, (*api).event.add_handler, "Event_AddHandler");
    
    true
}

#[cfg(windows)]
pub unsafe fn initialize_capi(api: *mut OmpApi) -> bool {
    false
}

