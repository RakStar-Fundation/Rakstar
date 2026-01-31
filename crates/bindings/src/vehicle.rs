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

pub type Vehicle_SetPos_t = unsafe extern "C" fn(
    vehicle: VehiclePtr,
    x: f32,
    y: f32,
    z: f32,
) -> bool;

pub type Vehicle_GetPos_t = unsafe extern "C" fn(
    vehicle: VehiclePtr,
    x: *mut f32,
    y: *mut f32,
    z: *mut f32,
) -> bool;

pub type Vehicle_SetHealth_t = unsafe extern "C" fn(vehicle: VehiclePtr, health: f32) -> bool;
pub type Vehicle_GetHealth_t = unsafe extern "C" fn(vehicle: VehiclePtr) -> f32;

pub struct VehicleAPI {
    pub create: Option<Vehicle_Create_t>,
    pub destroy: Option<Vehicle_Destroy_t>,
    pub from_id: Option<Vehicle_FromID_t>,
    pub get_id: Option<Vehicle_GetID_t>,
    pub set_pos: Option<Vehicle_SetPos_t>,
    pub get_pos: Option<Vehicle_GetPos_t>,
    pub set_health: Option<Vehicle_SetHealth_t>,
    pub get_health: Option<Vehicle_GetHealth_t>,
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
            set_health: None,
            get_health: None,
        }
    }
}
