use crate::call_api;
use bindings::types::VehiclePtr;

#[repr(transparent)]
pub struct Vehicle {
    pub(crate) ptr: VehiclePtr,
}

impl Vehicle {
    pub(crate) fn from_ptr(ptr: VehiclePtr) -> Option<Self> {
        (!ptr.is_null()).then(|| Self { ptr })
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
        let mut id: i32 = -1;

        call_api!(vehicle.create(
            model,
            x,
            y,
            z,
            rotation,
            color1,
            color2,
            respawn_delay,
            add_siren,
            &mut id
        ))
        .and_then(|ptr| Self::from_ptr(ptr))
    }

    pub fn destroy(&self) -> bool {
        call_api!(vehicle.destroy(self.ptr)).unwrap_or(false)
    }

    pub fn get_id(&self) -> i32 {
        call_api!(vehicle.get_id(self.ptr)).unwrap_or(-1)
    }

    pub fn get_pos(&self) -> (f32, f32, f32) {
        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);

        call_api!(vehicle.get_pos(
            self.ptr,
            &mut x as *mut f32,
            &mut y as *mut f32,
            &mut z as *mut f32
        ));

        (x, y, z)
    }

    pub fn set_pos(&self, x: f32, y: f32, z: f32) -> bool {
        call_api!(vehicle.set_pos(self.ptr, x, y, z)).unwrap_or(false)
    }

    pub fn get_rotation(&self) -> f32 {
        call_api!(vehicle.get_rotation(self.ptr)).unwrap_or(0.0)
    }

    pub fn set_rotation(&self, rotation: f32) -> bool {
        call_api!(vehicle.set_rotation(self.ptr, rotation)).unwrap_or(false)
    }

    pub fn get_health(&self) -> f32 {
        call_api!(vehicle.get_health(self.ptr)).unwrap_or(0.0)
    }

    pub fn set_health(&self, health: f32) -> bool {
        call_api!(vehicle.set_health(self.ptr, health)).unwrap_or(false)
    }

    pub fn get_model(&self) -> i32 {
        call_api!(vehicle.get_model(self.ptr)).unwrap_or(0)
    }

    pub fn get_interior(&self) -> i32 {
        call_api!(vehicle.get_interior(self.ptr)).unwrap_or(0)
    }

    pub fn set_interior(&self, interior: i32) -> bool {
        call_api!(vehicle.set_interior(self.ptr, interior)).unwrap_or(false)
    }

    pub fn get_virtual_world(&self) -> i32 {
        call_api!(vehicle.get_virtual_world(self.ptr)).unwrap_or(0)
    }

    pub fn set_virtual_world(&self, world: i32) -> bool {
        call_api!(vehicle.set_virtual_world(self.ptr, world)).unwrap_or(false)
    }
}
