use crate::call_api;
use bindings::types::{CAPIStringView, PlayerPtr};
use std::mem::MaybeUninit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        let ptr = call_api!(player.from_id => player_id; or None);
        if ptr.is_null() {
            return None;
        }
        Some(Self::from_ptr(ptr))
    }

    pub fn get_id(&self) -> i32 {
        call_api!(player.get_id => self.ptr; or -1)
    }

    pub fn get_name(&self) -> String {
        let mut name_view = MaybeUninit::<CAPIStringView>::uninit();
        call_api!(player.get_name => self.ptr, name_view.as_mut_ptr(); or String::new());
        unsafe {
            let name_view = name_view.assume_init();
            name_view.to_string().unwrap()
        }
    }

    pub fn send_client_message(&self, color: u32, text: &str) -> bool {
        let c_text = std::ffi::CString::new(text).unwrap();
        call_api!(player.send_client_message => self.ptr, color, c_text.as_ptr() as *const u8; or false)
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
        call_api!(player.get_health => self.ptr; or 0.0)
    }

    pub fn set_health(&self, health: f32) -> bool {
        call_api!(player.set_health => self.ptr, health; or false)
    }

    pub fn get_armour(&self) -> f32 {
        call_api!(player.get_armor => self.ptr; or 0.0)
    }

    pub fn set_armour(&self, armour: f32) -> bool {
        call_api!(player.set_armor => self.ptr, armour; or false)
    }

    pub fn get_pos(&self) -> (f32, f32, f32) {
        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
        call_api!(player.get_pos => self.ptr, &mut x as *mut f32, &mut y as *mut f32, &mut z as *mut f32; or (0.0, 0.0, 0.0));
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

    pub fn put_in_vehicle(&self, vehicle: &crate::Vehicle, seat: i32) -> bool {
        call_api!(player.put_in_vehicle => self.ptr, vehicle.ptr, seat; or false)
    }

    pub fn remove_from_vehicle(&self, force: bool) -> bool {
        call_api!(player.remove_from_vehicle => self.ptr, force; or false)
    }

    pub fn is_in_vehicle(&self, vehicle: &crate::Vehicle) -> bool {
        call_api!(player.is_in_vehicle => self.ptr, vehicle.ptr; or false)
    }

    pub fn is_in_any_vehicle(&self) -> bool {
        call_api!(player.is_in_any_vehicle => self.ptr; or false)
    }

    pub fn is_spawned(&self) -> bool {
        call_api!(player.is_spawned => self.ptr; or false)
    }

    pub fn get_vehicle_id(&self) -> i32 {
        call_api!(player.get_vehicle_id => self.ptr; or -1)
    }

    pub fn get_facing_angle(&self) -> f32 {
        call_api!(player.get_facing_angle => self.ptr; or 0.0)
    }

    pub fn set_facing_angle(&self, angle: f32) -> bool {
        call_api!(player.set_facing_angle => self.ptr, angle; or false)
    }

    pub fn show_dialog(
        &self,
        dialog_id: i32,
        style: DialogStyle,
        title: &str,
        body: &str,
        button1: &str,
        button2: &str,
    ) -> bool {
        let c_title = std::ffi::CString::new(title).unwrap();
        let c_body = std::ffi::CString::new(body).unwrap();
        let c_button1 = std::ffi::CString::new(button1).unwrap();
        let c_button2 = std::ffi::CString::new(button2).unwrap();

        unsafe {
            if let Some(api) = crate::macros::get_api() {
                if let Some(show) = api.dialog.show {
                    return show(
                        self.ptr,
                        dialog_id,
                        style as i32,
                        c_title.as_ptr(),
                        c_body.as_ptr(),
                        c_button1.as_ptr(),
                        c_button2.as_ptr(),
                    );
                }
            }
        }
        false
    }

    pub fn set_checkpoint(&self, x: f32, y: f32, z: f32, radius: f32) -> bool {
        call_api!(checkpoint.set => self.ptr, x, y, z, radius; or false)
    }

    pub fn disable_checkpoint(&self) -> bool {
        call_api!(checkpoint.disable => self.ptr; or false)
    }

    pub fn is_in_checkpoint(&self) -> bool {
        call_api!(checkpoint.is_player_in => self.ptr; or false)
    }

    pub fn is_checkpoint_active(&self) -> bool {
        call_api!(checkpoint.is_active => self.ptr; or false)
    }

    pub fn get_checkpoint(&self) -> Option<(f32, f32, f32, f32)> {
        let (mut x, mut y, mut z, mut radius) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
        let success = (|| call_api!(checkpoint.get => self.ptr, &mut x, &mut y, &mut z, &mut radius; or false))(
        );
        if success {
            Some((x, y, z, radius))
        } else {
            None
        }
    }

    pub fn set_race_checkpoint(
        &self,
        type_: i32,
        x: f32,
        y: f32,
        z: f32,
        next_x: f32,
        next_y: f32,
        next_z: f32,
        radius: f32,
    ) -> bool {
        call_api!(checkpoint.race_set => self.ptr, type_, x, y, z, next_x, next_y, next_z, radius; or false)
    }

    pub fn disable_race_checkpoint(&self) -> bool {
        call_api!(checkpoint.race_disable => self.ptr; or false)
    }

    pub fn is_in_race_checkpoint(&self) -> bool {
        call_api!(checkpoint.race_is_player_in => self.ptr; or false)
    }

    pub fn is_race_checkpoint_active(&self) -> bool {
        call_api!(checkpoint.race_is_active => self.ptr; or false)
    }

    pub fn get_race_checkpoint(&self) -> Option<(f32, f32, f32, f32, f32, f32, f32)> {
        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
        let (mut next_x, mut next_y, mut next_z) = (0.0f32, 0.0f32, 0.0f32);
        let mut radius = 0.0f32;

        let success = (|| call_api!(checkpoint.race_get => self.ptr, &mut x, &mut y, &mut z, &mut next_x, &mut next_y, &mut next_z, &mut radius; or false))(
        );
        if success {
            Some((x, y, z, next_x, next_y, next_z, radius))
        } else {
            None
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogStyle {
    MsgBox = 0,
    Input = 1,
    List = 2,
    Password = 3,
    TabList = 4,
    TabListHeaders = 5,
}
