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

pub trait FromCEvent<S> {
    fn from_c(source: S) -> Self;
}

impl FromCEvent<bindings::types::PlayerPtr> for crate::Player {
    fn from_c(ptr: bindings::types::PlayerPtr) -> Self {
        crate::Player::from_ptr(ptr)
    }
}

impl FromCEvent<bindings::types::VehiclePtr> for crate::Vehicle {
    fn from_c(ptr: bindings::types::VehiclePtr) -> Self {
        crate::Vehicle::from_ptr(ptr)
    }
}

impl FromCEvent<bindings::types::CAPIStringView> for String {
    fn from_c(view: bindings::types::CAPIStringView) -> Self {
        unsafe { view.to_string().unwrap_or_default() }
    }
}

impl<T> FromCEvent<T> for T {
    fn from_c(val: T) -> Self {
        val
    }
}

#[macro_export]
macro_rules! handle_event {
    ($comp:expr, $method:ident, $args_ty:ty, $( ::<$T:ty>($field:ident) ),* ) => {
        unsafe extern "C" fn $method(raw_args: *mut $args_ty) -> bool {
            if let Some(ref mut comp) = $comp {
                use $crate::EventHandler;
                let list = &*(*raw_args).list;
                comp.$method(
                   $(
                       <$T as $crate::macros::FromCEvent<_>>::from_c(*list.$field)
                   ),*
                );
            }
            true
        }
    };
}

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

            $crate::handle_event!(
                COMPONENT,
                on_player_connect,
                $crate::bindings::types::EventArgs<$crate::bindings::types::OnPlayerConnect>,
                ::<$crate::Player>(player)
            );

            $crate::handle_event!(
                COMPONENT,
                on_player_disconnect,
                $crate::bindings::types::EventArgs<$crate::bindings::types::OnPlayerDisconnect>,
                ::<$crate::Player>(player),
                ::<_>(reason)
            );

            $crate::handle_event!(
                COMPONENT,
                on_player_spawn,
                $crate::bindings::types::EventArgs<$crate::bindings::types::OnPlayerSpawn>,
                ::<$crate::Player>(player)
            );

            $crate::handle_event!(
                COMPONENT,
                on_player_death,
                $crate::bindings::types::EventArgs<$crate::bindings::types::OnPlayerDeath>,
                ::<$crate::Player>(player),
                ::<$crate::Player>(killer),
                ::<_>(reason)
            );

            $crate::handle_event!(
                COMPONENT,
                on_player_text,
                $crate::bindings::types::EventArgs<$crate::bindings::types::OnPlayerText>,
                ::<$crate::Player>(player),
                ::<String>(text)
            );

            $crate::handle_event!(
                COMPONENT,
                on_player_command_text,
                $crate::bindings::types::EventArgs<$crate::bindings::types::OnPlayerCommandText>,
                ::<$crate::Player>(player),
                ::<String>(command)
            );

            $crate::handle_event!(
                COMPONENT,
                on_player_request_class,
                $crate::bindings::types::EventArgs<$crate::bindings::types::OnPlayerRequestClass>,
                ::<$crate::Player>(player),
                ::<_>(classId)
            );

            $crate::handle_event!(
                COMPONENT,
                on_player_request_spawn,
                $crate::bindings::types::EventArgs<$crate::bindings::types::OnPlayerRequestSpawn>,
                ::<$crate::Player>(player)
            );

            $crate::handle_event!(
                COMPONENT,
                on_player_key_state_change,
                $crate::bindings::types::EventArgs<$crate::bindings::types::OnPlayerKeyStateChange>,
                ::<$crate::Player>(player),
                ::<_>(newKeys),
                ::<_>(oldKeys)
            );

            $crate::handle_event!(
                COMPONENT,
                on_player_stream_in,
                $crate::bindings::types::EventArgs<$crate::bindings::types::OnPlayerStreamIn>,
                ::<$crate::Player>(player),
                ::<$crate::Player>(forPlayer)
            );

            $crate::handle_event!(
                COMPONENT,
                on_player_stream_out,
                $crate::bindings::types::EventArgs<$crate::bindings::types::OnPlayerStreamOut>,
                ::<$crate::Player>(player),
                ::<$crate::Player>(forPlayer)
            );

            $crate::handle_event!(
                COMPONENT,
                on_player_take_damage,
                $crate::bindings::types::EventArgs<$crate::bindings::types::OnPlayerTakeDamage>,
                ::<$crate::Player>(player),
                ::<$crate::Player>(from),
                ::<_>(amount),
                ::<_>(weapon),
                ::<_>(bodypart)
            );

            $crate::handle_event!(
                COMPONENT,
                on_player_give_damage,
                $crate::bindings::types::EventArgs<$crate::bindings::types::OnPlayerGiveDamage>,
                ::<$crate::Player>(player),
                ::<$crate::Player>(to),
                ::<_>(amount),
                ::<_>(weapon),
                ::<_>(bodypart)
            );

            $crate::handle_event!(
                COMPONENT,
                on_player_enter_vehicle,
                $crate::bindings::types::EventArgs<$crate::bindings::types::OnPlayerEnterVehicle>,
                ::<$crate::Player>(player),
                ::<$crate::Vehicle>(vehicle),
                ::<_>(passenger)
            );

            $crate::handle_event!(
                COMPONENT,
                on_player_exit_vehicle,
                $crate::bindings::types::EventArgs<$crate::bindings::types::OnPlayerExitVehicle>,
                ::<$crate::Player>(player),
                ::<$crate::Vehicle>(vehicle)
            );


            extern "C" fn on_ready() {
                unsafe {
                    if let Some(ref mut comp) = COMPONENT {
                        comp.on_ready();
                    }

                    $crate::events::internal::register_internal_handlers();

                            match $crate::macros::get_api() {
                                Some(api) => {
                                    if let Some(add_handler) = api.event.add_handler {
                                        macro_rules! register_event {
                                            ($c_name:expr, $cb:ident) => {
                                                let event_name = CString::new($c_name).unwrap();
                                                add_handler(
                                                    event_name.as_ptr(),
                                                    $crate::events::PRIORITY_USER,
                                                    ::std::mem::transmute($cb as *const ::std::ffi::c_void),
                                                );
                                            };
                                        }

                                        register_event!("onPlayerConnect", on_player_connect);
                                        register_event!("onPlayerDisconnect", on_player_disconnect);
                                        register_event!("onPlayerSpawn", on_player_spawn);
                                        register_event!("onPlayerDeath", on_player_death);
                                        register_event!("onPlayerText", on_player_text);
                                        register_event!("onPlayerCommandText", on_player_command_text);
                                        register_event!("onPlayerRequestClass", on_player_request_class);
                                        register_event!("onPlayerRequestSpawn", on_player_request_spawn);
                                        register_event!("onPlayerKeyStateChange", on_player_key_state_change);
                                        register_event!("onPlayerStreamIn", on_player_stream_in);
                                        register_event!("onPlayerStreamOut", on_player_stream_out);
                                        register_event!("onPlayerTakeDamage", on_player_take_damage);
                                        register_event!("onPlayerGiveDamage", on_player_give_damage);
                                        register_event!("onPlayerEnterVehicle", on_player_enter_vehicle);
                                        register_event!("onPlayerExitVehicle", on_player_exit_vehicle);
                                    }
                                }
                                None => {}
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
pub use bindings::api::{OmpApi as __OmpApi, initialize_capi as __initialize_capi};
#[doc(hidden)]
pub use bindings::types::ComponentVersion as __ComponentVersion;
