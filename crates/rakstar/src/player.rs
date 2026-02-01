use bindings::types::{PlayerPtr, CAPIStringView};
use std::mem::MaybeUninit;
use crate::call_api;

pub struct Player {
    ptr: PlayerPtr,
}

impl Player {
    pub fn from_ptr(ptr: PlayerPtr) -> Self {
        Self { ptr }
    }

    pub fn as_ptr(&self) -> PlayerPtr {
        self.ptr
    }

    pub fn from_id(player_id: i32) -> Option<Self> {
        let ptr = call_api!(player.from_id => player_id);
        if ptr.is_null() {
            return None;
        }
        Some(Self::from_ptr(ptr))
    }

    pub fn get_id(&self) -> Option<i32> {
        unsafe {
            let api = crate::macros::get_api()?;
            let Some(func) = api.player.get_id else {
                return None;
            };
            Some(func(self.ptr))
        }
    }

    pub fn get_name(&self) -> Option<String> {
        let mut name_view = MaybeUninit::<CAPIStringView>::uninit();
        unsafe {
            call_api!(player.get_name => self.ptr, name_view.as_mut_ptr());
            let name_view = name_view.assume_init();
            name_view.to_string()
        }
    }

    pub fn kick(&self) -> bool {
        unsafe {
            let api = crate::macros::get_api().unwrap();
            let Some(func) = api.player.kick else {
                return false;
            };
            func(self.ptr)
        }
    }

    pub fn ban(&self) -> bool {
        unsafe {
            let api = crate::macros::get_api().unwrap();
            let Some(func) = api.player.ban else {
                return false;
            };
            func(self.ptr)
        }
    }

    pub fn spawn(&self) -> bool {
        unsafe {
            let api = crate::macros::get_api().unwrap();
            let Some(func) = api.player.spawn else {
                return false;
            };
            func(self.ptr)
        }
    }

    pub fn get_health(&self) -> Option<f32> {
        let mut health = 0.0f32;
        unsafe {
            let api = crate::macros::get_api()?;
            let Some(func) = api.player.get_health else {
                return None;
            };
            func(self.ptr, &mut health as *mut f32);
        }
        Some(health)
    }

    pub fn set_health(&self, health: f32) -> bool {
        unsafe {
            let api = crate::macros::get_api().unwrap();
            let Some(func) = api.player.set_health else {
                return false;
            };
            func(self.ptr, health)
        }
    }

    pub fn get_armour(&self) -> Option<f32> {
        let mut armour = 0.0f32;
        unsafe {
            let api = crate::macros::get_api()?;
            let Some(func) = api.player.get_armour else {
                return None;
            };
            func(self.ptr, &mut armour as *mut f32);
        }
        Some(armour)
    }

    pub fn set_armour(&self, armour: f32) -> bool {
        unsafe {
            let api = crate::macros::get_api().unwrap();
            let Some(func) = api.player.set_armour else {
                return false;
            };
            func(self.ptr, armour)
        }
    }

    pub fn get_pos(&self) -> Option<(f32, f32, f32)> {
        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
        unsafe {
            let api = crate::macros::get_api()?;
            let Some(func) = api.player.get_pos else {
                return None;
            };
            func(self.ptr, &mut x as *mut f32, &mut y as *mut f32, &mut z as *mut f32);
        }
        Some((x, y, z))
    }

    pub fn set_pos(&self, x: f32, y: f32, z: f32) -> bool {
        unsafe {
            let api = crate::macros::get_api().unwrap();
            let Some(func) = api.player.set_pos else {
                return false;
            };
            func(self.ptr, x, y, z)
        }
    }

    pub fn get_interior(&self) -> Option<i32> {
        unsafe {
            let api = crate::macros::get_api()?;
            let Some(func) = api.player.get_interior else {
                return None;
            };
            Some(func(self.ptr))
        }
    }

    pub fn set_interior(&self, interior: i32) -> bool {
        unsafe {
            let api = crate::macros::get_api().unwrap();
            let Some(func) = api.player.set_interior else {
                return false;
            };
            func(self.ptr, interior)
        }
    }

    pub fn get_virtual_world(&self) -> Option<i32> {
        unsafe {
            let api = crate::macros::get_api()?;
            let Some(func) = api.player.get_virtual_world else {
                return None;
            };
            Some(func(self.ptr))
        }
    }

    pub fn set_virtual_world(&self, world: i32) -> bool {
        unsafe {
            let api = crate::macros::get_api().unwrap();
            let Some(func) = api.player.set_virtual_world else {
                return false;
            };
            func(self.ptr, world)
        }
    }

    pub fn reset_weapons(&self) -> bool {
        unsafe {
            let api = crate::macros::get_api().unwrap();
            let Some(func) = api.player.reset_weapons else {
                return false;
            };
            func(self.ptr)
        }
    }

    pub fn give_weapon(&self, weapon: i32, ammo: i32) -> bool {
        unsafe {
            let api = crate::macros::get_api().unwrap();
            let Some(func) = api.player.give_weapon else {
                return false;
            };
            func(self.ptr, weapon, ammo)
        }
    }
}
