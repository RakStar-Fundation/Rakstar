#[macro_export]
macro_rules! call_api {
    ($field_path:ident . $field:ident => $($args:expr),* ; or $default:expr) => {
        unsafe {
            let api = match $crate::macros::get_api() {
                Some(api) => api,
                None => return $default,
            };
            let Some(func) = api.$field_path.$field else {
                return $default;
            };
            func($($args),*)
        }
    };
}

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

            unsafe extern "C" fn on_player_disconnect_callback() -> bool {
                if let Some(ref mut comp) = COMPONENT {
                    use $crate::EventHandler;
                    comp.on_player_disconnect(0, 0);
                }
                true
            }

            unsafe extern "C" fn on_player_spawn_callback() -> bool {
                if let Some(ref mut comp) = COMPONENT {
                    use $crate::EventHandler;
                    comp.on_player_spawn(0);
                }
                true
            }

            unsafe extern "C" fn on_player_death_callback() -> bool {
                if let Some(ref mut comp) = COMPONENT {
                    use $crate::EventHandler;
                    comp.on_player_death(0, 0, 0);
                }
                true
            }

            unsafe extern "C" fn on_player_text_callback() -> bool {
                if let Some(ref mut comp) = COMPONENT {
                    use $crate::EventHandler;
                    return comp.on_player_text(0, "");
                }
                true
            }

            unsafe extern "C" fn on_player_command_text_callback() -> bool {
                if let Some(ref mut comp) = COMPONENT {
                    use $crate::EventHandler;
                    return comp.on_player_command_text(0, "");
                }
                true
            }

            unsafe extern "C" fn on_player_request_class_callback() -> bool {
                if let Some(ref mut comp) = COMPONENT {
                    use $crate::EventHandler;
                    return comp.on_player_request_class(0, 0);
                }
                true
            }

            unsafe extern "C" fn on_player_request_spawn_callback() -> bool {
                if let Some(ref mut comp) = COMPONENT {
                    use $crate::EventHandler;
                    return comp.on_player_request_spawn(0);
                }
                true
            }

            unsafe extern "C" fn on_player_key_state_change_callback() -> bool {
                if let Some(ref mut comp) = COMPONENT {
                    use $crate::EventHandler;
                    comp.on_player_key_state_change(0, 0, 0);
                }
                true
            }

            unsafe extern "C" fn on_player_stream_in_callback() -> bool {
                if let Some(ref mut comp) = COMPONENT {
                    use $crate::EventHandler;
                    comp.on_player_stream_in(0, 0);
                }
                true
            }

            unsafe extern "C" fn on_player_stream_out_callback() -> bool {
                if let Some(ref mut comp) = COMPONENT {
                    use $crate::EventHandler;
                    comp.on_player_stream_out(0, 0);
                }
                true
            }

            unsafe extern "C" fn on_player_take_damage_callback() -> bool {
                if let Some(ref mut comp) = COMPONENT {
                    use $crate::EventHandler;
                    comp.on_player_take_damage(0, 0, 0.0, 0, 0);
                }
                true
            }

            unsafe extern "C" fn on_player_give_damage_callback() -> bool {
                if let Some(ref mut comp) = COMPONENT {
                    use $crate::EventHandler;
                    comp.on_player_give_damage(0, 0, 0.0, 0, 0);
                }
                true
            }

            unsafe extern "C" fn on_player_enter_vehicle_callback() -> bool {
                if let Some(ref mut comp) = COMPONENT {
                    use $crate::EventHandler;
                    comp.on_player_enter_vehicle(0, 0, false);
                }
                true
            }

            unsafe extern "C" fn on_player_exit_vehicle_callback() -> bool {
                if let Some(ref mut comp) = COMPONENT {
                    use $crate::EventHandler;
                    comp.on_player_exit_vehicle(0, 0);
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
                            add_handler(event_name.as_ptr(), $crate::events::PRIORITY_USER, ::std::mem::transmute(on_player_connect_callback as *const ::std::ffi::c_void));
                            
                            let event_name = CString::new("onPlayerDisconnect").unwrap();
                            add_handler(event_name.as_ptr(), $crate::events::PRIORITY_USER, ::std::mem::transmute(on_player_disconnect_callback as *const ::std::ffi::c_void));
                            
                            let event_name = CString::new("onPlayerSpawn").unwrap();
                            add_handler(event_name.as_ptr(), $crate::events::PRIORITY_USER, ::std::mem::transmute(on_player_spawn_callback as *const ::std::ffi::c_void));
                            
                            let event_name = CString::new("onPlayerDeath").unwrap();
                            add_handler(event_name.as_ptr(), $crate::events::PRIORITY_USER, ::std::mem::transmute(on_player_death_callback as *const ::std::ffi::c_void));
                            
                            let event_name = CString::new("onPlayerText").unwrap();
                            add_handler(event_name.as_ptr(), $crate::events::PRIORITY_USER, ::std::mem::transmute(on_player_text_callback as *const ::std::ffi::c_void));
                            
                            let event_name = CString::new("onPlayerCommandText").unwrap();
                            add_handler(event_name.as_ptr(), $crate::events::PRIORITY_USER, ::std::mem::transmute(on_player_command_text_callback as *const ::std::ffi::c_void));
                            
                            let event_name = CString::new("onPlayerRequestClass").unwrap();
                            add_handler(event_name.as_ptr(), $crate::events::PRIORITY_USER, ::std::mem::transmute(on_player_request_class_callback as *const ::std::ffi::c_void));
                            
                            let event_name = CString::new("onPlayerRequestSpawn").unwrap();
                            add_handler(event_name.as_ptr(), $crate::events::PRIORITY_USER, ::std::mem::transmute(on_player_request_spawn_callback as *const ::std::ffi::c_void));
                            
                            let event_name = CString::new("onPlayerKeyStateChange").unwrap();
                            add_handler(event_name.as_ptr(), $crate::events::PRIORITY_USER, ::std::mem::transmute(on_player_key_state_change_callback as *const ::std::ffi::c_void));
                            
                            let event_name = CString::new("onPlayerStreamIn").unwrap();
                            add_handler(event_name.as_ptr(), $crate::events::PRIORITY_USER, ::std::mem::transmute(on_player_stream_in_callback as *const ::std::ffi::c_void));
                            
                            let event_name = CString::new("onPlayerStreamOut").unwrap();
                            add_handler(event_name.as_ptr(), $crate::events::PRIORITY_USER, ::std::mem::transmute(on_player_stream_out_callback as *const ::std::ffi::c_void));
                            
                            let event_name = CString::new("onPlayerTakeDamage").unwrap();
                            add_handler(event_name.as_ptr(), $crate::events::PRIORITY_USER, ::std::mem::transmute(on_player_take_damage_callback as *const ::std::ffi::c_void));
                            
                            let event_name = CString::new("onPlayerGiveDamage").unwrap();
                            add_handler(event_name.as_ptr(), $crate::events::PRIORITY_USER, ::std::mem::transmute(on_player_give_damage_callback as *const ::std::ffi::c_void));
                            
                            let event_name = CString::new("onPlayerEnterVehicle").unwrap();
                            add_handler(event_name.as_ptr(), $crate::events::PRIORITY_USER, ::std::mem::transmute(on_player_enter_vehicle_callback as *const ::std::ffi::c_void));
                            
                            let event_name = CString::new("onPlayerExitVehicle").unwrap();
                            add_handler(event_name.as_ptr(), $crate::events::PRIORITY_USER, ::std::mem::transmute(on_player_exit_vehicle_callback as *const ::std::ffi::c_void));
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
