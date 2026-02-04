use crate::types::{NPCPtr, PlayerPtr};

pub type NPC_Connect_t = unsafe extern "C" fn(name: *const i8, script: *const i8) -> bool;
pub type NPC_Create_t = unsafe extern "C" fn(name: *const i8, id: *mut i32) -> NPCPtr;
pub type NPC_Destroy_t = unsafe extern "C" fn(npc: NPCPtr) -> bool;
pub type NPC_FromID_t = unsafe extern "C" fn(npcid: i32) -> NPCPtr;
pub type NPC_GetID_t = unsafe extern "C" fn(npc: NPCPtr) -> i32;
pub type NPC_IsValid_t = unsafe extern "C" fn(npc: NPCPtr) -> bool;
pub type NPC_GetPlayer_t = unsafe extern "C" fn(npc: NPCPtr) -> PlayerPtr;

pub type NPC_Spawn_t = unsafe extern "C" fn(npc: NPCPtr) -> bool;
pub type NPC_Respawn_t = unsafe extern "C" fn(npc: NPCPtr) -> bool;

pub type NPC_SetPos_t = unsafe extern "C" fn(npc: NPCPtr, x: f32, y: f32, z: f32) -> bool;
pub type NPC_GetPos_t =
    unsafe extern "C" fn(npc: NPCPtr, x: *mut f32, y: *mut f32, z: *mut f32) -> bool;
pub type NPC_SetRot_t = unsafe extern "C" fn(npc: NPCPtr, rx: f32, ry: f32, rz: f32) -> bool;
pub type NPC_GetRot_t =
    unsafe extern "C" fn(npc: NPCPtr, rx: *mut f32, ry: *mut f32, rz: *mut f32) -> bool;
pub type NPC_SetFacingAngle_t = unsafe extern "C" fn(npc: NPCPtr, angle: f32) -> bool;
pub type NPC_GetFacingAngle_t = unsafe extern "C" fn(npc: NPCPtr, angle: *mut f32) -> bool;

pub type NPC_SetVirtualWorld_t = unsafe extern "C" fn(npc: NPCPtr, virtual_world: i32) -> bool;
pub type NPC_GetVirtualWorld_t = unsafe extern "C" fn(npc: NPCPtr) -> i32;
pub type NPC_SetInterior_t = unsafe extern "C" fn(npc: NPCPtr, interior: i32) -> bool;
pub type NPC_GetInterior_t = unsafe extern "C" fn(npc: NPCPtr) -> i32;

pub type NPC_Move_t = unsafe extern "C" fn(
    npc: NPCPtr,
    x: f32,
    y: f32,
    z: f32,
    move_type: i32,
    move_speed: f32,
    stop_range: f32,
) -> bool;
pub type NPC_MoveToPlayer_t = unsafe extern "C" fn(
    npc: NPCPtr,
    player: PlayerPtr,
    move_type: i32,
    move_speed: f32,
    stop_range: f32,
    pos_check_update_delay: i32,
    auto_restart: bool,
) -> bool;
pub type NPC_StopMove_t = unsafe extern "C" fn(npc: NPCPtr) -> bool;
pub type NPC_IsMoving_t = unsafe extern "C" fn(npc: NPCPtr) -> bool;

pub type NPC_SetSkin_t = unsafe extern "C" fn(npc: NPCPtr, model: i32) -> bool;

pub type NPC_IsStreamedIn_t = unsafe extern "C" fn(npc: NPCPtr, player: PlayerPtr) -> bool;
pub type NPC_IsAnyStreamedIn_t = unsafe extern "C" fn(npc: NPCPtr) -> bool;

pub type NPC_SetHealth_t = unsafe extern "C" fn(npc: NPCPtr, health: f32) -> bool;
pub type NPC_GetHealth_t = unsafe extern "C" fn(npc: NPCPtr) -> f32;
pub type NPC_SetArmour_t = unsafe extern "C" fn(npc: NPCPtr, armour: f32) -> bool;
pub type NPC_GetArmour_t = unsafe extern "C" fn(npc: NPCPtr) -> f32;
pub type NPC_IsDead_t = unsafe extern "C" fn(npc: NPCPtr) -> bool;
pub type NPC_SetInvulnerable_t = unsafe extern "C" fn(npc: NPCPtr, toggle: bool) -> bool;
pub type NPC_IsInvulnerable_t = unsafe extern "C" fn(npc: NPCPtr) -> bool;

pub type NPC_SetWeapon_t = unsafe extern "C" fn(npc: NPCPtr, weapon: u8) -> bool;
pub type NPC_GetWeapon_t = unsafe extern "C" fn(npc: NPCPtr) -> u8;
pub type NPC_SetAmmo_t = unsafe extern "C" fn(npc: NPCPtr, ammo: i32) -> bool;
pub type NPC_GetAmmo_t = unsafe extern "C" fn(npc: NPCPtr) -> i32;
pub type NPC_SetAmmoInClip_t = unsafe extern "C" fn(npc: NPCPtr, ammo: i32) -> bool;
pub type NPC_GetAmmoInClip_t = unsafe extern "C" fn(npc: NPCPtr) -> i32;

pub type NPC_EnableReloading_t = unsafe extern "C" fn(npc: NPCPtr, enable: bool) -> bool;
pub type NPC_IsReloadEnabled_t = unsafe extern "C" fn(npc: NPCPtr) -> bool;
pub type NPC_IsReloading_t = unsafe extern "C" fn(npc: NPCPtr) -> bool;
pub type NPC_EnableInfiniteAmmo_t = unsafe extern "C" fn(npc: NPCPtr, enable: bool) -> bool;
pub type NPC_IsInfiniteAmmoEnabled_t = unsafe extern "C" fn(npc: NPCPtr) -> bool;
pub type NPC_GetWeaponState_t = unsafe extern "C" fn(npc: NPCPtr) -> i32;

pub type NPC_SetKeys_t =
    unsafe extern "C" fn(npc: NPCPtr, up_and_down: u16, left_and_right: u16, keys: u16) -> bool;
pub type NPC_GetKeys_t = unsafe extern "C" fn(
    npc: NPCPtr,
    up_and_down: *mut u16,
    left_and_right: *mut u16,
    keys: *mut u16,
) -> bool;

pub type NPC_SetWeaponSkillLevel_t =
    unsafe extern "C" fn(npc: NPCPtr, skill: u8, level: i32) -> bool;
pub type NPC_GetWeaponSkillLevel_t = unsafe extern "C" fn(npc: NPCPtr, skill: i32) -> i32;

#[repr(C)]
pub struct NPCAPI {
    pub connect: Option<NPC_Connect_t>,
    pub create: Option<NPC_Create_t>,
    pub destroy: Option<NPC_Destroy_t>,
    pub from_id: Option<NPC_FromID_t>,
    pub get_id: Option<NPC_GetID_t>,
    pub is_valid: Option<NPC_IsValid_t>,
    pub get_player: Option<NPC_GetPlayer_t>,

    pub spawn: Option<NPC_Spawn_t>,
    pub respawn: Option<NPC_Respawn_t>,

    pub set_pos: Option<NPC_SetPos_t>,
    pub get_pos: Option<NPC_GetPos_t>,
    pub set_rot: Option<NPC_SetRot_t>,
    pub get_rot: Option<NPC_GetRot_t>,
    pub set_facing_angle: Option<NPC_SetFacingAngle_t>,
    pub get_facing_angle: Option<NPC_GetFacingAngle_t>,

    pub set_virtual_world: Option<NPC_SetVirtualWorld_t>,
    pub get_virtual_world: Option<NPC_GetVirtualWorld_t>,
    pub set_interior: Option<NPC_SetInterior_t>,
    pub get_interior: Option<NPC_GetInterior_t>,

    pub move_: Option<NPC_Move_t>,
    pub move_to_player: Option<NPC_MoveToPlayer_t>,
    pub stop_move: Option<NPC_StopMove_t>,
    pub is_moving: Option<NPC_IsMoving_t>,

    pub set_skin: Option<NPC_SetSkin_t>,

    pub is_streamed_in: Option<NPC_IsStreamedIn_t>,
    pub is_any_streamed_in: Option<NPC_IsAnyStreamedIn_t>,

    pub set_health: Option<NPC_SetHealth_t>,
    pub get_health: Option<NPC_GetHealth_t>,
    pub set_armour: Option<NPC_SetArmour_t>,
    pub get_armour: Option<NPC_GetArmour_t>,
    pub is_dead: Option<NPC_IsDead_t>,
    pub set_invulnerable: Option<NPC_SetInvulnerable_t>,
    pub is_invulnerable: Option<NPC_IsInvulnerable_t>,

    pub set_weapon: Option<NPC_SetWeapon_t>,
    pub get_weapon: Option<NPC_GetWeapon_t>,
    pub set_ammo: Option<NPC_SetAmmo_t>,
    pub get_ammo: Option<NPC_GetAmmo_t>,
    pub set_ammo_in_clip: Option<NPC_SetAmmoInClip_t>,
    pub get_ammo_in_clip: Option<NPC_GetAmmoInClip_t>,

    pub enable_reloading: Option<NPC_EnableReloading_t>,
    pub is_reload_enabled: Option<NPC_IsReloadEnabled_t>,
    pub is_reloading: Option<NPC_IsReloading_t>,
    pub enable_infinite_ammo: Option<NPC_EnableInfiniteAmmo_t>,
    pub is_infinite_ammo_enabled: Option<NPC_IsInfiniteAmmoEnabled_t>,
    pub get_weapon_state: Option<NPC_GetWeaponState_t>,

    pub set_keys: Option<NPC_SetKeys_t>,
    pub get_keys: Option<NPC_GetKeys_t>,

    pub set_weapon_skill_level: Option<NPC_SetWeaponSkillLevel_t>,
    pub get_weapon_skill_level: Option<NPC_GetWeaponSkillLevel_t>,
}

impl Default for NPCAPI {
    fn default() -> Self {
        Self {
            connect: None,
            create: None,
            destroy: None,
            from_id: None,
            get_id: None,
            is_valid: None,
            get_player: None,
            spawn: None,
            respawn: None,
            set_pos: None,
            get_pos: None,
            set_rot: None,
            get_rot: None,
            set_facing_angle: None,
            get_facing_angle: None,
            set_virtual_world: None,
            get_virtual_world: None,
            set_interior: None,
            get_interior: None,
            move_: None,
            move_to_player: None,
            stop_move: None,
            is_moving: None,
            set_skin: None,
            is_streamed_in: None,
            is_any_streamed_in: None,
            set_health: None,
            get_health: None,
            set_armour: None,
            get_armour: None,
            is_dead: None,
            set_invulnerable: None,
            is_invulnerable: None,
            set_weapon: None,
            get_weapon: None,
            set_ammo: None,
            get_ammo: None,
            set_ammo_in_clip: None,
            get_ammo_in_clip: None,
            enable_reloading: None,
            is_reload_enabled: None,
            is_reloading: None,
            enable_infinite_ammo: None,
            is_infinite_ammo_enabled: None,
            get_weapon_state: None,
            set_keys: None,
            get_keys: None,
            set_weapon_skill_level: None,
            get_weapon_skill_level: None,
        }
    }
}
