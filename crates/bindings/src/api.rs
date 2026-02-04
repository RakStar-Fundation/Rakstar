use super::types::ComponentVersion;
use crate::npc::NPCAPI;
use crate::textdraw::TextDrawAPI;
use crate::vehicle::VehicleAPI;
use crate::{core::CoreAPI, player::PlayerAPI, player_object::PlayerObjectAPI};
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

    load_fn!(lib, api, (*api).core.max_players, "Core_MaxPlayers");

    load_fn!(lib, api, (*api).player.get_name, "Player_GetName");
    load_fn!(lib, api, (*api).player.from_id, "Player_FromID");
    load_fn!(lib, api, (*api).player.get_id, "Player_GetID");
    load_fn!(lib, api, (*api).player.kick, "Player_Kick");
    load_fn!(lib, api, (*api).player.ban, "Player_Ban");
    load_fn!(lib, api, (*api).player.spawn, "Player_Spawn");
    load_fn!(lib, api, (*api).player.get_health, "Player_GetHealth");
    load_fn!(lib, api, (*api).player.set_health, "Player_SetHealth");
    load_fn!(lib, api, (*api).player.get_armor, "Player_GetArmour");
    load_fn!(lib, api, (*api).player.set_armor, "Player_SetArmour");
    load_fn!(lib, api, (*api).player.get_pos, "Player_GetPos");
    load_fn!(lib, api, (*api).player.set_pos, "Player_SetPos");
    load_fn!(lib, api, (*api).player.get_interior, "Player_GetInterior");
    load_fn!(lib, api, (*api).player.set_interior, "Player_SetInterior");
    load_fn!(
        lib,
        api,
        (*api).player.get_virtual_world,
        "Player_GetVirtualWorld"
    );
    load_fn!(
        lib,
        api,
        (*api).player.set_virtual_world,
        "Player_SetVirtualWorld"
    );
    load_fn!(lib, api, (*api).player.reset_weapons, "Player_ResetWeapons");
    load_fn!(lib, api, (*api).player.give_weapon, "Player_GiveWeapon");
    load_fn!(
        lib,
        api,
        (*api).player.put_in_vehicle,
        "Player_PutInVehicle"
    );
    load_fn!(
        lib,
        api,
        (*api).player.remove_from_vehicle,
        "Player_RemoveFromVehicle"
    );
    load_fn!(lib, api, (*api).player.is_in_vehicle, "Player_IsInVehicle");
    load_fn!(
        lib,
        api,
        (*api).player.is_in_any_vehicle,
        "Player_IsInAnyVehicle"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_vehicle_id,
        "Player_GetVehicleID"
    );
    load_fn!(
        lib,
        api,
        (*api).player.get_facing_angle,
        "Player_GetFacingAngle"
    );
    load_fn!(
        lib,
        api,
        (*api).player.set_facing_angle,
        "Player_SetFacingAngle"
    );
    load_fn!(lib, api, (*api).player.is_spawned, "Player_IsSpawned");

    load_fn!(lib, api, (*api).vehicle.create, "Vehicle_Create");
    load_fn!(lib, api, (*api).vehicle.destroy, "Vehicle_Destroy");
    load_fn!(lib, api, (*api).vehicle.get_id, "Vehicle_GetID");
    load_fn!(lib, api, (*api).vehicle.get_pos, "Vehicle_GetPos");
    load_fn!(lib, api, (*api).vehicle.set_pos, "Vehicle_SetPos");
    load_fn!(lib, api, (*api).vehicle.get_rotation, "Vehicle_GetZAngle");
    load_fn!(lib, api, (*api).vehicle.set_rotation, "Vehicle_SetZAngle");
    load_fn!(lib, api, (*api).vehicle.get_health, "Vehicle_GetHealth");
    load_fn!(lib, api, (*api).vehicle.set_health, "Vehicle_SetHealth");
    load_fn!(lib, api, (*api).vehicle.get_model, "Vehicle_GetModel");
    load_fn!(lib, api, (*api).vehicle.get_interior, "Vehicle_GetInterior");
    load_fn!(lib, api, (*api).vehicle.set_interior, "Vehicle_SetInterior");
    load_fn!(
        lib,
        api,
        (*api).vehicle.get_virtual_world,
        "Vehicle_GetVirtualWorld"
    );
    load_fn!(
        lib,
        api,
        (*api).vehicle.set_virtual_world,
        "Vehicle_SetVirtualWorld"
    );

    load_fn!(lib, api, (*api).dialog.show, "Dialog_Show");
    load_fn!(lib, api, (*api).dialog.hide, "Dialog_Hide");

    load_fn!(lib, api, (*api).player_object.create, "PlayerObject_Create");
    load_fn!(
        lib,
        api,
        (*api).player_object.destroy,
        "PlayerObject_Destroy"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.from_id,
        "PlayerObject_FromID"
    );
    load_fn!(lib, api, (*api).player_object.get_id, "PlayerObject_GetID");
    load_fn!(
        lib,
        api,
        (*api).player_object.attach_to_vehicle,
        "PlayerObject_AttachToVehicle"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.attach_to_player,
        "PlayerObject_AttachToPlayer"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.attach_to_object,
        "PlayerObject_AttachToObject"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.set_pos,
        "PlayerObject_SetPos"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_pos,
        "PlayerObject_GetPos"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.set_rot,
        "PlayerObject_SetRot"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_rot,
        "PlayerObject_GetRot"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_model,
        "PlayerObject_GetModel"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.set_no_camera_collision,
        "PlayerObject_SetNoCameraCollision"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.is_valid,
        "PlayerObject_IsValid"
    );
    load_fn!(lib, api, (*api).player_object.move_, "PlayerObject_Move");
    load_fn!(lib, api, (*api).player_object.stop, "PlayerObject_Stop");
    load_fn!(
        lib,
        api,
        (*api).player_object.is_moving,
        "PlayerObject_IsMoving"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.begin_editing,
        "PlayerObject_BeginEditing"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.set_material,
        "PlayerObject_SetMaterial"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.set_material_text,
        "PlayerObject_SetMaterialText"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_draw_distance,
        "PlayerObject_GetDrawDistance"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_move_speed,
        "PlayerObject_GetMoveSpeed"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_moving_target_pos,
        "PlayerObject_GetMovingTargetPos"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_moving_target_rot,
        "PlayerObject_GetMovingTargetRot"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_attached_data,
        "PlayerObject_GetAttachedData"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_attached_offset,
        "PlayerObject_GetAttachedOffset"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_sync_rotation,
        "PlayerObject_GetSyncRotation"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.is_material_slot_used,
        "PlayerObject_IsMaterialSlotUsed"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_material,
        "PlayerObject_GetMaterial"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.get_material_text,
        "PlayerObject_GetMaterialText"
    );
    load_fn!(
        lib,
        api,
        (*api).player_object.is_no_camera_collision,
        "PlayerObject_IsNoCameraCollision"
    );

    load_fn!(lib, api, (*api).event.add_handler, "Event_AddHandler");

    load_fn!(lib, api, (*api).event.add_handler, "Event_AddHandler");

    load_fn!(lib, api, (*api).checkpoint.set, "Checkpoint_Set");
    load_fn!(lib, api, (*api).checkpoint.disable, "Checkpoint_Disable");
    load_fn!(
        lib,
        api,
        (*api).checkpoint.is_player_in,
        "Checkpoint_IsPlayerIn"
    );
    load_fn!(lib, api, (*api).checkpoint.is_active, "Checkpoint_IsActive");
    load_fn!(lib, api, (*api).checkpoint.get, "Checkpoint_Get");

    load_fn!(lib, api, (*api).checkpoint.race_set, "RaceCheckpoint_Set");
    load_fn!(
        lib,
        api,
        (*api).checkpoint.race_disable,
        "RaceCheckpoint_Disable"
    );
    load_fn!(
        lib,
        api,
        (*api).checkpoint.race_is_player_in,
        "RaceCheckpoint_IsPlayerIn"
    );
    load_fn!(
        lib,
        api,
        (*api).checkpoint.race_is_active,
        "RaceCheckpoint_IsActive"
    );
    load_fn!(lib, api, (*api).checkpoint.race_get, "RaceCheckpoint_Get");

    // NPC functions
    load_fn!(lib, api, (*api).npc.connect, "NPC_Connect");
    load_fn!(lib, api, (*api).npc.create, "NPC_Create");
    load_fn!(lib, api, (*api).npc.destroy, "NPC_Destroy");
    load_fn!(lib, api, (*api).npc.from_id, "NPC_FromID");
    load_fn!(lib, api, (*api).npc.get_id, "NPC_GetID");
    load_fn!(lib, api, (*api).npc.is_valid, "NPC_IsValid");
    load_fn!(lib, api, (*api).npc.get_player, "NPC_GetPlayer");
    load_fn!(lib, api, (*api).npc.spawn, "NPC_Spawn");
    load_fn!(lib, api, (*api).npc.respawn, "NPC_Respawn");
    load_fn!(lib, api, (*api).npc.set_pos, "NPC_SetPos");
    load_fn!(lib, api, (*api).npc.get_pos, "NPC_GetPos");
    load_fn!(lib, api, (*api).npc.set_rot, "NPC_SetRot");
    load_fn!(lib, api, (*api).npc.get_rot, "NPC_GetRot");
    load_fn!(lib, api, (*api).npc.set_facing_angle, "NPC_SetFacingAngle");
    load_fn!(lib, api, (*api).npc.get_facing_angle, "NPC_GetFacingAngle");
    load_fn!(
        lib,
        api,
        (*api).npc.set_virtual_world,
        "NPC_SetVirtualWorld"
    );
    load_fn!(
        lib,
        api,
        (*api).npc.get_virtual_world,
        "NPC_GetVirtualWorld"
    );
    load_fn!(lib, api, (*api).npc.set_interior, "NPC_SetInterior");
    load_fn!(lib, api, (*api).npc.get_interior, "NPC_GetInterior");
    load_fn!(lib, api, (*api).npc.move_, "NPC_Move");
    load_fn!(lib, api, (*api).npc.move_to_player, "NPC_MoveToPlayer");
    load_fn!(lib, api, (*api).npc.stop_move, "NPC_StopMove");
    load_fn!(lib, api, (*api).npc.is_moving, "NPC_IsMoving");
    load_fn!(lib, api, (*api).npc.set_skin, "NPC_SetSkin");
    load_fn!(lib, api, (*api).npc.is_streamed_in, "NPC_IsStreamedIn");
    load_fn!(
        lib,
        api,
        (*api).npc.is_any_streamed_in,
        "NPC_IsAnyStreamedIn"
    );
    load_fn!(lib, api, (*api).npc.set_health, "NPC_SetHealth");
    load_fn!(lib, api, (*api).npc.get_health, "NPC_GetHealth");
    load_fn!(lib, api, (*api).npc.set_armour, "NPC_SetArmour");
    load_fn!(lib, api, (*api).npc.get_armour, "NPC_GetArmour");
    load_fn!(lib, api, (*api).npc.is_dead, "NPC_IsDead");
    load_fn!(lib, api, (*api).npc.set_invulnerable, "NPC_SetInvulnerable");
    load_fn!(lib, api, (*api).npc.is_invulnerable, "NPC_IsInvulnerable");
    load_fn!(lib, api, (*api).npc.set_weapon, "NPC_SetWeapon");
    load_fn!(lib, api, (*api).npc.get_weapon, "NPC_GetWeapon");
    load_fn!(lib, api, (*api).npc.set_ammo, "NPC_SetAmmo");
    load_fn!(lib, api, (*api).npc.get_ammo, "NPC_GetAmmo");
    load_fn!(lib, api, (*api).npc.set_ammo_in_clip, "NPC_SetAmmoInClip");
    load_fn!(lib, api, (*api).npc.get_ammo_in_clip, "NPC_GetAmmoInClip");
    load_fn!(lib, api, (*api).npc.enable_reloading, "NPC_EnableReloading");
    load_fn!(
        lib,
        api,
        (*api).npc.is_reload_enabled,
        "NPC_IsReloadEnabled"
    );
    load_fn!(lib, api, (*api).npc.is_reloading, "NPC_IsReloading");
    load_fn!(
        lib,
        api,
        (*api).npc.enable_infinite_ammo,
        "NPC_EnableInfiniteAmmo"
    );
    load_fn!(
        lib,
        api,
        (*api).npc.is_infinite_ammo_enabled,
        "NPC_IsInfiniteAmmoEnabled"
    );
    load_fn!(lib, api, (*api).npc.get_weapon_state, "NPC_GetWeaponState");
    load_fn!(lib, api, (*api).npc.set_keys, "NPC_SetKeys");
    load_fn!(lib, api, (*api).npc.get_keys, "NPC_GetKeys");
    load_fn!(
        lib,
        api,
        (*api).npc.set_weapon_skill_level,
        "NPC_SetWeaponSkillLevel"
    );
    load_fn!(
        lib,
        api,
        (*api).npc.get_weapon_skill_level,
        "NPC_GetWeaponSkillLevel"
    );

    // TextDraw functions
    load_fn!(lib, api, (*api).text_draw.create, "TextDraw_Create");
    load_fn!(lib, api, (*api).text_draw.destroy, "TextDraw_Destroy");
    load_fn!(lib, api, (*api).text_draw.from_id, "TextDraw_FromID");
    load_fn!(lib, api, (*api).text_draw.get_id, "TextDraw_GetID");
    load_fn!(lib, api, (*api).text_draw.is_valid, "TextDraw_IsValid");
    load_fn!(
        lib,
        api,
        (*api).text_draw.is_visible_for_player,
        "TextDraw_IsVisibleForPlayer"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_letter_size,
        "TextDraw_SetLetterSize"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_text_size,
        "TextDraw_SetTextSize"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_alignment,
        "TextDraw_SetAlignment"
    );
    load_fn!(lib, api, (*api).text_draw.set_color, "TextDraw_SetColor");
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_box_color,
        "TextDraw_SetBoxColor"
    );
    load_fn!(lib, api, (*api).text_draw.set_use_box, "TextDraw_SetUseBox");
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_outline,
        "TextDraw_SetOutline"
    );
    load_fn!(lib, api, (*api).text_draw.set_shadow, "TextDraw_SetShadow");
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_background_color,
        "TextDraw_SetBackgroundColor"
    );
    load_fn!(lib, api, (*api).text_draw.set_font, "TextDraw_SetFont");
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_proportional,
        "TextDraw_SetProportional"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_selectable,
        "TextDraw_SetSelectable"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.show_for_player,
        "TextDraw_ShowForPlayer"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.hide_for_player,
        "TextDraw_HideForPlayer"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.show_for_all,
        "TextDraw_ShowForAll"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.hide_for_all,
        "TextDraw_HideForAll"
    );
    load_fn!(lib, api, (*api).text_draw.set_string, "TextDraw_SetString");
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_string_for_player,
        "TextDraw_SetStringForPlayer"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_preview_model,
        "TextDraw_SetPreviewModel"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_preview_rot,
        "TextDraw_SetPreviewRot"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.set_preview_veh_col,
        "TextDraw_SetPreviewVehCol"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.hide_for_player,
        "TextDraw_HideForPlayer"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.show_for_all,
        "TextDraw_ShowForAll"
    );
    load_fn!(
        lib,
        api,
        (*api).text_draw.hide_for_all,
        "TextDraw_HideForAll"
    );
    load_fn!(lib, api, (*api).text_draw.set_string, "TextDraw_SetString");
    load_fn!(lib, api, (*api).text_draw.get_pos, "TextDraw_GetPos");
    load_fn!(lib, api, (*api).text_draw.set_pos, "TextDraw_SetPos");

    true
}

#[cfg(windows)]
pub unsafe fn initialize_capi(api: *mut OmpApi) -> bool {
    false
}
