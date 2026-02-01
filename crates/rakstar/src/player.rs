use crate::call_api;
use bindings::types::{CAPIStringView, PlayerPtr};
use std::mem::MaybeUninit;

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
        unsafe {
            let api = crate::macros::get_api()?;
            let Some(func) = api.player.from_id else {
                return None;
            };
            let ptr = func(player_id);
            if ptr.is_null() {
                return None;
            }
            Some(Self::from_ptr(ptr))
        }
    }

    pub fn get_id(&self) -> i32 {
        call_api!(player.get_id => self.ptr; or -1)
    }

    pub fn get_name(&self) -> Option<String> {
        let mut name_view = MaybeUninit::<CAPIStringView>::uninit();
        call_api!(player.get_name => self.ptr, name_view.as_mut_ptr(); or return None);
        unsafe {
            let name_view = name_view.assume_init();
            name_view.to_string()
        }
    }

    pub fn kick(&self) -> bool {
        call_api!(player.kick => self.ptr; or false)
    }

    pub fn ban(&self) -> bool {
        call_api!(player.ban => self.ptr; or false)
    }

    pub fn spawn(&self) -> bool {
        call_api!(player.spawn => self.ptr; or false)
    }

    pub fn get_health(&self) -> f32 {
        let mut health = 0.0f32;
        call_api!(player.get_health => self.ptr, &mut health as *mut f32; or return 0.0);
        health
    }

    pub fn set_health(&self, health: f32) -> bool {
        call_api!(player.set_health => self.ptr, health; or false)
    }

    pub fn get_armour(&self) -> f32 {
        let mut armour = 0.0f32;
        call_api!(player.get_armour => self.ptr, &mut armour as *mut f32; or return 0.0);
        armour
    }

    pub fn set_armour(&self, armour: f32) -> bool {
        call_api!(player.set_armour => self.ptr, armour; or false)
    }

    pub fn get_pos(&self) -> (f32, f32, f32) {
        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
        call_api!(player.get_pos => self.ptr, &mut x as *mut f32, &mut y as *mut f32, &mut z as *mut f32; or return (0.0, 0.0, 0.0));
        (x, y, z)
    }

    pub fn set_pos(&self, x: f32, y: f32, z: f32) -> bool {
        call_api!(player.set_pos => self.ptr, x, y, z; or false)
    }

    pub fn get_interior(&self) -> i32 {
        call_api!(player.get_interior => self.ptr; or 0)
    }

    pub fn set_interior(&self, interior: i32) -> bool {
        call_api!(player.set_interior => self.ptr, interior; or false)
    }

    pub fn get_virtual_world(&self) -> i32 {
        call_api!(player.get_virtual_world => self.ptr; or 0)
    }

    pub fn set_virtual_world(&self, world: i32) -> bool {
        call_api!(player.set_virtual_world => self.ptr, world; or false)
    }

    pub fn reset_weapons(&self) -> bool {
        call_api!(player.reset_weapons => self.ptr; or false)
    }

    pub fn give_weapon(&self, weapon: i32, ammo: i32) -> bool {
        call_api!(player.give_weapon => self.ptr, weapon, ammo; or false)
    }
}
