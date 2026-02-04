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
        let ptr = call_api!(text_draw.create => x, y, c_text.as_ptr(), &mut id as *mut i32; or return None);
        if ptr.is_null() {
            return None;
        }
        Some(Self { ptr })
    }

    pub fn from_ptr(ptr: TextDrawPtr) -> Self {
        Self { ptr }
    }

    pub fn as_ptr(&self) -> TextDrawPtr {
        self.ptr
    }

    pub fn from_id(id: i32) -> Option<Self> {
        let ptr = call_api!(text_draw.from_id => id; or None);
        if ptr.is_null() {
            return None;
        }
        Some(Self { ptr })
    }

    pub fn get_id(&self) -> i32 {
        call_api!(text_draw.get_id => self.ptr; or -1)
    }

    pub fn destroy(&self) -> bool {
        call_api!(text_draw.destroy => self.ptr; or false)
    }

    pub fn set_letter_size(&self, x: f32, y: f32) -> bool {
        call_api!(text_draw.set_letter_size => self.ptr, x, y; or false)
    }

    pub fn set_text_size(&self, x: f32, y: f32) -> bool {
        call_api!(text_draw.set_text_size => self.ptr, x, y; or false)
    }

    pub fn set_alignment(&self, alignment: i32) -> bool {
        call_api!(text_draw.set_alignment => self.ptr, alignment; or false)
    }

    pub fn set_color(&self, color: u32) -> bool {
        call_api!(text_draw.set_color => self.ptr, color; or false)
    }

    pub fn set_font(&self, font: i32) -> bool {
        call_api!(text_draw.set_font => self.ptr, font; or false)
    }

    pub fn set_proportional(&self, proportional: bool) -> bool {
        call_api!(text_draw.set_proportional => self.ptr, proportional; or false)
    }

    pub fn show_for_player(&self, player: &crate::Player) -> bool {
        call_api!(text_draw.show_for_player => player.as_ptr(), self.ptr; or false)
    }

    pub fn hide_for_player(&self, player: &crate::Player) -> bool {
        call_api!(text_draw.hide_for_player => player.as_ptr(), self.ptr; or false)
    }

    pub fn show_for_all(&self) -> bool {
        call_api!(text_draw.show_for_all => self.ptr; or false)
    }

    pub fn hide_for_all(&self) -> bool {
        call_api!(text_draw.hide_for_all => self.ptr; or false)
    }

    pub fn set_string(&self, text: &str) -> bool {
        let c_text = std::ffi::CString::new(text).unwrap();
        call_api!(text_draw.set_string => self.ptr, c_text.as_ptr(); or false)
    }

    pub fn get_pos(&self) -> (f32, f32) {
        let (mut x, mut y) = (0.0f32, 0.0f32);
        call_api!(text_draw.get_pos => self.ptr, &mut x as *mut f32, &mut y as *mut f32; or (0.0, 0.0));
        (x, y)
    }

    pub fn set_pos(&self, x: f32, y: f32) -> bool {
        call_api!(text_draw.set_pos => self.ptr, x, y; or false)
    }
}
