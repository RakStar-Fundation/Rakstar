use std::sync::OnceLock;

static API: OnceLock<bindings::api::OmpApi> = OnceLock::new();

#[macro_export]
macro_rules! entrypoint {
    ($component:ty, $name:expr, $version:expr) => {
        static mut COMPONENT: Option<$component> = None;

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn ComponentEntryPoint() -> *mut ::std::ffi::c_void {
            use ::std::ffi::CString;
            use ::std::mem::MaybeUninit;

            let mut api_uninit = MaybeUninit::<$crate::macros::__OmpApi>::uninit();
            let api_ptr = api_uninit.as_mut_ptr();
            
            if !$crate::macros::__initialize_capi(api_ptr) {
                eprintln!("Failed to initialize open.mp C API");
                return ::std::ptr::null_mut();
            }
            
            let api = api_uninit.assume_init();
            let _ = $crate::macros::set_api(api);

            COMPONENT = Some(<$component>::default());

            let version = $crate::macros::__ComponentVersion {
                major: $version.0,
                minor: $version.1,
                patch: $version.2,
                prerel: $version.3,
            };

            let name = CString::new($name).unwrap();
            
            unsafe extern "C" fn on_player_connect_callback() -> bool {
                if let Some(ref mut comp) = COMPONENT {
                    use $crate::EventHandler;
                    comp.on_player_connect(0);
                }
                true
            }
            
            extern "C" fn on_ready() {
                unsafe {
                    if let Some(ref mut comp) = COMPONENT {
                        comp.on_ready();
                    }
                    
                    $crate::events::internal::register_internal_handlers();
                    
                    if let Some(api) = $crate::macros::get_api() {
                        if let Some(add_handler) = api.event.add_handler {
                            let event_name = CString::new("onPlayerConnect").unwrap();
                            add_handler(
                                event_name.as_ptr(),
                                $crate::events::PRIORITY_USER,
                                ::std::mem::transmute(on_player_connect_callback as *const ::std::ffi::c_void)
                            );
                        }
                    }
                }
            }
            
            extern "C" fn on_reset() {
                unsafe {
                    if let Some(ref mut comp) = COMPONENT {
                        comp.on_reset();
                    }
                }
            }
            
            extern "C" fn on_free() {
                unsafe {
                    if let Some(ref mut comp) = COMPONENT {
                        comp.on_free();
                    }
                }
            }

            let api = $crate::macros::get_api().expect("API not initialized");
            if let Some(create_fn) = api.component.create {
                let component_ptr = create_fn(
                    0x913B89092F8F6A68,
                    name.as_ptr(),
                    version,
                    on_ready as *const ::std::ffi::c_void,
                    on_reset as *const ::std::ffi::c_void,
                    on_free as *const ::std::ffi::c_void,
                );
                component_ptr
            } else {
                eprintln!("Component.Create function not available");
                ::std::ptr::null_mut()
            }
        }
    };
}

pub fn get_api() -> Option<&'static bindings::api::OmpApi> {
    API.get()
}

pub fn set_api(api: bindings::api::OmpApi) -> Result<(), bindings::api::OmpApi> {
    API.set(api)
}

pub trait Component: Default {
    fn on_ready(&mut self) {}
    fn on_reset(&mut self) {}
    fn on_free(&mut self) {}
}

#[doc(hidden)]
pub use bindings::types::ComponentVersion as __ComponentVersion;
#[doc(hidden)]
pub use bindings::api::{OmpApi as __OmpApi, initialize_capi as __initialize_capi};
