#[macro_export]
macro_rules! entrypoint {
    ($component:ty, $name:expr, $version:expr) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn ComponentEntryPoint() -> *mut ::std::ffi::c_void {
            use ::std::ffi::CString;

            static mut COMPONENT: Option<$component> = None;
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

            ::std::ptr::null_mut()
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
