use std::ffi::{c_void, CString};
use super::types::ComponentVersion;

pub type ComponentCreateFn = unsafe extern "C" fn(
    uid: u64,
    name: *const i8,
    version: ComponentVersion,
    on_ready: *const c_void,
    on_reset: *const c_void,
    on_free: *const c_void,
) -> *mut c_void;

#[repr(C)]
pub struct ComponentApi {
    pub create: Option<ComponentCreateFn>,
}

#[repr(C)]
pub struct OmpApi {
    pub actor: *const c_void,
    pub checkpoint: *const c_void,
    pub race_checkpoint: *const c_void,
    pub class: *const c_void,
    pub player: *const c_void,
    pub component: ComponentApi,
    pub config: *const c_void,
    pub core: *const c_void,
    pub npc: *const c_void,
    pub custom_model: *const c_void,
    pub dialog: *const c_void,
    pub event: *const c_void,
    pub gang_zone: *const c_void,
    pub menu: *const c_void,
    pub object: *const c_void,
    pub player_object: *const c_void,
    pub pickup: *const c_void,
    pub all: *const c_void,
    pub recording: *const c_void,
    pub text_draw: *const c_void,
    pub player_text_draw: *const c_void,
    pub text_label: *const c_void,
    pub player_text_label: *const c_void,
    pub vehicle: *const c_void,
}

#[cfg(unix)]
pub unsafe fn initialize_capi(api: *mut OmpApi) -> bool {
    use std::ffi::CStr;
    
    let lib_path = CString::new("./components/$CAPI.so").unwrap();
    let lib = libc::dlopen(lib_path.as_ptr(), libc::RTLD_LAZY | libc::RTLD_LOCAL);
    
    if lib.is_null() {
        return false;
    }

    let create_fn_name = CString::new("Component_Create").unwrap();
    let create_fn_ptr = libc::dlsym(lib, create_fn_name.as_ptr());
    
    if create_fn_ptr.is_null() {
        libc::dlclose(lib);
        return false;
    }

    (*api).component.create = Some(std::mem::transmute(create_fn_ptr));
    
    true
}

#[cfg(windows)]
pub unsafe fn initialize_capi(api: *mut OmpApi) -> bool {
    false
}

