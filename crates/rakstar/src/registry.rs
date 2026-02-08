use crate::{FeatureRegistry, GameData, MiddlewareRegistry};
use std::sync::Mutex;

static FEATURE_REGISTRY: Mutex<Option<usize>> = Mutex::new(None);
static MIDDLEWARE_REGISTRY: Mutex<Option<usize>> = Mutex::new(None);

pub fn get_feature_registry<T: GameData>() -> Option<&'static Mutex<FeatureRegistry<T>>> {
    let addr = (*FEATURE_REGISTRY.lock().unwrap())?;
    unsafe { (addr as *const Mutex<FeatureRegistry<T>>).as_ref() }
}

pub fn get_middleware_registry<T: GameData>() -> Option<&'static Mutex<MiddlewareRegistry<T>>> {
    let addr = (*MIDDLEWARE_REGISTRY.lock().unwrap())?;
    unsafe { (addr as *const Mutex<MiddlewareRegistry<T>>).as_ref() }
}

pub fn set_feature_registry<T: GameData>(registry: &'static Mutex<FeatureRegistry<T>>) {
    *FEATURE_REGISTRY.lock().unwrap() = Some(registry as *const _ as usize);
}

pub fn set_middleware_registry<T: GameData>(registry: &'static Mutex<MiddlewareRegistry<T>>) {
    *MIDDLEWARE_REGISTRY.lock().unwrap() = Some(registry as *const _ as usize);
}
