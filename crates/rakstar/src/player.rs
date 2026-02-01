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

    pub fn get_name(&self) -> Option<String> {
        let mut name_view = MaybeUninit::<CAPIStringView>::uninit();
        unsafe {
            call_api!(player.get_name => self.ptr, name_view.as_mut_ptr());
            let name_view = name_view.assume_init();
            name_view.to_string()
        }
    }
}
