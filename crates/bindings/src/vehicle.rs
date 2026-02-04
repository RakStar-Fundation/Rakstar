#![allow(non_snake_case, non_camel_case_types)]
use crate::types::*;

pub type Vehicle_Create_t = unsafe extern "C" fn(
    modelid: i32,
    x: f32,
    y: f32,
    z: f32,
    rotation: f32,
    color1: i32,
    color2: i32,
    respawn_delay: i32,
    add_siren: bool,
    id: *mut i32,
) -> VehiclePtr;

pub type Vehicle_Destroy_t = unsafe extern "C" fn(vehicle: VehiclePtr) -> bool;
pub type Vehicle_FromID_t = unsafe extern "C" fn(vehicleid: i32) -> VehiclePtr;
pub type Vehicle_GetID_t = unsafe extern "C" fn(vehicle: VehiclePtr) -> i32;

pub type Vehicle_SetPos_t =
    unsafe extern "C" fn(vehicle: VehiclePtr, x: f32, y: f32, z: f32) -> bool;

pub type Vehicle_GetPos_t =
    unsafe extern "C" fn(vehicle: VehiclePtr, x: *mut f32, y: *mut f32, z: *mut f32) -> bool;

pub type Vehicle_SetHealth_t = unsafe extern "C" fn(vehicle: VehiclePtr, health: f32) -> bool;
pub type Vehicle_GetHealth_t = unsafe extern "C" fn(vehicle: VehiclePtr) -> f32;

pub type VehicleGetRotationFn = unsafe extern "C" fn(vehicle: VehiclePtr) -> f32;
pub type VehicleSetRotationFn = unsafe extern "C" fn(vehicle: VehiclePtr, rotation: f32) -> bool;

pub type VehicleGetModelFn = unsafe extern "C" fn(vehicle: VehiclePtr) -> i32;
pub type VehicleGetInteriorFn = unsafe extern "C" fn(vehicle: VehiclePtr) -> i32;
pub type VehicleSetInteriorFn = unsafe extern "C" fn(vehicle: VehiclePtr, interior: i32) -> bool;
pub type VehicleGetVirtualWorldFn = unsafe extern "C" fn(vehicle: VehiclePtr) -> i32;
pub type VehicleSetVirtualWorldFn = unsafe extern "C" fn(vehicle: VehiclePtr, world: i32) -> bool;

#[repr(C)]
pub struct VehicleAPI {
    pub create: Option<Vehicle_Create_t>,
    pub destroy: Option<Vehicle_Destroy_t>,
    pub from_id: Option<Vehicle_FromID_t>,
    pub get_id: Option<Vehicle_GetID_t>,
    pub set_pos: Option<Vehicle_SetPos_t>,
    pub get_pos: Option<Vehicle_GetPos_t>,
    pub get_rotation: Option<VehicleGetRotationFn>,
    pub set_rotation: Option<VehicleSetRotationFn>,
    pub set_health: Option<Vehicle_SetHealth_t>,
    pub get_health: Option<Vehicle_GetHealth_t>,
    pub get_model: Option<VehicleGetModelFn>,
    pub get_interior: Option<VehicleGetInteriorFn>,
    pub set_interior: Option<VehicleSetInteriorFn>,
    pub get_virtual_world: Option<VehicleGetVirtualWorldFn>,
    pub set_virtual_world: Option<VehicleSetVirtualWorldFn>,
}

impl Default for VehicleAPI {
    fn default() -> Self {
        Self {
            create: None,
            destroy: None,
            from_id: None,
            get_id: None,
            set_pos: None,
            get_pos: None,
            get_rotation: None,
            set_rotation: None,
            set_health: None,
            get_health: None,
            get_model: None,
            get_interior: None,
            set_interior: None,
            get_virtual_world: None,
            set_virtual_world: None,
        }
    }
}
