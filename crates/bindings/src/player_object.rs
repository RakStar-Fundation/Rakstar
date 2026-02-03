#![allow(non_snake_case, non_camel_case_types)]
use crate::types::*;

pub type PlayerObject_Create_t = unsafe extern "C" fn(
    player: PlayerPtr,
    modelid: i32,
    x: f32,
    y: f32,
    z: f32,
    rotationX: f32,
    rotationY: f32,
    rotationZ: f32,
    drawDistance: f32,
    id: *mut i32,
) -> PlayerObjectPtr;

pub type PlayerObject_Destroy_t =
    unsafe extern "C" fn(player: PlayerPtr, object: PlayerObjectPtr) -> bool;
pub type PlayerObject_FromID_t =
    unsafe extern "C" fn(player: PlayerPtr, objectid: i32) -> PlayerObjectPtr;
pub type PlayerObject_GetID_t =
    unsafe extern "C" fn(player: PlayerPtr, object: PlayerObjectPtr) -> i32;

pub type PlayerObject_AttachToVehicle_t = unsafe extern "C" fn(
    player: PlayerPtr,
    object: PlayerObjectPtr,
    vehicle: VehiclePtr,
    offsetX: f32,
    offsetY: f32,
    offsetZ: f32,
    rotationX: f32,
    rotationY: f32,
    rotationZ: f32,
) -> bool;

pub type PlayerObject_AttachToPlayer_t = unsafe extern "C" fn(
    player: PlayerPtr,
    object: PlayerObjectPtr,
    playerAttachedTo: PlayerPtr,
    offsetX: f32,
    offsetY: f32,
    offsetZ: f32,
    rotationX: f32,
    rotationY: f32,
    rotationZ: f32,
) -> bool;

pub type PlayerObject_AttachToObject_t = unsafe extern "C" fn(
    player: PlayerPtr,
    object: PlayerObjectPtr,
    attachedTo: ObjectPtr,
    offsetX: f32,
    offsetY: f32,
    offsetZ: f32,
    rotationX: f32,
    rotationY: f32,
    rotationZ: f32,
    syncRotation: bool,
) -> bool;

pub type PlayerObject_SetPos_t = unsafe extern "C" fn(
    player: PlayerPtr,
    object: PlayerObjectPtr,
    x: f32,
    y: f32,
    z: f32,
) -> bool;
pub type PlayerObject_GetPos_t = unsafe extern "C" fn(
    player: PlayerPtr,
    object: PlayerObjectPtr,
    x: *mut f32,
    y: *mut f32,
    z: *mut f32,
) -> bool;

pub type PlayerObject_SetRot_t = unsafe extern "C" fn(
    player: PlayerPtr,
    object: PlayerObjectPtr,
    rotationX: f32,
    rotationY: f32,
    rotationZ: f32,
) -> bool;
pub type PlayerObject_GetRot_t = unsafe extern "C" fn(
    player: PlayerPtr,
    object: PlayerObjectPtr,
    rotationX: *mut f32,
    rotationY: *mut f32,
    rotationZ: *mut f32,
) -> bool;

pub type PlayerObject_GetModel_t =
    unsafe extern "C" fn(player: PlayerPtr, object: PlayerObjectPtr) -> i32;
pub type PlayerObject_SetNoCameraCollision_t =
    unsafe extern "C" fn(player: PlayerPtr, object: PlayerObjectPtr) -> bool;
pub type PlayerObject_IsValid_t =
    unsafe extern "C" fn(player: PlayerPtr, object: PlayerObjectPtr) -> bool;

pub type PlayerObject_Move_t = unsafe extern "C" fn(
    player: PlayerPtr,
    object: PlayerObjectPtr,
    x: f32,
    y: f32,
    z: f32,
    speed: f32,
    rotationX: f32,
    rotationY: f32,
    rotationZ: f32,
) -> i32;

pub type PlayerObject_Stop_t =
    unsafe extern "C" fn(player: PlayerPtr, object: PlayerObjectPtr) -> bool;
pub type PlayerObject_IsMoving_t =
    unsafe extern "C" fn(player: PlayerPtr, object: PlayerObjectPtr) -> bool;

pub type PlayerObject_BeginEditing_t =
    unsafe extern "C" fn(player: PlayerPtr, object: PlayerObjectPtr) -> bool;

pub type PlayerObject_SetMaterial_t = unsafe extern "C" fn(
    player: PlayerPtr,
    object: PlayerObjectPtr,
    materialIndex: i32,
    modelId: i32,
    textureLibrary: *const u8,
    textureName: *const u8,
    materialColor: u32,
) -> bool;

pub type PlayerObject_SetMaterialText_t = unsafe extern "C" fn(
    player: PlayerPtr,
    object: PlayerObjectPtr,
    text: *const u8,
    materialIndex: i32,
    materialSize: i32,
    fontface: *const u8,
    fontsize: i32,
    bold: bool,
    fontColor: u32,
    backgroundColor: u32,
    textalignment: i32,
) -> bool;

pub type PlayerObject_GetDrawDistance_t =
    unsafe extern "C" fn(player: PlayerPtr, object: PlayerObjectPtr) -> f32;
pub type PlayerObject_GetMoveSpeed_t =
    unsafe extern "C" fn(player: PlayerPtr, object: PlayerObjectPtr) -> f32;

pub type PlayerObject_GetMovingTargetPos_t = unsafe extern "C" fn(
    player: PlayerPtr,
    object: PlayerObjectPtr,
    targetX: *mut f32,
    targetY: *mut f32,
    targetZ: *mut f32,
) -> bool;

pub type PlayerObject_GetMovingTargetRot_t = unsafe extern "C" fn(
    player: PlayerPtr,
    object: PlayerObjectPtr,
    rotationX: *mut f32,
    rotationY: *mut f32,
    rotationZ: *mut f32,
) -> bool;

pub type PlayerObject_GetAttachedData_t = unsafe extern "C" fn(
    player: PlayerPtr,
    object: PlayerObjectPtr,
    parentVehicle: *mut i32,
    parentObject: *mut i32,
    parentPlayer: *mut i32,
) -> bool;

pub type PlayerObject_GetAttachedOffset_t = unsafe extern "C" fn(
    player: PlayerPtr,
    object: PlayerObjectPtr,
    offsetX: *mut f32,
    offsetY: *mut f32,
    offsetZ: *mut f32,
    rotationX: *mut f32,
    rotationY: *mut f32,
    rotationZ: *mut f32,
) -> bool;

pub type PlayerObject_GetSyncRotation_t =
    unsafe extern "C" fn(player: PlayerPtr, object: PlayerObjectPtr) -> bool;
pub type PlayerObject_IsMaterialSlotUsed_t =
    unsafe extern "C" fn(player: PlayerPtr, object: PlayerObjectPtr, materialIndex: i32) -> bool;

pub type PlayerObject_GetMaterial_t = unsafe extern "C" fn(
    player: PlayerPtr,
    object: PlayerObjectPtr,
    materialIndex: i32,
    modelid: *mut i32,
    textureLibrary: *mut CAPIStringView,
    textureName: *mut CAPIStringView,
    materialColor: *mut i32,
) -> bool;

pub type PlayerObject_GetMaterialText_t = unsafe extern "C" fn(
    player: PlayerPtr,
    object: PlayerObjectPtr,
    materialIndex: i32,
    text: *mut CAPIStringView,
    materialSize: *mut i32,
    fontFace: *mut CAPIStringView,
    fontSize: *mut i32,
    bold: *mut bool,
    fontColor: *mut i32,
    backgroundColor: *mut i32,
    textAlignment: *mut i32,
) -> bool;

pub type PlayerObject_IsNoCameraCollision_t =
    unsafe extern "C" fn(player: PlayerPtr, object: PlayerObjectPtr) -> bool;

#[repr(C)]
pub struct PlayerObjectAPI {
    pub create: Option<PlayerObject_Create_t>,
    pub destroy: Option<PlayerObject_Destroy_t>,
    pub from_id: Option<PlayerObject_FromID_t>,
    pub get_id: Option<PlayerObject_GetID_t>,
    pub attach_to_vehicle: Option<PlayerObject_AttachToVehicle_t>,
    pub attach_to_player: Option<PlayerObject_AttachToPlayer_t>,
    pub attach_to_object: Option<PlayerObject_AttachToObject_t>,
    pub set_pos: Option<PlayerObject_SetPos_t>,
    pub get_pos: Option<PlayerObject_GetPos_t>,
    pub set_rot: Option<PlayerObject_SetRot_t>,
    pub get_rot: Option<PlayerObject_GetRot_t>,
    pub get_model: Option<PlayerObject_GetModel_t>,
    pub set_no_camera_collision: Option<PlayerObject_SetNoCameraCollision_t>,
    pub is_valid: Option<PlayerObject_IsValid_t>,
    pub move_: Option<PlayerObject_Move_t>,
    pub stop: Option<PlayerObject_Stop_t>,
    pub is_moving: Option<PlayerObject_IsMoving_t>,
    pub begin_editing: Option<PlayerObject_BeginEditing_t>,
    pub set_material: Option<PlayerObject_SetMaterial_t>,
    pub set_material_text: Option<PlayerObject_SetMaterialText_t>,
    pub get_draw_distance: Option<PlayerObject_GetDrawDistance_t>,
    pub get_move_speed: Option<PlayerObject_GetMoveSpeed_t>,
    pub get_moving_target_pos: Option<PlayerObject_GetMovingTargetPos_t>,
    pub get_moving_target_rot: Option<PlayerObject_GetMovingTargetRot_t>,
    pub get_attached_data: Option<PlayerObject_GetAttachedData_t>,
    pub get_attached_offset: Option<PlayerObject_GetAttachedOffset_t>,
    pub get_sync_rotation: Option<PlayerObject_GetSyncRotation_t>,
    pub is_material_slot_used: Option<PlayerObject_IsMaterialSlotUsed_t>,
    pub get_material: Option<PlayerObject_GetMaterial_t>,
    pub get_material_text: Option<PlayerObject_GetMaterialText_t>,
    pub is_no_camera_collision: Option<PlayerObject_IsNoCameraCollision_t>,
}

impl Default for PlayerObjectAPI {
    fn default() -> Self {
        Self {
            create: None,
            destroy: None,
            from_id: None,
            get_id: None,
            attach_to_vehicle: None,
            attach_to_player: None,
            attach_to_object: None,
            set_pos: None,
            get_pos: None,
            set_rot: None,
            get_rot: None,
            get_model: None,
            set_no_camera_collision: None,
            is_valid: None,
            move_: None,
            stop: None,
            is_moving: None,
            begin_editing: None,
            set_material: None,
            set_material_text: None,
            get_draw_distance: None,
            get_move_speed: None,
            get_moving_target_pos: None,
            get_moving_target_rot: None,
            get_attached_data: None,
            get_attached_offset: None,
            get_sync_rotation: None,
            is_material_slot_used: None,
            get_material: None,
            get_material_text: None,
            is_no_camera_collision: None,
        }
    }
}
