use crate::call_api;
use bindings::types::{CAPIStringView, ObjectPtr, PlayerObjectPtr, PlayerPtr};
use std::mem::MaybeUninit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerObject {
    player: PlayerPtr,
    ptr: PlayerObjectPtr,
}

impl PlayerObject {
    pub fn from_ptr(player: PlayerPtr, ptr: PlayerObjectPtr) -> Self {
        Self { player, ptr }
    }

    pub fn create(
        player: &crate::Player,
        model_id: i32,
        x: f32,
        y: f32,
        z: f32,
        rot_x: f32,
        rot_y: f32,
        rot_z: f32,
        draw_distance: f32,
    ) -> Option<Self> {
        let mut id = 0;
        unsafe {
            let api = crate::macros::get_api()?;
            let Some(create) = api.player_object.create else {
                return None;
            };
            let ptr = create(
                player.as_ptr(),
                model_id,
                x,
                y,
                z,
                rot_x,
                rot_y,
                rot_z,
                draw_distance,
                &mut id,
            );
            if ptr.is_null() {
                return None;
            }
            Some(Self::from_ptr(player.as_ptr(), ptr))
        }
    }

    pub fn from_id(player: &crate::Player, id: i32) -> Option<Self> {
        unsafe {
            let api = crate::macros::get_api()?;
            let Some(from_id) = api.player_object.from_id else {
                return None;
            };
            let ptr = from_id(player.as_ptr(), id);
            if ptr.is_null() {
                return None;
            }
            Some(Self::from_ptr(player.as_ptr(), ptr))
        }
    }

    pub fn as_ptr(&self) -> PlayerObjectPtr {
        self.ptr
    }

    pub fn destroy(&self) -> bool {
        call_api!(player_object.destroy => self.player, self.ptr; or false)
    }

    pub fn get_id(&self) -> i32 {
        call_api!(player_object.get_id => self.player, self.ptr; or -1)
    }

    pub fn attach_to_vehicle(
        &self,
        vehicle: &crate::Vehicle,
        offset_x: f32,
        offset_y: f32,
        offset_z: f32,
        rot_x: f32,
        rot_y: f32,
        rot_z: f32,
    ) -> bool {
        call_api!(player_object.attach_to_vehicle => self.player, self.ptr, vehicle.ptr, offset_x, offset_y, offset_z, rot_x, rot_y, rot_z; or false)
    }

    pub fn attach_to_player(
        &self,
        attached_to: &crate::Player,
        offset_x: f32,
        offset_y: f32,
        offset_z: f32,
        rot_x: f32,
        rot_y: f32,
        rot_z: f32,
    ) -> bool {
        call_api!(player_object.attach_to_player => self.player, self.ptr, attached_to.as_ptr(), offset_x, offset_y, offset_z, rot_x, rot_y, rot_z; or false)
    }

    pub fn attach_to_object(
        &self,
        attached_to: ObjectPtr,
        offset_x: f32,
        offset_y: f32,
        offset_z: f32,
        rot_x: f32,
        rot_y: f32,
        rot_z: f32,
        sync_rotation: bool,
    ) -> bool {
        call_api!(player_object.attach_to_object => self.player, self.ptr, attached_to, offset_x, offset_y, offset_z, rot_x, rot_y, rot_z, sync_rotation; or false)
    }

    pub fn set_pos(&self, x: f32, y: f32, z: f32) -> bool {
        call_api!(player_object.set_pos => self.player, self.ptr, x, y, z; or false)
    }

    pub fn get_pos(&self) -> (f32, f32, f32) {
        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
        call_api!(player_object.get_pos => self.player, self.ptr, &mut x as *mut f32, &mut y as *mut f32, &mut z as *mut f32; or (0.0, 0.0, 0.0));
        (x, y, z)
    }

    pub fn set_rot(&self, rot_x: f32, rot_y: f32, rot_z: f32) -> bool {
        call_api!(player_object.set_rot => self.player, self.ptr, rot_x, rot_y, rot_z; or false)
    }

    pub fn get_rot(&self) -> (f32, f32, f32) {
        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
        call_api!(player_object.get_rot => self.player, self.ptr, &mut x as *mut f32, &mut y as *mut f32, &mut z as *mut f32; or (0.0, 0.0, 0.0));
        (x, y, z)
    }

    pub fn get_model(&self) -> i32 {
        call_api!(player_object.get_model => self.player, self.ptr; or 0)
    }

    pub fn set_no_camera_collision(&self) -> bool {
        call_api!(player_object.set_no_camera_collision => self.player, self.ptr; or false)
    }

    pub fn is_valid(&self) -> bool {
        call_api!(player_object.is_valid => self.player, self.ptr; or false)
    }

    pub fn move_(
        &self,
        x: f32,
        y: f32,
        z: f32,
        speed: f32,
        rot_x: f32,
        rot_y: f32,
        rot_z: f32,
    ) -> i32 {
        call_api!(player_object.move_ => self.player, self.ptr, x, y, z, speed, rot_x, rot_y, rot_z; or -1)
    }

    pub fn stop(&self) -> bool {
        call_api!(player_object.stop => self.player, self.ptr; or false)
    }

    pub fn is_moving(&self) -> bool {
        call_api!(player_object.is_moving => self.player, self.ptr; or false)
    }

    pub fn begin_editing(&self) -> bool {
        call_api!(player_object.begin_editing => self.player, self.ptr; or false)
    }

    pub fn set_material(
        &self,
        material_index: i32,
        model_id: i32,
        texture_library: &str,
        texture_name: &str,
        material_color: u32,
    ) -> bool {
        let c_lib = std::ffi::CString::new(texture_library).unwrap();
        let c_name = std::ffi::CString::new(texture_name).unwrap();
        call_api!(player_object.set_material => self.player, self.ptr, material_index, model_id, c_lib.as_ptr() as *const u8, c_name.as_ptr() as *const u8, material_color; or false)
    }

    pub fn set_material_text(
        &self,
        text: &str,
        material_index: i32,
        material_size: i32,
        font_face: &str,
        font_size: i32,
        bold: bool,
        font_color: u32,
        background_color: u32,
        text_alignment: i32,
    ) -> bool {
        let c_text = std::ffi::CString::new(text).unwrap();
        let c_font = std::ffi::CString::new(font_face).unwrap();
        call_api!(player_object.set_material_text => self.player, self.ptr, c_text.as_ptr() as *const u8, material_index, material_size, c_font.as_ptr() as *const u8, font_size, bold, font_color, background_color, text_alignment; or false)
    }

    pub fn get_draw_distance(&self) -> f32 {
        call_api!(player_object.get_draw_distance => self.player, self.ptr; or 0.0)
    }

    pub fn get_move_speed(&self) -> f32 {
        call_api!(player_object.get_move_speed => self.player, self.ptr; or 0.0)
    }

    pub fn get_moving_target_pos(&self) -> (f32, f32, f32) {
        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
        call_api!(player_object.get_moving_target_pos => self.player, self.ptr, &mut x as *mut f32, &mut y as *mut f32, &mut z as *mut f32; or (0.0, 0.0, 0.0));
        (x, y, z)
    }

    pub fn get_moving_target_rot(&self) -> (f32, f32, f32) {
        let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
        call_api!(player_object.get_moving_target_rot => self.player, self.ptr, &mut x as *mut f32, &mut y as *mut f32, &mut z as *mut f32; or (0.0, 0.0, 0.0));
        (x, y, z)
    }

    pub fn get_attached_data(&self) -> (i32, i32, i32) {
        let (mut vehicle, mut object, mut player) = (0, 0, 0);
        call_api!(player_object.get_attached_data => self.player, self.ptr, &mut vehicle as *mut i32, &mut object as *mut i32, &mut player as *mut i32; or (0, 0, 0));
        (vehicle, object, player)
    }

    pub fn get_attached_offset(&self) -> (f32, f32, f32, f32, f32, f32) {
        let (mut x, mut y, mut z, mut rx, mut ry, mut rz) =
            (0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32);
        call_api!(player_object.get_attached_offset => self.player, self.ptr, &mut x as *mut f32, &mut y as *mut f32, &mut z as *mut f32, &mut rx as *mut f32, &mut ry as *mut f32, &mut rz as *mut f32; or (0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
        (x, y, z, rx, ry, rz)
    }

    pub fn get_sync_rotation(&self) -> bool {
        call_api!(player_object.get_sync_rotation => self.player, self.ptr; or false)
    }

    pub fn is_material_slot_used(&self, material_index: i32) -> bool {
        call_api!(player_object.is_material_slot_used => self.player, self.ptr, material_index; or false)
    }

    pub fn get_material(&self, material_index: i32) -> (i32, String, String, i32) {
        let mut model_id = 0;
        let mut lib_view = MaybeUninit::<CAPIStringView>::uninit();
        let mut name_view = MaybeUninit::<CAPIStringView>::uninit();
        let mut material_color = 0;

        call_api!(player_object.get_material => self.player, self.ptr, material_index, &mut model_id as *mut i32, lib_view.as_mut_ptr(), name_view.as_mut_ptr(), &mut material_color as *mut i32; or (0, String::new(), String::new(), 0));

        unsafe {
            let lib_view = lib_view.assume_init();
            let name_view = name_view.assume_init();
            (
                model_id,
                lib_view.to_string().unwrap_or_default(),
                name_view.to_string().unwrap_or_default(),
                material_color,
            )
        }
    }

    pub fn get_material_text(
        &self,
        material_index: i32,
    ) -> (String, i32, String, i32, bool, i32, i32, i32) {
        let mut text_view = MaybeUninit::<CAPIStringView>::uninit();
        let mut material_size = 0;
        let mut font_face_view = MaybeUninit::<CAPIStringView>::uninit();
        let mut font_size = 0;
        let mut bold = false;
        let mut font_color = 0;
        let mut background_color = 0;
        let mut text_alignment = 0;

        call_api!(player_object.get_material_text => self.player, self.ptr, material_index, text_view.as_mut_ptr(), &mut material_size as *mut i32, font_face_view.as_mut_ptr(), &mut font_size as *mut i32, &mut bold as *mut bool, &mut font_color as *mut i32, &mut background_color as *mut i32, &mut text_alignment as *mut i32; or (String::new(), 0, String::new(), 0, false, 0, 0, 0));

        unsafe {
            let text_view = text_view.assume_init();
            let font_face_view = font_face_view.assume_init();
            (
                text_view.to_string().unwrap_or_default(),
                material_size,
                font_face_view.to_string().unwrap_or_default(),
                font_size,
                bold,
                font_color,
                background_color,
                text_alignment,
            )
        }
    }

    pub fn is_no_camera_collision(&self) -> bool {
        call_api!(player_object.is_no_camera_collision => self.player, self.ptr; or false)
    }
}
