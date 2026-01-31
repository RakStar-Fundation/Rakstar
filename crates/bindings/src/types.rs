use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ComponentVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub prerel: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventPriorityType {
    Highest = 0,
    FairlyHigh = 1,
    Default = 2,
    FairlyLow = 3,
    Lowest = 4,
}

#[repr(C)]
pub struct EventArgsCommon {
    pub size: i32,
    pub list: *mut *mut c_void,
}

pub type EventCallbackCommon = unsafe extern "C" fn(args: *mut EventArgsCommon) -> bool;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CAPIStringView {
    pub len: u32,
    pub data: *const u8,
}

impl CAPIStringView {
    pub unsafe fn as_str(&self) -> Option<&str> {
        if self.data.is_null() {
            return None;
        }
        let slice = std::slice::from_raw_parts(self.data, self.len as usize);
        std::str::from_utf8(slice).ok()
    }

    pub unsafe fn to_string(&self) -> Option<String> {
        self.as_str().map(|s| s.to_string())
    }
}

#[repr(C)]
pub struct CAPIStringBuffer {
    pub capacity: u32,
    pub len: u32,
    pub data: *mut u8,
}

impl CAPIStringBuffer {
    pub fn new(buffer: &mut [u8]) -> Self {
        Self {
            capacity: buffer.len() as u32,
            len: 0,
            data: buffer.as_mut_ptr(),
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        if self.data.is_null() || self.len == 0 {
            return None;
        }
        unsafe {
            let slice = std::slice::from_raw_parts(self.data, self.len as usize);
            std::str::from_utf8(slice).ok()
        }
    }
}

pub type PlayerPtr = *mut c_void;
pub type VehiclePtr = *mut c_void;
pub type ActorPtr = *mut c_void;
pub type ObjectPtr = *mut c_void;
pub type PlayerObjectPtr = *mut c_void;
pub type PickupPtr = *mut c_void;
pub type MenuPtr = *mut c_void;
pub type TextDrawPtr = *mut c_void;
pub type PlayerTextDrawPtr = *mut c_void;
pub type TextLabelPtr = *mut c_void;
pub type PlayerTextLabel3DPtr = *mut c_void;
pub type ClassPtr = *mut c_void;
pub type GangZonePtr = *mut c_void;
pub type NPCPtr = *mut c_void;
