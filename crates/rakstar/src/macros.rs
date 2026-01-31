#[macro_export]
macro_rules! entrypoint {
    ($component:ty, $name:expr, $version:expr) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn ComponentEntryPoint() -> *mut ::std::ffi::c_void {
            use ::std::ffi::CString;
            use ::std::mem::MaybeUninit;

            static mut COMPONENT: Option<$component> = None;
            static mut API: MaybeUninit<$crate::macros::__OmpApi> = MaybeUninit::uninit();

            let api_ptr = API.as_mut_ptr();
            if !$crate::macros::__initialize_capi(api_ptr) {
                eprintln!("Failed to initialize open.mp C API");
                return ::std::ptr::null_mut();
            }

            COMPONENT = Some(<$component>::default());

            let version = $crate::macros::__ComponentVersion {
                major: $version.0,
                minor: $version.1,
                patch: $version.2,
                prerel: $version.3,
            };

            let name = CString::new($name).unwrap();
            
            extern "C" fn on_ready() {
                unsafe {
                    if let Some(ref mut comp) = COMPONENT {
                        comp.on_ready();
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

            let api_ref = &*api_ptr;
            if let Some(create_fn) = api_ref.component.create {
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

pub trait Component: Default {
    fn on_ready(&mut self) {}
    fn on_reset(&mut self) {}
    fn on_free(&mut self) {}
}

#[doc(hidden)]
pub use bindings::types::ComponentVersion as __ComponentVersion;
#[doc(hidden)]
pub use bindings::api::{OmpApi as __OmpApi, initialize_capi as __initialize_capi};
