#![allow(non_snake_case)]
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
pub struct EventArgs<T> {
    pub size: i32,
    pub list: *mut T,
}

#[repr(C)]
pub struct EventArgsCommon {
    pub size: i32,
    pub list: *mut *mut c_void,
}

#[repr(C)]
pub struct OnPlayerConnect {
    pub player: *mut PlayerPtr,
}
#[repr(C)]
pub struct OnPlayerDisconnect {
    pub player: *mut PlayerPtr,
    pub reason: *mut i32,
}
#[repr(C)]
pub struct OnPlayerSpawn {
    pub player: *mut PlayerPtr,
}
#[repr(C)]
pub struct OnPlayerDeath {
    pub player: *mut PlayerPtr,
    pub killer: *mut PlayerPtr,
    pub reason: *mut i32,
}
#[repr(C)]
pub struct OnPlayerText {
    pub player: *mut PlayerPtr,
    pub text: *mut CAPIStringView,
}
#[repr(C)]
pub struct OnPlayerCommandText {
    pub player: *mut PlayerPtr,
    pub command: *mut CAPIStringView,
}
#[repr(C)]
pub struct OnPlayerRequestClass {
    pub player: *mut PlayerPtr,
    pub classId: *mut i32,
}
#[repr(C)]
pub struct OnPlayerRequestSpawn {
    pub player: *mut PlayerPtr,
}
#[repr(C)]
pub struct OnPlayerKeyStateChange {
    pub player: *mut PlayerPtr,
    pub newKeys: *mut i32,
    pub oldKeys: *mut i32,
}
#[repr(C)]
pub struct OnPlayerStreamIn {
    pub player: *mut PlayerPtr,
    pub forPlayer: *mut PlayerPtr,
}
#[repr(C)]
pub struct OnPlayerStreamOut {
    pub player: *mut PlayerPtr,
    pub forPlayer: *mut PlayerPtr,
}
#[repr(C)]
pub struct OnPlayerTakeDamage {
    pub player: *mut PlayerPtr,
    pub from: *mut PlayerPtr,
    pub amount: *mut f32,
    pub weapon: *mut i32,
    pub bodypart: *mut i32,
}
#[repr(C)]
pub struct OnPlayerGiveDamage {
    pub player: *mut PlayerPtr,
    pub to: *mut PlayerPtr,
    pub amount: *mut f32,
    pub weapon: *mut i32,
    pub bodypart: *mut i32,
}
#[repr(C)]
pub struct OnPlayerEnterVehicle {
    pub player: *mut PlayerPtr,
    pub vehicle: *mut VehiclePtr,
    pub passenger: *mut bool,
}
#[repr(C)]
pub struct OnPlayerExitVehicle {
    pub player: *mut PlayerPtr,
    pub vehicle: *mut VehiclePtr,
}
#[repr(C)]
pub struct OnDialogResponse {
    pub player: *mut PlayerPtr,
    pub dialogId: *mut i32,
    pub response: *mut i32,
    pub listItem: *mut i32,
    pub inputText: *mut CAPIStringView,
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
