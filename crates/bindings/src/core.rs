#![allow(non_snake_case, non_camel_case_types)]
use crate::types::*;

use crate::{CAPIStringBuffer, CAPIStringView};

pub type Core_TickCount_t = unsafe extern "C" fn() -> u32;
pub type Core_MaxPlayers_t = unsafe extern "C" fn() -> i32;
pub type Core_Log_t = unsafe extern "C" fn(text: *const u8) -> bool;
pub type Core_IsAdminTeleportAllowed_t = unsafe extern "C" fn() -> bool;
pub type Core_AllowAdminTeleport_t = unsafe extern "C" fn(allow: bool) -> bool;
pub type Core_AreAllAnimationsEnabled_t = unsafe extern "C" fn() -> bool;
pub type Core_EnableAllAnimations_t = unsafe extern "C" fn(allow: bool) -> bool;
pub type Core_IsAnimationLibraryValid_t = unsafe extern "C" fn(name: *const u8) -> bool;
pub type Core_AreInteriorWeaponsAllowed_t = unsafe extern "C" fn() -> bool;
pub type Core_AllowInteriorWeapons_t = unsafe extern "C" fn(allow: bool) -> bool;
pub type Core_BlockIpAddress_t = unsafe extern "C" fn(ipAddress: *const u8, timeMS: i32) -> bool;
pub type Core_UnBlockIpAddress_t = unsafe extern "C" fn(ipAddress: *const u8) -> bool;
pub type Core_DisableEntryExitMarkers_t = unsafe extern "C" fn() -> bool;
pub type Core_DisableNameTagsLOS_t = unsafe extern "C" fn() -> bool;
pub type Core_EnableZoneNames_t = unsafe extern "C" fn(enable: bool) -> bool;
pub type Core_ShowGameTextForAll_t =
    unsafe extern "C" fn(msg: *const u8, time: i32, style: i32) -> bool;
pub type Core_HideGameTextForAll_t = unsafe extern "C" fn(style: i32) -> bool;
pub type Core_NetworkStats_t = unsafe extern "C" fn(output: *mut CAPIStringBuffer) -> i32;
pub type Core_ServerTickRate_t = unsafe extern "C" fn() -> i32;
pub type Core_GetWeaponName_t =
    unsafe extern "C" fn(weaponid: i32, output: *mut CAPIStringView) -> bool;
pub type Core_SetChatRadius_t = unsafe extern "C" fn(globalChatRadius: f32) -> bool;
pub type Core_SetMarkerRadius_t = unsafe extern "C" fn(playerMarkerRadius: f32) -> bool;
pub type Core_SendRconCommand_t = unsafe extern "C" fn(command: *const u8) -> bool;
pub type Core_SetDeathDropAmount_t = unsafe extern "C" fn(amount: i32) -> bool;
pub type Core_GameMode_SetText_t = unsafe extern "C" fn(string: *const u8) -> bool;
pub type Core_SetGravity_t = unsafe extern "C" fn(gravity: f32) -> bool;
pub type Core_GetGravity_t = unsafe extern "C" fn() -> f32;
pub type Core_SetNameTagsDrawDistance_t = unsafe extern "C" fn(distance: f32) -> bool;
pub type Core_SetWeather_t = unsafe extern "C" fn(weatherid: i32) -> bool;
pub type Core_SetWorldTime_t = unsafe extern "C" fn(hour: i32) -> bool;
pub type Core_ShowNameTags_t = unsafe extern "C" fn(show: bool) -> bool;
pub type Core_ShowPlayerMarkers_t = unsafe extern "C" fn(mode: i32) -> bool;
pub type Core_UsePedAnims_t = unsafe extern "C" fn() -> bool;
pub type Core_GetWeather_t = unsafe extern "C" fn() -> i32;
pub type Core_GetWorldTime_t = unsafe extern "C" fn() -> i32;
pub type Core_ToggleChatTextReplacement_t = unsafe extern "C" fn(enable: bool) -> bool;
pub type Core_IsChatTextReplacementToggled_t = unsafe extern "C" fn() -> bool;
pub type Core_IsNickNameValid_t = unsafe extern "C" fn(name: *const u8) -> bool;
pub type Core_AllowNickNameCharacter_t = unsafe extern "C" fn(character: i32, allow: bool) -> bool;
pub type Core_IsNickNameCharacterAllowed_t = unsafe extern "C" fn(character: i32) -> bool;
pub type Core_ClearBanList_t = unsafe extern "C" fn() -> bool;
pub type Core_IsIpAddressBanned_t = unsafe extern "C" fn(ip: *const u8) -> bool;
pub type Core_GetWeaponSlot_t = unsafe extern "C" fn(weapon: u8) -> i32;
pub type Core_AddRule_t = unsafe extern "C" fn(name: *const u8, value: *const u8) -> bool;
pub type Core_IsValidRule_t = unsafe extern "C" fn(name: *const u8) -> bool;
pub type Core_RemoveRule_t = unsafe extern "C" fn(name: *const u8) -> bool;

#[repr(C)]
pub struct CoreAPI {
    pub tick_count: Option<Core_TickCount_t>,
    pub max_players: Option<Core_MaxPlayers_t>,
    pub log: Option<Core_Log_t>,
    pub is_admin_teleport_allowed: Option<Core_IsAdminTeleportAllowed_t>,
    pub allow_admin_teleport: Option<Core_AllowAdminTeleport_t>,
    pub are_all_animations_enabled: Option<Core_AreAllAnimationsEnabled_t>,
    pub enable_all_animations: Option<Core_EnableAllAnimations_t>,
    pub is_animation_library_valid: Option<Core_IsAnimationLibraryValid_t>,
    pub are_interior_weapons_allowed: Option<Core_AreInteriorWeaponsAllowed_t>,
    pub allow_interior_weapons: Option<Core_AllowInteriorWeapons_t>,
    pub block_ip_address: Option<Core_BlockIpAddress_t>,
    pub un_block_ip_address: Option<Core_UnBlockIpAddress_t>,
    pub disable_entry_exit_markers: Option<Core_DisableEntryExitMarkers_t>,
    pub disable_name_tags_los: Option<Core_DisableNameTagsLOS_t>,
    pub enable_zone_names: Option<Core_EnableZoneNames_t>,
    pub show_game_text_for_all: Option<Core_ShowGameTextForAll_t>,
    pub hide_game_text_for_all: Option<Core_HideGameTextForAll_t>,
    pub network_stats: Option<Core_NetworkStats_t>,
    pub server_tick_rate: Option<Core_ServerTickRate_t>,
    pub get_weapon_name: Option<Core_GetWeaponName_t>,
    pub set_chat_radius: Option<Core_SetChatRadius_t>,
    pub set_marker_radius: Option<Core_SetMarkerRadius_t>,
    pub send_rcon_command: Option<Core_SendRconCommand_t>,
    pub set_death_drop_amount: Option<Core_SetDeathDropAmount_t>,
    pub game_mode_set_text: Option<Core_GameMode_SetText_t>,
    pub set_gravity: Option<Core_SetGravity_t>,
    pub get_gravity: Option<Core_GetGravity_t>,
    pub set_name_tags_draw_distance: Option<Core_SetNameTagsDrawDistance_t>,
    pub set_weather: Option<Core_SetWeather_t>,
    pub set_world_time: Option<Core_SetWorldTime_t>,
    pub show_name_tags: Option<Core_ShowNameTags_t>,
    pub show_player_markers: Option<Core_ShowPlayerMarkers_t>,
    pub use_ped_anims: Option<Core_UsePedAnims_t>,
    pub get_weather: Option<Core_GetWeather_t>,
    pub get_world_time: Option<Core_GetWorldTime_t>,
    pub toggle_chat_text_replacement: Option<Core_ToggleChatTextReplacement_t>,
    pub is_chat_text_replacement_toggled: Option<Core_IsChatTextReplacementToggled_t>,
    pub is_nick_name_valid: Option<Core_IsNickNameValid_t>,
    pub allow_nick_name_character: Option<Core_AllowNickNameCharacter_t>,
    pub is_nick_name_character_allowed: Option<Core_IsNickNameCharacterAllowed_t>,
    pub clear_ban_list: Option<Core_ClearBanList_t>,
    pub is_ip_address_banned: Option<Core_IsIpAddressBanned_t>,
    pub get_weapon_slot: Option<Core_GetWeaponSlot_t>,
    pub add_rule: Option<Core_AddRule_t>,
    pub is_valid_rule: Option<Core_IsValidRule_t>,
    pub remove_rule: Option<Core_RemoveRule_t>,
}
