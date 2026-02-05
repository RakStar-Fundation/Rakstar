#![allow(non_snake_case, non_camel_case_types)]
use crate::types::*;

pub type Player_SetSpawnInfo_t = unsafe extern "C" fn(
    player: PlayerPtr,
    team: u8,
    skin: i32,
    x: f32,
    y: f32,
    z: f32,
    angle: f32,
    weapon1: u8,
    ammo1: u32,
    weapon2: u8,
    ammo2: u32,
    weapon3: u8,
    ammo3: u32,
) -> bool;

pub type Player_GetSpawnInfo_t = unsafe extern "C" fn(
    player: PlayerPtr,
    team: *mut u8,
    skin: *mut i32,
    x: *mut f32,
    y: *mut f32,
    z: *mut f32,
    angle: *mut f32,
    weapon1: *mut u8,
    ammo1: *mut u32,
    weapon2: *mut u8,
    ammo2: *mut u32,
    weapon3: *mut u8,
    ammo3: *mut u32,
) -> bool;

pub type Player_FromID_t = unsafe extern "C" fn(playerid: i32) -> PlayerPtr;
pub type Player_GetID_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;

pub type Player_SendClientMessage_t =
    unsafe extern "C" fn(player: PlayerPtr, color: u32, text: *const u8) -> bool;

pub type Player_SetPos_t = unsafe extern "C" fn(player: PlayerPtr, x: f32, y: f32, z: f32) -> bool;

pub type Player_GetPos_t =
    unsafe extern "C" fn(player: PlayerPtr, x: *mut f32, y: *mut f32, z: *mut f32) -> bool;

pub type Player_SetHealth_t = unsafe extern "C" fn(player: PlayerPtr, health: f32) -> bool;
pub type Player_GetHealth_t = unsafe extern "C" fn(player: PlayerPtr) -> f32;

pub type Player_SetArmor_t = unsafe extern "C" fn(player: PlayerPtr, armor: f32) -> bool;
pub type Player_GetArmor_t = unsafe extern "C" fn(player: PlayerPtr) -> f32;

pub type Player_SetSkin_t = unsafe extern "C" fn(player: PlayerPtr, skin: i32) -> bool;
pub type Player_GetSkin_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;

pub type Player_SetColor_t = unsafe extern "C" fn(player: PlayerPtr, color: u32) -> bool;
pub type Player_GetColor_t = unsafe extern "C" fn(player: PlayerPtr) -> u32;
pub type Player_GetDefaultColor_t = unsafe extern "C" fn(player: PlayerPtr) -> u32;

pub type Player_SetScore_t = unsafe extern "C" fn(player: PlayerPtr, score: i32) -> bool;
pub type Player_GetScore_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;

pub type Player_SetTeam_t = unsafe extern "C" fn(player: PlayerPtr, team: i32) -> bool;
pub type Player_GetTeam_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;

pub type Player_SetName_t = unsafe extern "C" fn(player: PlayerPtr, name: *const u8) -> i32;
pub type Player_GetName_t =
    unsafe extern "C" fn(player: PlayerPtr, name: *mut CAPIStringView) -> i32;

pub type Player_GiveMoney_t = unsafe extern "C" fn(player: PlayerPtr, amount: i32) -> bool;
pub type Player_GetMoney_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;
pub type Player_ResetMoney_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;

pub type Player_GiveWeapon_t =
    unsafe extern "C" fn(player: PlayerPtr, weapon: i32, ammo: i32) -> bool;

pub type Player_RemoveWeapon_t = unsafe extern "C" fn(player: PlayerPtr, weapon: i32) -> bool;
pub type Player_ResetWeapons_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;
pub type Player_GetWeapon_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;

pub type Player_SetInterior_t = unsafe extern "C" fn(player: PlayerPtr, interior: i32) -> bool;
pub type Player_GetInterior_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;

pub type Player_SetVirtualWorld_t = unsafe extern "C" fn(player: PlayerPtr, vw: i32) -> bool;
pub type Player_GetVirtualWorld_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;

pub type Player_Kick_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;
pub type Player_Ban_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;
pub type Player_BanEx_t = unsafe extern "C" fn(player: PlayerPtr, reason: *const u8) -> bool;

pub type Player_IsAdmin_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;
pub type Player_SetAdmin_t = unsafe extern "C" fn(player: PlayerPtr, set: bool) -> bool;

pub type Player_IsNPC_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;
pub type Player_IsSpawned_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;

pub type Player_GetState_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;
pub type Player_GetPing_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;

pub type Player_SetFacingAngle_t = unsafe extern "C" fn(player: PlayerPtr, angle: f32) -> bool;
pub type Player_GetFacingAngle_t = unsafe extern "C" fn(player: PlayerPtr) -> f32;

pub type Player_GetIp_t = unsafe extern "C" fn(player: PlayerPtr, ip: *mut CAPIStringBuffer) -> i32;
pub type Player_GetRawIp_t = unsafe extern "C" fn(player: PlayerPtr) -> u32;

pub type Player_SetTime_t = unsafe extern "C" fn(player: PlayerPtr, hour: i32, minute: i32) -> bool;
pub type Player_GetTime_t =
    unsafe extern "C" fn(player: PlayerPtr, hour: *mut i32, minute: *mut i32) -> bool;

pub type Player_SetWeather_t = unsafe extern "C" fn(player: PlayerPtr, weather: i32) -> bool;
pub type Player_GetWeather_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;

pub type Player_SetGravity_t = unsafe extern "C" fn(player: PlayerPtr, gravity: f32) -> bool;
pub type Player_GetGravity_t = unsafe extern "C" fn(player: PlayerPtr) -> f32;

pub type Player_Spawn_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;

pub type Player_SetVelocity_t =
    unsafe extern "C" fn(player: PlayerPtr, x: f32, y: f32, z: f32) -> bool;

pub type Player_GetVelocity_t =
    unsafe extern "C" fn(player: PlayerPtr, x: *mut f32, y: *mut f32, z: *mut f32) -> bool;

pub type Player_IsInVehicle_t =
    unsafe extern "C" fn(player: PlayerPtr, targetVehicle: VehiclePtr) -> bool;

pub type Player_IsInAnyVehicle_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;
pub type Player_GetVehicleID_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;
pub type Player_GetVehicleSeat_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;

pub type Player_PutInVehicle_t =
    unsafe extern "C" fn(player: PlayerPtr, vehicle: VehiclePtr, seat: i32) -> bool;

pub type Player_RemoveFromVehicle_t = unsafe extern "C" fn(player: PlayerPtr, force: bool) -> bool;

pub type Player_ToggleControllable_t =
    unsafe extern "C" fn(player: PlayerPtr, enable: bool) -> bool;
pub type Player_IsControllable_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;

pub type Player_ShowGameText_t =
    unsafe extern "C" fn(player: PlayerPtr, text: *const u8, time: i32, style: i32) -> bool;

pub type Player_HideGameText_t = unsafe extern "C" fn(player: PlayerPtr, style: i32) -> bool;

pub type Player_ApplyAnimation_t = unsafe extern "C" fn(
    player: PlayerPtr,
    animlib: *const u8,
    animname: *const u8,
    delta: f32,
    loop_: bool,
    lockX: bool,
    lockY: bool,
    freeze: bool,
    time: u32,
    sync: i32,
) -> bool;

pub type Player_ClearAnimations_t = unsafe extern "C" fn(player: PlayerPtr, syncType: i32) -> bool;

pub type Player_SetCameraPos_t =
    unsafe extern "C" fn(player: PlayerPtr, x: f32, y: f32, z: f32) -> bool;

pub type Player_GetCameraPos_t =
    unsafe extern "C" fn(player: PlayerPtr, x: *mut f32, y: *mut f32, z: *mut f32) -> bool;

pub type Player_SetCameraLookAt_t =
    unsafe extern "C" fn(player: PlayerPtr, x: f32, y: f32, z: f32, cutType: i32) -> bool;

pub type Player_SetCameraBehind_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;

pub type Player_GetPlayerAmmo_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;
pub type Player_SetAmmo_t = unsafe extern "C" fn(player: PlayerPtr, id: u8, ammo: u32) -> bool;
pub type Player_SetArmedWeapon_t = unsafe extern "C" fn(player: PlayerPtr, weapon: u8) -> bool;
pub type Player_GetWeaponData_t =
    unsafe extern "C" fn(player: PlayerPtr, slot: i32, weaponid: *mut i32, ammo: *mut i32) -> bool;
pub type Player_GetWeaponState_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;

pub type Player_GetKeys_t = unsafe extern "C" fn(
    player: PlayerPtr,
    keys: *mut i32,
    updown: *mut i32,
    leftright: *mut i32,
) -> bool;

// Camera
pub type Player_GetCameraMode_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;
pub type Player_GetCameraZoom_t = unsafe extern "C" fn(player: PlayerPtr) -> f32;
pub type Player_GetCameraAspectRatio_t = unsafe extern "C" fn(player: PlayerPtr) -> f32;
pub type Player_GetCameraFrontVector_t =
    unsafe extern "C" fn(player: PlayerPtr, x: *mut f32, y: *mut f32, z: *mut f32) -> bool;

// Animation
pub type Player_GetAnimationIndex_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;
pub type Player_GetAnimationFlags_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;
pub type Player_GetAnimationName_t =
    unsafe extern "C" fn(index: i32, lib: *mut CAPIStringView, name: *mut CAPIStringView) -> bool;

// Spectate
pub type Player_GetPlayerSpectateID_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;
pub type Player_GetSpectateType_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;
pub type Player_SpectatePlayer_t =
    unsafe extern "C" fn(player: PlayerPtr, target: PlayerPtr, mode: i32) -> bool;
pub type Player_SpectateVehicle_t =
    unsafe extern "C" fn(player: PlayerPtr, target: VehiclePtr, mode: i32) -> bool;

// Surfing
pub type Player_GetSurfingVehicle_t = unsafe extern "C" fn(player: PlayerPtr) -> VehiclePtr;
pub type Player_GetSurfingObject_t =
    unsafe extern "C" fn(player: PlayerPtr) -> *const std::ffi::c_void;
pub type Player_GetSurfingOffsets_t = unsafe extern "C" fn(
    player: PlayerPtr,
    offsetX: *mut f32,
    offsetY: *mut f32,
    offsetZ: *mut f32,
) -> bool;

// Target
pub type Player_GetTargetPlayer_t = unsafe extern "C" fn(player: PlayerPtr) -> PlayerPtr;
pub type Player_GetTargetActor_t =
    unsafe extern "C" fn(player: PlayerPtr) -> *const std::ffi::c_void;

// Distance
pub type Player_GetDistanceFromPoint_t =
    unsafe extern "C" fn(player: PlayerPtr, x: f32, y: f32, z: f32) -> f32;
pub type Player_IsInRangeOfPoint_t =
    unsafe extern "C" fn(player: PlayerPtr, range: f32, x: f32, y: f32, z: f32) -> bool;

// Skills
pub type Player_GetSkillLevel_t = unsafe extern "C" fn(player: PlayerPtr, skill: i32) -> i32;
pub type Player_SetSkillLevel_t =
    unsafe extern "C" fn(player: PlayerPtr, weapon: u8, level: i32) -> bool;

// Wanted/Drunk
pub type Player_GetWantedLevel_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;
pub type Player_SetWantedLevel_t = unsafe extern "C" fn(player: PlayerPtr, level: i32) -> bool;
pub type Player_GetDrunkLevel_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;
pub type Player_SetDrunkLevel_t = unsafe extern "C" fn(player: PlayerPtr, level: i32) -> bool;

// Special Action
pub type Player_GetSpecialAction_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;
pub type Player_SetSpecialAction_t = unsafe extern "C" fn(player: PlayerPtr, action: u32) -> bool;

// Fighting Style
pub type Player_GetFightingStyle_t = unsafe extern "C" fn(player: PlayerPtr) -> i32;
pub type Player_SetFightingStyle_t = unsafe extern "C" fn(player: PlayerPtr, style: i32) -> bool;

// Misc
pub type Player_ForceClassSelection_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;
pub type Player_AllowTeleport_t = unsafe extern "C" fn(player: PlayerPtr, allow: bool) -> bool;
pub type Player_IsTeleportAllowed_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;
pub type Player_AllowWeapons_t = unsafe extern "C" fn(player: PlayerPtr, allow: bool) -> bool;
pub type Player_AreWeaponsAllowed_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;
pub type Player_ToggleClock_t = unsafe extern "C" fn(player: PlayerPtr, enable: bool) -> bool;
pub type Player_HasClock_t = unsafe extern "C" fn(player: PlayerPtr) -> bool;

#[derive(Default)]
pub struct PlayerAPI {
    pub set_spawn_info: Option<Player_SetSpawnInfo_t>,
    pub get_spawn_info: Option<Player_GetSpawnInfo_t>,
    pub from_id: Option<Player_FromID_t>,
    pub get_id: Option<Player_GetID_t>,
    pub send_client_message: Option<Player_SendClientMessage_t>,
    pub set_pos: Option<Player_SetPos_t>,
    pub get_pos: Option<Player_GetPos_t>,
    pub set_health: Option<Player_SetHealth_t>,
    pub get_health: Option<Player_GetHealth_t>,
    pub set_armor: Option<Player_SetArmor_t>,
    pub get_armor: Option<Player_GetArmor_t>,
    pub set_skin: Option<Player_SetSkin_t>,
    pub get_skin: Option<Player_GetSkin_t>,
    pub set_color: Option<Player_SetColor_t>,
    pub get_color: Option<Player_GetColor_t>,
    pub get_default_color: Option<Player_GetDefaultColor_t>,
    pub set_score: Option<Player_SetScore_t>,
    pub get_score: Option<Player_GetScore_t>,
    pub set_team: Option<Player_SetTeam_t>,
    pub get_team: Option<Player_GetTeam_t>,
    pub set_name: Option<Player_SetName_t>,
    pub get_name: Option<Player_GetName_t>,
    pub give_money: Option<Player_GiveMoney_t>,
    pub get_money: Option<Player_GetMoney_t>,
    pub reset_money: Option<Player_ResetMoney_t>,
    pub give_weapon: Option<Player_GiveWeapon_t>,
    pub remove_weapon: Option<Player_RemoveWeapon_t>,
    pub reset_weapons: Option<Player_ResetWeapons_t>,
    pub get_weapon: Option<Player_GetWeapon_t>,
    pub set_interior: Option<Player_SetInterior_t>,
    pub get_interior: Option<Player_GetInterior_t>,
    pub set_virtual_world: Option<Player_SetVirtualWorld_t>,
    pub get_virtual_world: Option<Player_GetVirtualWorld_t>,
    pub kick: Option<Player_Kick_t>,
    pub ban: Option<Player_Ban_t>,
    pub ban_ex: Option<Player_BanEx_t>,
    pub is_admin: Option<Player_IsAdmin_t>,
    pub set_admin: Option<Player_SetAdmin_t>,
    pub is_npc: Option<Player_IsNPC_t>,
    pub is_spawned: Option<Player_IsSpawned_t>,
    pub get_state: Option<Player_GetState_t>,
    pub get_ping: Option<Player_GetPing_t>,
    pub set_facing_angle: Option<Player_SetFacingAngle_t>,
    pub get_facing_angle: Option<Player_GetFacingAngle_t>,
    pub get_ip: Option<Player_GetIp_t>,
    pub get_raw_ip: Option<Player_GetRawIp_t>,
    pub set_time: Option<Player_SetTime_t>,
    pub get_time: Option<Player_GetTime_t>,
    pub set_weather: Option<Player_SetWeather_t>,
    pub get_weather: Option<Player_GetWeather_t>,
    pub set_gravity: Option<Player_SetGravity_t>,
    pub get_gravity: Option<Player_GetGravity_t>,
    pub spawn: Option<Player_Spawn_t>,
    pub set_velocity: Option<Player_SetVelocity_t>,
    pub get_velocity: Option<Player_GetVelocity_t>,
    pub is_in_vehicle: Option<Player_IsInVehicle_t>,
    pub is_in_any_vehicle: Option<Player_IsInAnyVehicle_t>,
    pub get_vehicle_id: Option<Player_GetVehicleID_t>,
    pub get_vehicle_seat: Option<Player_GetVehicleSeat_t>,
    pub put_in_vehicle: Option<Player_PutInVehicle_t>,
    pub remove_from_vehicle: Option<Player_RemoveFromVehicle_t>,
    pub toggle_controllable: Option<Player_ToggleControllable_t>,
    pub is_controllable: Option<Player_IsControllable_t>,
    pub show_game_text: Option<Player_ShowGameText_t>,
    pub hide_game_text: Option<Player_HideGameText_t>,
    pub apply_animation: Option<Player_ApplyAnimation_t>,
    pub clear_animations: Option<Player_ClearAnimations_t>,
    pub set_camera_pos: Option<Player_SetCameraPos_t>,
    pub get_camera_pos: Option<Player_GetCameraPos_t>,
    pub set_camera_look_at: Option<Player_SetCameraLookAt_t>,
    pub set_camera_behind: Option<Player_SetCameraBehind_t>,
    pub get_player_ammo: Option<Player_GetPlayerAmmo_t>,
    pub set_ammo: Option<Player_SetAmmo_t>,
    pub set_armed_weapon: Option<Player_SetArmedWeapon_t>,
    pub get_weapon_data: Option<Player_GetWeaponData_t>,
    pub get_weapon_state: Option<Player_GetWeaponState_t>,
    pub get_keys: Option<Player_GetKeys_t>,
    pub get_camera_mode: Option<Player_GetCameraMode_t>,
    pub get_camera_zoom: Option<Player_GetCameraZoom_t>,
    pub get_camera_aspect_ratio: Option<Player_GetCameraAspectRatio_t>,
    pub get_camera_front_vector: Option<Player_GetCameraFrontVector_t>,
    pub get_animation_index: Option<Player_GetAnimationIndex_t>,
    pub get_animation_flags: Option<Player_GetAnimationFlags_t>,
    pub get_animation_name: Option<Player_GetAnimationName_t>,
    pub get_player_spectate_id: Option<Player_GetPlayerSpectateID_t>,
    pub get_spectate_type: Option<Player_GetSpectateType_t>,
    pub spectate_player: Option<Player_SpectatePlayer_t>,
    pub spectate_vehicle: Option<Player_SpectateVehicle_t>,
    pub get_surfing_vehicle: Option<Player_GetSurfingVehicle_t>,
    pub get_surfing_object: Option<Player_GetSurfingObject_t>,
    pub get_surfing_offsets: Option<Player_GetSurfingOffsets_t>,
    pub get_target_player: Option<Player_GetTargetPlayer_t>,
    pub get_target_actor: Option<Player_GetTargetActor_t>,
    pub get_distance_from_point: Option<Player_GetDistanceFromPoint_t>,
    pub is_in_range_of_point: Option<Player_IsInRangeOfPoint_t>,
    pub get_skill_level: Option<Player_GetSkillLevel_t>,
    pub set_skill_level: Option<Player_SetSkillLevel_t>,
    pub get_wanted_level: Option<Player_GetWantedLevel_t>,
    pub set_wanted_level: Option<Player_SetWantedLevel_t>,
    pub get_drunk_level: Option<Player_GetDrunkLevel_t>,
    pub set_drunk_level: Option<Player_SetDrunkLevel_t>,
    pub get_special_action: Option<Player_GetSpecialAction_t>,
    pub set_special_action: Option<Player_SetSpecialAction_t>,
    pub get_fighting_style: Option<Player_GetFightingStyle_t>,
    pub set_fighting_style: Option<Player_SetFightingStyle_t>,
    pub force_class_selection: Option<Player_ForceClassSelection_t>,
    pub allow_teleport: Option<Player_AllowTeleport_t>,
    pub is_teleport_allowed: Option<Player_IsTeleportAllowed_t>,
    pub allow_weapons: Option<Player_AllowWeapons_t>,
    pub are_weapons_allowed: Option<Player_AreWeaponsAllowed_t>,
    pub toggle_clock: Option<Player_ToggleClock_t>,
    pub has_clock: Option<Player_HasClock_t>,
}
