#![allow(non_snake_case, non_camel_case_types)]
use crate::call_api;
use bindings::types::TextDrawPtr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextDraw {
    ptr: TextDrawPtr,
}

impl TextDraw {
    pub fn create(x: f32, y: f32, text: &str) -> Option<Self> {
        let mut id: i32 = -1;
        let c_text = std::ffi::CString::new(text).ok()?;

        call_api!(text_draw.create(x, y, c_text.as_ptr(), &mut id))
            .and_then(|ptr| Self::from_ptr(ptr))
    }

    pub fn from_ptr(ptr: TextDrawPtr) -> Option<Self> {
        (!ptr.is_null()).then(|| Self { ptr })
    }

    pub fn as_ptr(&self) -> TextDrawPtr {
        self.ptr
    }

    pub fn from_id(id: i32) -> Option<Self> {
        call_api!(text_draw.from_id(id)).and_then(|ptr| Self::from_ptr(ptr))
    }

    pub fn get_id(&self) -> i32 {
        call_api!(text_draw.get_id(self.ptr)).unwrap_or(-1)
    }

    pub fn destroy(&self) -> bool {
        call_api!(text_draw.destroy(self.ptr)).unwrap_or(false)
    }

    pub fn is_valid(&self) -> bool {
        call_api!(text_draw.is_valid(self.ptr)).unwrap_or(false)
    }

    pub fn is_visible_for_player(&self, player: &crate::Player) -> bool {
        call_api!(text_draw.is_visible_for_player(player.as_ptr(), self.ptr)).unwrap_or(false)
    }

    pub fn set_letter_size(&self, x: f32, y: f32) -> bool {
        call_api!(text_draw.set_letter_size(self.ptr, x, y)).unwrap_or(false)
    }

    pub fn set_text_size(&self, x: f32, y: f32) -> bool {
        call_api!(text_draw.set_text_size(self.ptr, x, y)).unwrap_or(false)
    }

    pub fn set_alignment(&self, alignment: i32) -> bool {
        call_api!(text_draw.set_alignment(self.ptr, alignment)).unwrap_or(false)
    }

    pub fn set_color(&self, color: u32) -> bool {
        call_api!(text_draw.set_color(self.ptr, color)).unwrap_or(false)
    }

    pub fn set_box_color(&self, color: u32) -> bool {
        call_api!(text_draw.set_box_color(self.ptr, color)).unwrap_or(false)
    }

    pub fn set_use_box(&self, use_box: bool) -> bool {
        call_api!(text_draw.set_use_box(self.ptr, use_box)).unwrap_or(false)
    }

    pub fn set_outline(&self, size: i32) -> bool {
        call_api!(text_draw.set_outline(self.ptr, size)).unwrap_or(false)
    }

    pub fn set_shadow(&self, size: i32) -> bool {
        call_api!(text_draw.set_shadow(self.ptr, size)).unwrap_or(false)
    }

    pub fn set_background_color(&self, color: u32) -> bool {
        call_api!(text_draw.set_background_color(self.ptr, color)).unwrap_or(false)
    }

    pub fn set_font(&self, font: i32) -> bool {
        call_api!(text_draw.set_font(self.ptr, font)).unwrap_or(false)
    }

    pub fn set_proportional(&self, proportional: bool) -> bool {
        call_api!(text_draw.set_proportional(self.ptr, proportional)).unwrap_or(false)
    }

    pub fn set_selectable(&self, selectable: bool) -> bool {
        call_api!(text_draw.set_selectable(self.ptr, selectable)).unwrap_or(false)
    }

    pub fn show_for_player(&self, player: &crate::Player) -> bool {
        call_api!(text_draw.show_for_player(player.as_ptr(), self.ptr)).unwrap_or(false)
    }

    pub fn hide_for_player(&self, player: &crate::Player) -> bool {
        call_api!(text_draw.hide_for_player(player.as_ptr(), self.ptr)).unwrap_or(false)
    }

    pub fn show_for_all(&self) -> bool {
        call_api!(text_draw.show_for_all(self.ptr)).unwrap_or(false)
    }

    pub fn hide_for_all(&self) -> bool {
        call_api!(text_draw.hide_for_all(self.ptr)).unwrap_or(false)
    }

    pub fn set_string(&self, text: &str) -> bool {
        let c_text = std::ffi::CString::new(text).unwrap();
        call_api!(text_draw.set_string(self.ptr, c_text.as_ptr())).unwrap_or(false)
    }

    pub fn set_string_for_player(&self, player: &crate::Player, text: &str) -> bool {
        let c_text = std::ffi::CString::new(text).unwrap();
        call_api!(text_draw.set_string_for_player(self.ptr, player.as_ptr(), c_text.as_ptr()))
            .unwrap_or(false)
    }

    pub fn set_preview_model(&self, model: i32) -> bool {
        call_api!(text_draw.set_preview_model(self.ptr, model)).unwrap_or(false)
    }

    pub fn set_preview_rot(
        &self,
        rotation_x: f32,
        rotation_y: f32,
        rotation_z: f32,
        zoom: f32,
    ) -> bool {
        call_api!(text_draw.set_preview_rot(self.ptr, rotation_x, rotation_y, rotation_z, zoom))
            .unwrap_or(false)
    }

    pub fn set_preview_veh_col(&self, color1: i32, color2: i32) -> bool {
        call_api!(text_draw.set_preview_veh_col(self.ptr, color1, color2)).unwrap_or(false)
    }

    pub fn get_pos(&self) -> (f32, f32) {
        let (mut x, mut y) = (0.0f32, 0.0f32);

        call_api!(text_draw.get_pos(self.ptr, &mut x as *mut f32, &mut y as *mut f32));

        (x, y)
    }

    pub fn set_pos(&self, x: f32, y: f32) -> bool {
        call_api!(text_draw.set_pos(self.ptr, x, y)).unwrap_or(false)
    }
}
