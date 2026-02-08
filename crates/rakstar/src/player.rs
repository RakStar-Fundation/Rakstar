use crate::call_api;
use bindings::types::{CAPIStringView, PlayerPtr};
use std::mem::MaybeUninit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player {
    ptr: PlayerPtr,
}

unsafe impl Send for Player {}

impl Player {
    pub fn from_ptr(ptr: PlayerPtr) -> Option<Self> {
        (!ptr.is_null()).then(|| Self { ptr })
    }

    pub fn as_ptr(&self) -> PlayerPtr {
        self.ptr
    }

    pub fn from_id(player_id: i32) -> Option<Self> {
        call_api!(player.from_id(player_id)).and_then(|ptr| Self::from_ptr(ptr))
    }

    pub fn get_id(&self) -> i32 {
        call_api!(player.get_id(self.ptr)).unwrap_or(-1)
    }

    pub fn get_name(&self) -> String {
        let mut name_view = MaybeUninit::<CAPIStringView>::uninit();

        call_api!(player.get_name(self.ptr, name_view.as_mut_ptr()));

        unsafe {
            let name_view = name_view.assume_init();
            name_view.to_string().unwrap()
        }
    }

    pub fn send_client_message(&self, color: u32, text: &str) -> bool {
        let c_text = std::ffi::CString::new(text).unwrap();

        call_api!(player.send_client_message(self.ptr, color, c_text.as_ptr() as *const u8))
            .unwrap_or(false)
    }

    pub fn kick(&self) -> bool {
        call_api!(player.kick(self.ptr)).unwrap_or(false)
    }

    pub fn ban(&self) -> bool {
        call_api!(player.ban(self.ptr)).unwrap_or(false)
    }

    pub fn spawn(&self) -> bool {
        call_api!(player.spawn(self.ptr)).unwrap_or(false)
    }

    pub fn get_health(&self) -> f32 {
        call_api!(player.get_health(self.ptr)).unwrap_or(0.0)
    }

    pub fn set_health(&self, health: f32) -> bool {
        call_api!(player.set_health(self.ptr, health)).unwrap_or(false)
    }

    pub fn get_armour(&self) -> f32 {
        call_api!(player.get_armor(self.ptr)).unwrap_or(0.0)
    }

    pub fn set_armour(&self, armour: f32) -> bool {
        call_api!(player.set_armor(self.ptr, armour)).unwrap_or(false)
    }

    pub fn get_pos(&self) -> (f32, f32, f32) {
        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);

        call_api!(player.get_pos(
            self.ptr,
            &mut x as *mut f32,
            &mut y as *mut f32,
            &mut z as *mut f32
        ));

        (x, y, z)
    }

    pub fn set_pos(&self, x: f32, y: f32, z: f32) -> bool {
        call_api!(player.set_pos(self.ptr, x, y, z)).unwrap_or(false)
    }

    pub fn get_interior(&self) -> i32 {
        call_api!(player.get_interior(self.ptr)).unwrap_or(0)
    }

    pub fn set_interior(&self, interior: i32) -> bool {
        call_api!(player.set_interior(self.ptr, interior)).unwrap_or(false)
    }

    pub fn get_virtual_world(&self) -> i32 {
        call_api!(player.get_virtual_world(self.ptr)).unwrap_or(0)
    }

    pub fn set_virtual_world(&self, world: i32) -> bool {
        call_api!(player.set_virtual_world(self.ptr, world)).unwrap_or(false)
    }

    pub fn reset_weapons(&self) -> bool {
        call_api!(player.reset_weapons(self.ptr)).unwrap_or(false)
    }

    pub fn give_weapon(&self, weapon: i32, ammo: i32) -> bool {
        call_api!(player.give_weapon(self.ptr, weapon, ammo)).unwrap_or(false)
    }

    pub fn put_in_vehicle(&self, vehicle: &crate::Vehicle, seat: i32) -> bool {
        call_api!(player.put_in_vehicle(self.ptr, vehicle.ptr, seat)).unwrap_or(false)
    }

    pub fn remove_from_vehicle(&self, force: bool) -> bool {
        call_api!(player.remove_from_vehicle(self.ptr, force)).unwrap_or(false)
    }

    pub fn is_in_vehicle(&self, vehicle: &crate::Vehicle) -> bool {
        call_api!(player.is_in_vehicle(self.ptr, vehicle.ptr)).unwrap_or(false)
    }

    pub fn is_in_any_vehicle(&self) -> bool {
        call_api!(player.is_in_any_vehicle(self.ptr)).unwrap_or(false)
    }

    pub fn is_spawned(&self) -> bool {
        call_api!(player.is_spawned(self.ptr)).unwrap_or(false)
    }

    pub fn get_vehicle_id(&self) -> i32 {
        call_api!(player.get_vehicle_id(self.ptr)).unwrap_or(-1)
    }

    pub fn get_facing_angle(&self) -> f32 {
        call_api!(player.get_facing_angle(self.ptr)).unwrap_or(0.0)
    }

    pub fn set_facing_angle(&self, angle: f32) -> bool {
        call_api!(player.set_facing_angle(self.ptr, angle)).unwrap_or(false)
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

        call_api!(dialog.show(
            self.ptr,
            dialog_id,
            style as i32,
            c_title.as_ptr(),
            c_body.as_ptr(),
            c_button1.as_ptr(),
            c_button2.as_ptr()
        ))
        .unwrap_or(false)
    }

    pub fn set_checkpoint(&self, x: f32, y: f32, z: f32, radius: f32) -> bool {
        call_api!(checkpoint.set(self.ptr, x, y, z, radius)).unwrap_or(false)
    }

    pub fn disable_checkpoint(&self) -> bool {
        call_api!(checkpoint.disable(self.ptr)).unwrap_or(false)
    }

    pub fn is_in_checkpoint(&self) -> bool {
        call_api!(checkpoint.is_player_in(self.ptr)).unwrap_or(false)
    }

    pub fn is_checkpoint_active(&self) -> bool {
        call_api!(checkpoint.is_active(self.ptr)).unwrap_or(false)
    }

    pub fn get_checkpoint(&self) -> Option<(f32, f32, f32, f32)> {
        let (mut x, mut y, mut z, mut radius) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);

        call_api!(checkpoint.get(self.ptr, &mut x, &mut y, &mut z, &mut radius))
            .filter(|&success| success)
            .map(|_| (x, y, z, radius))
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
        call_api!(checkpoint.race_set(self.ptr, type_, x, y, z, next_x, next_y, next_z, radius))
            .unwrap_or(false)
    }

    pub fn disable_race_checkpoint(&self) -> bool {
        call_api!(checkpoint.race_disable(self.ptr)).unwrap_or(false)
    }

    pub fn is_in_race_checkpoint(&self) -> bool {
        call_api!(checkpoint.race_is_player_in(self.ptr)).unwrap_or(false)
    }

    pub fn is_race_checkpoint_active(&self) -> bool {
        call_api!(checkpoint.race_is_active(self.ptr)).unwrap_or(false)
    }

    pub fn get_race_checkpoint(&self) -> Option<(f32, f32, f32, f32, f32, f32, f32)> {
        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
        let (mut next_x, mut next_y, mut next_z) = (0.0f32, 0.0f32, 0.0f32);
        let mut radius = 0.0f32;

        call_api!(checkpoint.race_get(
            self.ptr,
            &mut x,
            &mut y,
            &mut z,
            &mut next_x,
            &mut next_y,
            &mut next_z,
            &mut radius
        ))
        .filter(|&success| success)
        .map(|_| (x, y, z, next_x, next_y, next_z, radius))
    }

    pub fn get_ammo(&self) -> i32 {
        call_api!(player.get_player_ammo(self.ptr)).unwrap_or(0)
    }

    pub fn set_ammo(&self, weapon_id: u8, ammo: u32) -> bool {
        call_api!(player.set_ammo(self.ptr, weapon_id, ammo)).unwrap_or(false)
    }

    pub fn set_armed_weapon(&self, weapon: u8) -> bool {
        call_api!(player.set_armed_weapon(self.ptr, weapon)).unwrap_or(false)
    }

    pub fn get_weapon_data(&self, slot: i32) -> Option<(i32, i32)> {
        let mut weapon_id = 0i32;
        let mut ammo = 0i32;

        call_api!(player.get_weapon_data(self.ptr, slot, &mut weapon_id, &mut ammo))
            .filter(|&success| success)
            .map(|_| (weapon_id, ammo))
    }

    pub fn get_weapon_state(&self) -> i32 {
        call_api!(player.get_weapon_state(self.ptr)).unwrap_or(0)
    }

    pub fn get_keys(&self) -> (i32, i32, i32) {
        let mut keys = 0i32;
        let mut updown = 0i32;
        let mut leftright = 0i32;

        call_api!(player.get_keys(self.ptr, &mut keys, &mut updown, &mut leftright));

        (keys, updown, leftright)
    }

    pub fn get_camera_mode(&self) -> i32 {
        call_api!(player.get_camera_mode(self.ptr)).unwrap_or(0)
    }

    pub fn get_camera_zoom(&self) -> f32 {
        call_api!(player.get_camera_zoom(self.ptr)).unwrap_or(0.0)
    }

    pub fn get_camera_aspect_ratio(&self) -> f32 {
        call_api!(player.get_camera_aspect_ratio(self.ptr)).unwrap_or(0.0)
    }

    pub fn get_camera_front_vector(&self) -> (f32, f32, f32) {
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        let mut z = 0.0f32;

        call_api!(player.get_camera_front_vector(self.ptr, &mut x, &mut y, &mut z));

        (x, y, z)
    }

    pub fn get_animation_index(&self) -> i32 {
        call_api!(player.get_animation_index(self.ptr)).unwrap_or(0)
    }

    pub fn get_animation_flags(&self) -> i32 {
        call_api!(player.get_animation_flags(self.ptr)).unwrap_or(0)
    }

    pub fn get_animation_name(index: i32) -> Option<(String, String)> {
        let mut lib = MaybeUninit::<CAPIStringView>::uninit();
        let mut name = MaybeUninit::<CAPIStringView>::uninit();

        call_api!(player.get_animation_name(index, lib.as_mut_ptr(), name.as_mut_ptr()))
            .filter(|&success| success)
            .and_then(|_| unsafe {
                let lib_view = lib.assume_init();
                let name_view = name.assume_init();

                Some((
                    lib_view.to_string().unwrap_or_default(),
                    name_view.to_string().unwrap_or_default(),
                ))
            })
    }

    pub fn get_spectate_id(&self) -> i32 {
        call_api!(player.get_player_spectate_id(self.ptr)).unwrap_or(-1)
    }

    pub fn get_spectate_type(&self) -> i32 {
        call_api!(player.get_spectate_type(self.ptr)).unwrap_or(0)
    }

    pub fn spectate_player(&self, target: &Player, mode: i32) -> bool {
        call_api!(player.spectate_player(self.ptr, target.ptr, mode)).unwrap_or(false)
    }

    pub fn spectate_vehicle(&self, vehicle_ptr: bindings::types::VehiclePtr, mode: i32) -> bool {
        call_api!(player.spectate_vehicle(self.ptr, vehicle_ptr, mode)).unwrap_or(false)
    }

    pub fn get_surfing_vehicle(&self) -> *const std::ffi::c_void {
        call_api!(player.get_surfing_vehicle(self.ptr)).unwrap_or(std::ptr::null_mut())
    }

    pub fn get_surfing_object(&self) -> *const std::ffi::c_void {
        call_api!(player.get_surfing_object(self.ptr)).unwrap_or(std::ptr::null_mut())
    }

    pub fn get_surfing_offsets(&self) -> (f32, f32, f32) {
        let mut offset_x = 0.0f32;
        let mut offset_y = 0.0f32;
        let mut offset_z = 0.0f32;

        call_api!(player.get_surfing_offsets(
            self.ptr,
            &mut offset_x,
            &mut offset_y,
            &mut offset_z
        ));

        (offset_x, offset_y, offset_z)
    }

    pub fn get_target_player(&self) -> Option<Player> {
        call_api!(player.get_target_player(self.ptr)).and_then(|ptr| Player::from_ptr(ptr))
    }

    pub fn get_target_actor(&self) -> *const std::ffi::c_void {
        call_api!(player.get_target_actor(self.ptr)).unwrap_or(std::ptr::null())
    }

    pub fn get_distance_from_point(&self, x: f32, y: f32, z: f32) -> f32 {
        call_api!(player.get_distance_from_point(self.ptr, x, y, z)).unwrap_or(0.0)
    }

    pub fn is_in_range_of_point(&self, range: f32, x: f32, y: f32, z: f32) -> bool {
        call_api!(player.is_in_range_of_point(self.ptr, range, x, y, z)).unwrap_or(false)
    }

    pub fn get_skill_level(&self, skill: i32) -> i32 {
        call_api!(player.get_skill_level(self.ptr, skill)).unwrap_or(0)
    }

    pub fn set_skill_level(&self, weapon: u8, level: i32) -> bool {
        call_api!(player.set_skill_level(self.ptr, weapon, level)).unwrap_or(false)
    }

    pub fn get_wanted_level(&self) -> i32 {
        call_api!(player.get_wanted_level(self.ptr)).unwrap_or(0)
    }

    pub fn set_wanted_level(&self, level: i32) -> bool {
        call_api!(player.set_wanted_level(self.ptr, level)).unwrap_or(false)
    }

    pub fn get_drunk_level(&self) -> i32 {
        call_api!(player.get_drunk_level(self.ptr)).unwrap_or(0)
    }

    pub fn set_drunk_level(&self, level: i32) -> bool {
        call_api!(player.set_drunk_level(self.ptr, level)).unwrap_or(false)
    }

    pub fn get_special_action(&self) -> i32 {
        call_api!(player.get_special_action(self.ptr)).unwrap_or(0)
    }

    pub fn set_special_action(&self, action: u32) -> bool {
        call_api!(player.set_special_action(self.ptr, action)).unwrap_or(false)
    }

    pub fn get_fighting_style(&self) -> i32 {
        call_api!(player.get_fighting_style(self.ptr)).unwrap_or(0)
    }

    pub fn set_fighting_style(&self, style: i32) -> bool {
        call_api!(player.set_fighting_style(self.ptr, style)).unwrap_or(false)
    }

    pub fn force_class_selection(&self) -> bool {
        call_api!(player.force_class_selection(self.ptr)).unwrap_or(false)
    }

    pub fn allow_teleport(&self, allow: bool) -> bool {
        call_api!(player.allow_teleport(self.ptr, allow)).unwrap_or(false)
    }

    pub fn is_teleport_allowed(&self) -> bool {
        call_api!(player.is_teleport_allowed(self.ptr)).unwrap_or(false)
    }

    pub fn allow_weapons(&self, allow: bool) -> bool {
        call_api!(player.allow_weapons(self.ptr, allow)).unwrap_or(false)
    }

    pub fn are_weapons_allowed(&self) -> bool {
        call_api!(player.are_weapons_allowed(self.ptr)).unwrap_or(false)
    }

    pub fn toggle_clock(&self, enable: bool) -> bool {
        call_api!(player.toggle_clock(self.ptr, enable)).unwrap_or(false)
    }

    pub fn has_clock(&self) -> bool {
        call_api!(player.has_clock(self.ptr)).unwrap_or(false)
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
