use super::types::ComponentVersion;
use crate::npc::NPCAPI;
use crate::textdraw::TextDrawAPI;
use crate::vehicle::VehicleAPI;
use crate::{core::CoreAPI, player::PlayerAPI, player_object::PlayerObjectAPI};
use libloading::{Library, Symbol};
use std::ffi::{c_void, CString};

pub type ComponentCreateFn = unsafe extern "C" fn(
    uid: u64,
    name: *const i8,
    version: ComponentVersion,
    on_ready: *const c_void,
    on_reset: *const c_void,
    on_free: *const c_void,
) -> *mut c_void;

pub type EventCallback = unsafe extern "C" fn() -> bool;

pub type EventAddHandlerFn =
    unsafe extern "C" fn(name: *const i8, priority: i32, callback: EventCallback) -> bool;

#[repr(C)]
pub struct ComponentApi {
    pub create: Option<ComponentCreateFn>,
}

#[repr(C)]
pub struct EventApi {
    pub add_handler: Option<EventAddHandlerFn>,
}

#[repr(C)]
pub struct OmpApi {
    pub actor: *const c_void,
    pub checkpoint: crate::checkpoint::CheckpointAPI,
    pub race_checkpoint: *const c_void,
    pub class: *const c_void,
    pub player: PlayerAPI,
    pub component: ComponentApi,
    pub config: *const c_void,
    pub core: CoreAPI,
    pub npc: NPCAPI,
    pub custom_model: *const c_void,
    pub dialog: crate::dialog::DialogAPI,
    pub event: EventApi,
    pub gang_zone: *const c_void,
    pub menu: *const c_void,
    pub object: *const c_void,
    pub player_object: PlayerObjectAPI,
    pub pickup: *const c_void,
    pub all: *const c_void,
    pub recording: *const c_void,
    pub text_draw: TextDrawAPI,
    pub player_text_draw: *const c_void,
    pub text_label: *const c_void,
    pub player_text_label: *const c_void,
    pub vehicle: VehicleAPI,
}

unsafe impl Send for OmpApi {}
unsafe impl Sync for OmpApi {}

macro_rules! load_fn {
    ($lib:expr, $api:expr, $field:expr, $name:expr) => {
        let fn_name = $name.as_bytes();
        unsafe {
            let sym: Result<Symbol<_>, _> = $lib.get(fn_name);
            if let Ok(sym) = sym {
                $field = Some(*sym);
            }
        }
    };
}

pub unsafe fn initialize_capi(api: *mut OmpApi) -> bool {
    let lib_path = if cfg!(unix) {
        "./components/$CAPI.so"
    } else if cfg!(windows) {
        "./components/$CAPI.dll"
    } else {
        return false;
    };

    let lib = match Library::new(lib_path) {
        Ok(lib) => lib,
        Err(e) => {
            eprintln!("Failed to load library {}: {}", lib_path, e);
            return false;
        }
    };

    // Component_Create is required
    let create_fn_name = b"Component_Create\0";
    let create_fn: Symbol<ComponentCreateFn> = match lib.get(create_fn_name) {
        Ok(sym) => sym,
        Err(e) => {
            eprintln!("Failed to find Component_Create: {}", e);
            return false;
        }
    };
    
    (*api).component.create = Some(*create_fn);

    // Load all other functions
    load_fn!(lib, api, (*api).core.max_players, "Core_MaxPlayers\0");

    // Player functions
    load_fn!(lib, api, (*api).player.get_name, "Player_GetName\0");
    load_fn!(lib, api, (*api).player.from_id, "Player_FromID\0");
    load_fn!(lib, api, (*api).player.get_id, "Player_GetID\0");
    load_fn!(lib, api, (*api).player.kick, "Player_Kick\0");
    load_fn!(lib, api, (*api).player.ban, "Player_Ban\0");
    load_fn!(lib, api, (*api).player.spawn, "Player_Spawn\0");
    load_fn!(lib, api, (*api).player.get_health, "Player_GetHealth\0");
    load_fn!(lib, api, (*api).player.set_health, "Player_SetHealth\0");
    load_fn!(lib, api, (*api).player.get_armor, "Player_GetArmour\0");
    load_fn!(lib, api, (*api).player.set_armor, "Player_SetArmour\0");
    load_fn!(lib, api, (*api).player.get_pos, "Player_GetPos\0");
    load_fn!(lib, api, (*api).player.set_pos, "Player_SetPos\0");
    load_fn!(lib, api, (*api).player.get_interior, "Player_GetInterior\0");
    load_fn!(lib, api, (*api).player.set_interior, "Player_SetInterior\0");
    load_fn!(
        lib,
        api,
        (*api).player.get_virtual_world,
        "Player_GetVirtualWorld\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.set_virtual_world,
        "Player_SetVirtualWorld\0"
    );
    load_fn!(lib, api, (*api).player.reset_weapons, "Player_ResetWeapons\0");
    load_fn!(lib, api, (*api).player.give_weapon, "Player_GiveWeapon\0");
    load_fn!(
        lib,
        api,
        (*api).player.put_in_vehicle,
        "Player_PutInVehicle\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.remove_from_vehicle,
        "Player_RemoveFromVehicle\0"
    );
    load_fn!(lib, api, (*api).player.is_in_vehicle, "Player_IsInVehicle\0");
    load_fn!(
        lib,
        api,
        (*api).player.is_in_any_vehicle,
        "Player_IsInAnyVehicle\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_vehicle_id,
        "Player_GetVehicleID\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_facing_angle,
        "Player_GetFacingAngle\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.set_facing_angle,
        "Player_SetFacingAngle\0"
    );
    load_fn!(lib, api, (*api).player.is_spawned, "Player_IsSpawned\0");
    load_fn!(
        lib,
        api,
        (*api).player.get_player_ammo,
        "Player_GetPlayerAmmo\0"
    );
    load_fn!(lib, api, (*api).player.set_ammo, "Player_SetAmmo\0");
    load_fn!(
        lib,
        api,
        (*api).player.set_armed_weapon,
        "Player_SetArmedWeapon\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_weapon_data,
        "Player_GetWeaponData\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_weapon_state,
        "Player_GetWeaponState\0"
    );
    load_fn!(lib, api, (*api).player.get_keys, "Player_GetKeys\0");
    load_fn!(
        lib,
        api,
        (*api).player.get_camera_mode,
        "Player_GetCameraMode\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_camera_zoom,
        "Player_GetCameraZoom\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_camera_aspect_ratio,
        "Player_GetCameraAspectRatio\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_camera_front_vector,
        "Player_GetCameraFrontVector\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_animation_index,
        "Player_GetAnimationIndex\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_animation_flags,
        "Player_GetAnimationFlags\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_animation_name,
        "Player_GetAnimationName\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_player_spectate_id,
        "Player_GetPlayerSpectateID\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_spectate_type,
        "Player_GetSpectateType\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.spectate_player,
        "Player_SpectatePlayer\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.spectate_vehicle,
        "Player_SpectateVehicle\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_surfing_vehicle,
        "Player_GetSurfingVehicle\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_surfing_object,
        "Player_GetSurfingObject\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_surfing_offsets,
        "Player_GetSurfingOffsets\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_target_player,
        "Player_GetTargetPlayer\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_target_actor,
        "Player_GetTargetActor\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_distance_from_point,
        "Player_GetDistanceFromPoint\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.is_in_range_of_point,
        "Player_IsInRangeOfPoint\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_skill_level,
        "Player_GetSkillLevel\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.set_skill_level,
        "Player_SetSkillLevel\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_wanted_level,
        "Player_GetWantedLevel\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.set_wanted_level,
        "Player_SetWantedLevel\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_drunk_level,
        "Player_GetDrunkLevel\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.set_drunk_level,
        "Player_SetDrunkLevel\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_special_action,
        "Player_GetSpecialAction\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.set_special_action,
        "Player_SetSpecialAction\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_fighting_style,
        "Player_GetFightingStyle\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.set_fighting_style,
        "Player_SetFightingStyle\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.force_class_selection,
        "Player_ForceClassSelection\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.allow_teleport,
        "Player_AllowTeleport\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player.is_teleport_allowed,
        "Player_IsTeleportAllowed\0"
    );
    load_fn!(lib, api, (*api).player.allow_weapons, "Player_AllowWeapons\0");
    load_fn!(
        lib,
        api,
        (*api).player.are_weapons_allowed,
        "Player_AreWeaponsAllowed\0"
    );
    load_fn!(lib, api, (*api).player.toggle_clock, "Player_ToggleClock\0");
    load_fn!(lib, api, (*api).player.has_clock, "Player_HasClock\0");

    // Vehicle functions
    load_fn!(lib, api, (*api).vehicle.create, "Vehicle_Create\0");
    load_fn!(lib, api, (*api).vehicle.destroy, "Vehicle_Destroy\0");
    load_fn!(lib, api, (*api).vehicle.get_id, "Vehicle_GetID\0");
    load_fn!(lib, api, (*api).vehicle.get_pos, "Vehicle_GetPos\0");
    load_fn!(lib, api, (*api).vehicle.set_pos, "Vehicle_SetPos\0");
    load_fn!(lib, api, (*api).vehicle.get_rotation, "Vehicle_GetZAngle\0");
    load_fn!(lib, api, (*api).vehicle.set_rotation, "Vehicle_SetZAngle\0");
    load_fn!(lib, api, (*api).vehicle.get_health, "Vehicle_GetHealth\0");
    load_fn!(lib, api, (*api).vehicle.set_health, "Vehicle_SetHealth\0");
    load_fn!(lib, api, (*api).vehicle.get_model, "Vehicle_GetModel\0");
    load_fn!(lib, api, (*api).vehicle.get_interior, "Vehicle_GetInterior\0");
    load_fn!(lib, api, (*api).vehicle.set_interior, "Vehicle_SetInterior\0");
    load_fn!(
        lib,
        api,
        (*api).vehicle.get_virtual_world,
        "Vehicle_GetVirtualWorld\0"
    );
    load_fn!(
        lib,
        api,
        (*api).vehicle.set_virtual_world,
        "Vehicle_SetVirtualWorld\0"
    );

    // Dialog functions
    load_fn!(lib, api, (*api).dialog.show, "Dialog_Show\0");
    load_fn!(lib, api, (*api).dialog.hide, "Dialog_Hide\0");

    // PlayerObject functions
    load_fn!(lib, api, (*api).player_object.create, "PlayerObject_Create\0");
    load_fn!(
        lib,
        api,
        (*api).player_object.destroy,
        "PlayerObject_Destroy\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.from_id,
        "PlayerObject_FromID\0"
    );
    load_fn!(lib, api, (*api).player_object.get_id, "PlayerObject_GetID\0");
    load_fn!(
        lib,
        api,
        (*api).player_object.attach_to_vehicle,
        "PlayerObject_AttachToVehicle\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.attach_to_player,
        "PlayerObject_AttachToPlayer\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.attach_to_object,
        "PlayerObject_AttachToObject\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.set_pos,
        "PlayerObject_SetPos\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_pos,
        "PlayerObject_GetPos\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.set_rot,
        "PlayerObject_SetRot\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_rot,
        "PlayerObject_GetRot\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_model,
        "PlayerObject_GetModel\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.set_no_camera_collision,
        "PlayerObject_SetNoCameraCollision\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.is_valid,
        "PlayerObject_IsValid\0"
    );
    load_fn!(lib, api, (*api).player_object.move_, "PlayerObject_Move\0");
    load_fn!(lib, api, (*api).player_object.stop, "PlayerObject_Stop\0");
    load_fn!(
        lib,
        api,
        (*api).player_object.is_moving,
        "PlayerObject_IsMoving\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.begin_editing,
        "PlayerObject_BeginEditing\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.set_material,
        "PlayerObject_SetMaterial\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.set_material_text,
        "PlayerObject_SetMaterialText\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_draw_distance,
        "PlayerObject_GetDrawDistance\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_move_speed,
        "PlayerObject_GetMoveSpeed\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_moving_target_pos,
        "PlayerObject_GetMovingTargetPos\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_moving_target_rot,
        "PlayerObject_GetMovingTargetRot\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_attached_data,
        "PlayerObject_GetAttachedData\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_attached_offset,
        "PlayerObject_GetAttachedOffset\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_sync_rotation,
        "PlayerObject_GetSyncRotation\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.is_material_slot_used,
        "PlayerObject_IsMaterialSlotUsed\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_material,
        "PlayerObject_GetMaterial\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_material_text,
        "PlayerObject_GetMaterialText\0"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.is_no_camera_collision,
        "PlayerObject_IsNoCameraCollision\0"
    );

    // Event functions
    load_fn!(lib, api, (*api).event.add_handler, "Event_AddHandler\0");

    // Checkpoint functions
    load_fn!(lib, api, (*api).checkpoint.set, "Checkpoint_Set\0");
    load_fn!(lib, api, (*api).checkpoint.disable, "Checkpoint_Disable\0");
    load_fn!(
        lib,
        api,
        (*api).checkpoint.is_player_in,
        "Checkpoint_IsPlayerIn\0"
    );
    load_fn!(lib, api, (*api).checkpoint.is_active, "Checkpoint_IsActive\0");
    load_fn!(lib, api, (*api).checkpoint.get, "Checkpoint_Get\0");

    // RaceCheckpoint functions
    load_fn!(lib, api, (*api).checkpoint.race_set, "RaceCheckpoint_Set\0");
    load_fn!(
        lib,
        api,
        (*api).checkpoint.race_disable,
        "RaceCheckpoint_Disable\0"
    );
    load_fn!(
        lib,
        api,
        (*api).checkpoint.race_is_player_in,
        "RaceCheckpoint_IsPlayerIn\0"
    );
    load_fn!(
        lib,
        api,
        (*api).checkpoint.race_is_active,
        "RaceCheckpoint_IsActive\0"
    );
    load_fn!(lib, api, (*api).checkpoint.race_get, "RaceCheckpoint_Get\0");

    // NPC functions
    load_fn!(lib, api, (*api).npc.connect, "NPC_Connect\0");
    load_fn!(lib, api, (*api).npc.create, "NPC_Create\0");
    load_fn!(lib, api, (*api).npc.destroy, "NPC_Destroy\0");
    load_fn!(lib, api, (*api).npc.from_id, "NPC_FromID\0");
    load_fn!(lib, api, (*api).npc.get_id, "NPC_GetID\0");
    load_fn!(lib, api, (*api).npc.is_valid, "NPC_IsValid\0");
    load_fn!(lib, api, (*api).npc.get_player, "NPC_GetPlayer\0");
    load_fn!(lib, api, (*api).npc.spawn, "NPC_Spawn\0");
    load_fn!(lib, api, (*api).npc.respawn, "NPC_Respawn\0");
    load_fn!(lib, api, (*api).npc.set_pos, "NPC_SetPos\0");
    load_fn!(lib, api, (*api).npc.get_pos, "NPC_GetPos\0");
    load_fn!(lib, api, (*api).npc.set_rot, "NPC_SetRot\0");
    load_fn!(lib, api, (*api).npc.get_rot, "NPC_GetRot\0");
    load_fn!(lib, api, (*api).npc.set_facing_angle, "NPC_SetFacingAngle\0");
    load_fn!(lib, api, (*api).npc.get_facing_angle, "NPC_GetFacingAngle\0");
    load_fn!(
        lib,
        api,
        (*api).npc.set_virtual_world,
        "NPC_SetVirtualWorld\0"
    );
    load_fn!(
        lib,
        api,
        (*api).npc.get_virtual_world,
        "NPC_GetVirtualWorld\0"
    );
    load_fn!(lib, api, (*api).npc.set_interior, "NPC_SetInterior\0");
    load_fn!(lib, api, (*api).npc.get_interior, "NPC_GetInterior\0");
    load_fn!(lib, api, (*api).npc.move_, "NPC_Move\0");
    load_fn!(lib, api, (*api).npc.move_to_player, "NPC_MoveToPlayer\0");
    load_fn!(lib, api, (*api).npc.stop_move, "NPC_StopMove\0");
    load_fn!(lib, api, (*api).npc.is_moving, "NPC_IsMoving\0");
    load_fn!(lib, api, (*api).npc.set_skin, "NPC_SetSkin\0");
    load_fn!(lib, api, (*api).npc.is_streamed_in, "NPC_IsStreamedIn\0");
    load_fn!(
        lib,
        api,
        (*api).npc.is_any_streamed_in,
        "NPC_IsAnyStreamedIn\0"
    );
    load_fn!(lib, api, (*api).npc.set_health, "NPC_SetHealth\0");
    load_fn!(lib, api, (*api).npc.get_health, "NPC_GetHealth\0");
    load_fn!(lib, api, (*api).npc.set_armour, "NPC_SetArmour\0");
    load_fn!(lib, api, (*api).npc.get_armour, "NPC_GetArmour\0");
    load_fn!(lib, api, (*api).npc.is_dead, "NPC_IsDead\0");
    load_fn!(lib, api, (*api).npc.set_invulnerable, "NPC_SetInvulnerable\0");
    load_fn!(lib, api, (*api).npc.is_invulnerable, "NPC_IsInvulnerable\0");
    load_fn!(lib, api, (*api).npc.set_weapon, "NPC_SetWeapon\0");
    load_fn!(lib, api, (*api).npc.get_weapon, "NPC_GetWeapon\0");
    load_fn!(lib, api, (*api).npc.set_ammo, "NPC_SetAmmo\0");
    load_fn!(lib, api, (*api).npc.get_ammo, "NPC_GetAmmo\0");
    load_fn!(lib, api, (*api).npc.set_ammo_in_clip, "NPC_SetAmmoInClip\0");
    load_fn!(lib, api, (*api).npc.get_ammo_in_clip, "NPC_GetAmmoInClip\0");
    load_fn!(lib, api, (*api).npc.enable_reloading, "NPC_EnableReloading\0");
    load_fn!(
        lib,
        api,
        (*api).npc.is_reload_enabled,
        "NPC_IsReloadEnabled\0"
    );
    load_fn!(lib, api, (*api).npc.is_reloading, "NPC_IsReloading\0");
    load_fn!(
        lib,
        api,
        (*api).npc.enable_infinite_ammo,
        "NPC_EnableInfiniteAmmo\0"
    );
    load_fn!(
        lib,
        api,
        (*api).npc.is_infinite_ammo_enabled,
        "NPC_IsInfiniteAmmoEnabled\0"
    );
    load_fn!(lib, api, (*api).npc.get_weapon_state, "NPC_GetWeaponState\0");
    load_fn!(lib, api, (*api).npc.set_keys, "NPC_SetKeys\0");
    load_fn!(lib, api, (*api).npc.get_keys, "NPC_GetKeys\0");
    load_fn!(
        lib,
        api,
        (*api).npc.set_weapon_skill_level,
        "NPC_SetWeaponSkillLevel\0"
    );
    load_fn!(
        lib,
        api,
        (*api).npc.get_weapon_skill_level,
        "NPC_GetWeaponSkillLevel\0"
    );

    // TextDraw functions
    load_fn!(lib, api, (*api).text_draw.create, "TextDraw_Create\0");
    load_fn!(lib, api, (*api).text_draw.destroy, "TextDraw_Destroy\0");
    load_fn!(lib, api, (*api).text_draw.from_id, "TextDraw_FromID\0");
    load_fn!(lib, api, (*api).text_draw.get_id, "TextDraw_GetID\0");
    load_fn!(lib, api, (*api).text_draw.is_valid, "TextDraw_IsValid\0");
    load_fn!(
        lib,
        api,
        (*api).text_draw.is_visible_for_player,
        "TextDraw_IsVisibleForPlayer\0"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_letter_size,
        "TextDraw_SetLetterSize\0"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_text_size,
        "TextDraw_SetTextSize\0"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_alignment,
        "TextDraw_SetAlignment\0"
    );
    load_fn!(lib, api, (*api).text_draw.set_color, "TextDraw_SetColor\0");
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_box_color,
        "TextDraw_SetBoxColor\0"
    );
    load_fn!(lib, api, (*api).text_draw.set_use_box, "TextDraw_SetUseBox\0");
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_outline,
        "TextDraw_SetOutline\0"
    );
    load_fn!(lib, api, (*api).text_draw.set_shadow, "TextDraw_SetShadow\0");
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_background_color,
        "TextDraw_SetBackgroundColor\0"
    );
    load_fn!(lib, api, (*api).text_draw.set_font, "TextDraw_SetFont\0");
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_proportional,
        "TextDraw_SetProportional\0"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_selectable,
        "TextDraw_SetSelectable\0"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.show_for_player,
        "TextDraw_ShowForPlayer\0"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.hide_for_player,
        "TextDraw_HideForPlayer\0"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.show_for_all,
        "TextDraw_ShowForAll\0"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.hide_for_all,
        "TextDraw_HideForAll\0"
    );
    load_fn!(lib, api, (*api).text_draw.set_string, "TextDraw_SetString\0");
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_string_for_player,
        "TextDraw_SetStringForPlayer\0"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_preview_model,
        "TextDraw_SetPreviewModel\0"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_preview_rot,
        "TextDraw_SetPreviewRot\0"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_preview_veh_col,
        "TextDraw_SetPreviewVehCol\0"
    );
    load_fn!(lib, api, (*api).text_draw.get_pos, "TextDraw_GetPos\0");
    load_fn!(lib, api, (*api).text_draw.set_pos, "TextDraw_SetPos\0");

    true
}