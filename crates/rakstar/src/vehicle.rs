use crate::call_api;
use bindings::{CAPIStringView, VehiclePtr};
use std::mem::MaybeUninit;

#[repr(transparent)]
pub struct Vehicle {
    pub(crate) ptr: VehiclePtr,
}

impl Vehicle {
    pub(crate) fn from_ptr(ptr: VehiclePtr) -> Self {
        Self { ptr }
    }

    pub fn create(
        model: i32,
        x: f32,
        y: f32,
        z: f32,
        rotation: f32,
        color1: i32,
        color2: i32,
        respawn_delay: i32,
        add_siren: bool,
    ) -> Option<Self> {
        unsafe {
            let api = crate::macros::get_api()?;
            let Some(func) = api.vehicle.create else {
                return None;
            };
            let mut id = 0i32;
            let ptr = func(
                model,
                x,
                y,
                z,
                rotation,
                color1,
                color2,
                respawn_delay,
                add_siren,
                &mut id as *mut i32,
            );
            if ptr.is_null() {
                return None;
            }
            Some(Self::from_ptr(ptr))
        }
    }

    pub fn destroy(&self) -> bool {
        call_api!(vehicle.destroy => self.ptr; or false)
    }

    pub fn get_id(&self) -> i32 {
        call_api!(vehicle.get_id => self.ptr; or -1)
    }

    pub fn get_pos(&self) -> (f32, f32, f32) {
        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
        call_api!(vehicle.get_pos => self.ptr, &mut x as *mut f32, &mut y as *mut f32, &mut z as *mut f32; or (0.0, 0.0, 0.0));
        (x, y, z)
    }

    pub fn set_pos(&self, x: f32, y: f32, z: f32) -> bool {
        call_api!(vehicle.set_pos => self.ptr, x, y, z; or false)
    }

    pub fn get_rotation(&self) -> f32 {
        let mut rotation = 0.0f32;
        call_api!(vehicle.get_rotation => self.ptr, &mut rotation as *mut f32; or 0.0);
        rotation
    }

    pub fn set_rotation(&self, rotation: f32) -> bool {
        call_api!(vehicle.set_rotation => self.ptr, rotation; or false)
    }

    pub fn get_health(&self) -> f32 {
        call_api!(vehicle.get_health => self.ptr; or 0.0)
    }

    pub fn set_health(&self, health: f32) -> bool {
        call_api!(vehicle.set_health => self.ptr, health; or false)
    }

    pub fn get_model(&self) -> i32 {
        call_api!(vehicle.get_model => self.ptr; or 0)
    }

    pub fn get_interior(&self) -> i32 {
        call_api!(vehicle.get_interior => self.ptr; or 0)
    }

    pub fn set_interior(&self, interior: i32) -> bool {
        call_api!(vehicle.set_interior => self.ptr, interior; or false)
    }

    pub fn get_virtual_world(&self) -> i32 {
        call_api!(vehicle.get_virtual_world => self.ptr; or 0)
    }

    pub fn set_virtual_world(&self, world: i32) -> bool {
        call_api!(vehicle.set_virtual_world => self.ptr, world; or false)
    }
}
