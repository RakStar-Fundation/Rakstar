#[macro_export]
macro_rules! entrypoint_new {
    (
        data: $data:ty,
        name: $name:expr,
        version: $version:expr
        $(, features: [ $($feature:expr),* $(,)? ])?
        $(, middlewares: [ $($middleware:expr),* $(,)? ])?
    ) => {
        static mut GAMEDATA: Option<::std::sync::Arc<::std::sync::Mutex<dyn $crate::GameData>>> = None;

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn ComponentEntryPoint() -> *mut ::std::ffi::c_void {
            use ::std::ffi::CString;
            use ::std::mem::MaybeUninit;
            use ::std::sync::{Arc, Mutex};

            let mut api_uninit = MaybeUninit::<$crate::macros::__OmpApi>::uninit();
            let api_ptr = api_uninit.as_mut_ptr();

            if !$crate::macros::__initialize_capi(api_ptr) {
                eprintln!("Failed to initialize open.mp C API");
                return ::std::ptr::null_mut();
            }

            let api = api_uninit.assume_init();
            let _ = $crate::macros::set_api(api);

            let data = Arc::new(Mutex::new(<$data>::new()));
            GAMEDATA = Some(data.clone());

            $crate::registry::init_registries();

            $(
                if let Some(middleware_registry) = $crate::get_middleware_registry() {
                    let mut registry = middleware_registry.lock().unwrap();
                    $(
                        registry.register($middleware);
                    )*
                }
            )?

            $(
                if let Some(feature_registry) = $crate::get_feature_registry() {
                    let mut registry = feature_registry.lock().unwrap();
                    $(
                        registry.register($feature);
                    )*
                }
            )?

            let version = $crate::macros::__ComponentVersion {
                major: $version.0,
                minor: $version.1,
                patch: $version.2,
                prerel: $version.3,
            };

            let name = CString::new($name).unwrap();

            $crate::handle_event_new!(on_player_connect, $crate::Player);
            $crate::handle_event_new!(on_player_disconnect, $crate::Player, i32);
            $crate::handle_event_new!(on_player_spawn, $crate::Player);
            $crate::handle_event_new!(on_player_text, $crate::Player, String);
            $crate::handle_event_new!(on_player_command_text, $crate::Player, String);
            $crate::handle_event_new!(on_dialog_response, $crate::Player, i32, i32, i32, String);

            let api_ref = $crate::macros::get_api().expect("API not initialized");

            if let Some(add_handler) = api_ref.event.add_handler {
                use ::std::ffi::CString;

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
                register_event!("onPlayerText", on_player_text);
                register_event!("onPlayerCommandText", on_player_command_text);
                register_event!("onDialogResponse", on_dialog_response);
            }

            unsafe extern "C" fn on_ready() {
                if let Some(ref data) = GAMEDATA {
                    data.lock().unwrap().on_ready();

                    if let Some(feature_registry) = $crate::get_feature_registry() {
                        feature_registry.lock().unwrap().on_ready(data);
                    }
                }

                $crate::runtime::init();
                $crate::events::internal::register_internal_handlers();
            }

        unsafe extern "C" fn on_reset() {
            if let Some(ref data) = GAMEDATA {
                data.lock().unwrap().on_reset();

                if let Some(feature_registry) = $crate::get_feature_registry() {
                    feature_registry.lock().unwrap().on_reset(data);
                }
            }
        }

        unsafe extern "C" fn on_free() {
            if let Some(ref data) = GAMEDATA {
                data.lock().unwrap().on_free();

                if let Some(feature_registry) = $crate::get_feature_registry() {
                    feature_registry.lock().unwrap().on_free(data);
                }
            }
        }

        let api_for_create = $crate::macros::get_api().expect("API not initialized");
        if let Some(create) = api_for_create.component.create {
            create(
                0x913B89092F8F6A68,
                name.as_ptr(),
                version,
                on_ready as *const ::std::ffi::c_void,
                on_reset as *const ::std::ffi::c_void,
                on_free as *const ::std::ffi::c_void,
            )
        } else {
            eprintln!("Component.Create function not available");
            ::std::ptr::null_mut()
        }
    }
};
}

#[macro_export]
macro_rules! handle_event_new {
    (on_player_connect, $player_ty:ty) => {
        unsafe extern "C" fn on_player_connect(
            raw_args: *mut $crate::bindings::types::EventArgs<
                $crate::bindings::types::OnPlayerConnect,
            >,
        ) -> bool {
            if let Some(ref data) = GAMEDATA {
                let list = &*(*raw_args).list;
                let player = <$player_ty as $crate::macros::FromCEvent<_>>::from_c(*list.player);

                if let Some(middleware_registry) = $crate::get_middleware_registry() {
                    if !middleware_registry
                        .lock()
                        .unwrap()
                        .dispatch_player_connect(player, data)
                    {
                        return true;
                    }
                }

                if let Some(feature_registry) = $crate::get_feature_registry() {
                    feature_registry
                        .lock()
                        .unwrap()
                        .dispatch_player_connect(player, data);
                }
            }
            true
        }
    };

    (on_player_disconnect, $player_ty:ty, $reason_ty:ty) => {
        unsafe extern "C" fn on_player_disconnect(
            raw_args: *mut $crate::bindings::types::EventArgs<
                $crate::bindings::types::OnPlayerDisconnect,
            >,
        ) -> bool {
            if let Some(ref data) = GAMEDATA {
                let list = &*(*raw_args).list;
                let player = <$player_ty as $crate::macros::FromCEvent<_>>::from_c(*list.player);
                let reason = <$reason_ty as $crate::macros::FromCEvent<_>>::from_c(*list.reason);

                if let Some(middleware_registry) = $crate::get_middleware_registry() {
                    if !middleware_registry
                        .lock()
                        .unwrap()
                        .dispatch_player_disconnect(player, reason, data)
                    {
                        return true;
                    }
                }

                if let Some(feature_registry) = $crate::get_feature_registry() {
                    feature_registry
                        .lock()
                        .unwrap()
                        .dispatch_player_disconnect(player, reason, data);
                }
            }
            true
        }
    };

    (on_player_spawn, $player_ty:ty) => {
        unsafe extern "C" fn on_player_spawn(
            raw_args: *mut $crate::bindings::types::EventArgs<
                $crate::bindings::types::OnPlayerSpawn,
            >,
        ) -> bool {
            if let Some(ref data) = GAMEDATA {
                let list = &*(*raw_args).list;
                let player = <$player_ty as $crate::macros::FromCEvent<_>>::from_c(*list.player);

                if let Some(middleware_registry) = $crate::get_middleware_registry() {
                    if !middleware_registry
                        .lock()
                        .unwrap()
                        .dispatch_player_spawn(player, data)
                    {
                        return true;
                    }
                }

                if let Some(feature_registry) = $crate::get_feature_registry() {
                    feature_registry
                        .lock()
                        .unwrap()
                        .dispatch_player_spawn(player, data);
                }
            }
            true
        }
    };

    (on_player_text, $player_ty:ty, $text_ty:ty) => {
        unsafe extern "C" fn on_player_text(
            raw_args: *mut $crate::bindings::types::EventArgs<
                $crate::bindings::types::OnPlayerText,
            >,
        ) -> bool {
            if let Some(ref data) = GAMEDATA {
                let list = &*(*raw_args).list;
                let player = <$player_ty as $crate::macros::FromCEvent<_>>::from_c(*list.player);
                let mut text = <$text_ty as $crate::macros::FromCEvent<_>>::from_c(*list.text);

                println!("dispatch player text");
                if let Some(middleware_registry) = $crate::get_middleware_registry() {
                    if !middleware_registry
                        .lock()
                        .unwrap()
                        .dispatch_player_text(player, &mut text, data)
                    {
                        return true;
                    }
                }

                if let Some(feature_registry) = $crate::get_feature_registry() {
                    feature_registry
                        .lock()
                        .unwrap()
                        .dispatch_player_text(player, text, data);
                }
            }
            true
        }
    };

    (on_player_command_text, $player_ty:ty, $cmd_ty:ty) => {
        unsafe extern "C" fn on_player_command_text(
            raw_args: *mut $crate::bindings::types::EventArgs<
                $crate::bindings::types::OnPlayerCommandText,
            >,
        ) -> bool {
            println!("[DEBUG] on_player_command_text handler called");

            if let Some(ref data) = GAMEDATA {
                let list = &*(*raw_args).list;
                let player = <$player_ty as $crate::macros::FromCEvent<_>>::from_c(*list.player);
                let command = <$cmd_ty as $crate::macros::FromCEvent<_>>::from_c(*list.command);

                println!(
                    "[DEBUG] Player: {}, Command: {}",
                    player.get_name(),
                    command
                );

                if let Some(middleware_registry) = $crate::get_middleware_registry() {
                    println!("[DEBUG] Dispatching to middleware registry");
                    if !middleware_registry
                        .lock()
                        .unwrap()
                        .dispatch_player_command_text(player, command.clone(), data)
                    {
                        return true;
                    }
                }

                if let Some(feature_registry) = $crate::get_feature_registry() {
                    println!("[DEBUG] Dispatching to feature registry");
                    feature_registry
                        .lock()
                        .unwrap()
                        .dispatch_player_command_text(player, command, data);
                }
            }
            false
        }
    };

    (on_dialog_response, $player_ty:ty, $($arg_ty:ty),+) => {
        unsafe extern "C" fn on_dialog_response(
            raw_args: *mut $crate::bindings::types::EventArgs<
                $crate::bindings::types::OnDialogResponse,
            >,
        ) -> bool {
            if let Some(ref data) = GAMEDATA {
                let list = &*(*raw_args).list;
                let player = <$player_ty as $crate::macros::FromCEvent<_>>::from_c(*list.player);
                let dialog_id = *list.dialogId;
                let response = *list.response;
                let list_item = *list.listItem;
                let input_text = <String as $crate::macros::FromCEvent<_>>::from_c(*list.inputText);

                if let Some(middleware_registry) = $crate::get_middleware_registry() {
                    if !middleware_registry
                        .lock()
                        .unwrap()
                        .dispatch_dialog_response(
                            player,
                            dialog_id,
                            response,
                            list_item,
                            input_text.clone(),
                            data,
                        )
                    {
                        return true;
                    }
                }

                if let Some(feature_registry) = $crate::get_feature_registry() {
                    feature_registry.lock().unwrap().dispatch_dialog_response(
                        player, dialog_id, response, list_item, input_text, data,
                    );
                }
            }
            true
        }
    };
}
