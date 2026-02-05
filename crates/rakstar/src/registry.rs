use crate::{FeatureRegistry, MiddlewareRegistry};
use std::sync::{Mutex, OnceLock};

static FEATURE_REGISTRY: OnceLock<Mutex<FeatureRegistry>> = OnceLock::new();
static MIDDLEWARE_REGISTRY: OnceLock<Mutex<MiddlewareRegistry>> = OnceLock::new();

pub fn get_feature_registry() -> Option<&'static Mutex<FeatureRegistry>> {
    FEATURE_REGISTRY.get()
}

pub fn get_middleware_registry() -> Option<&'static Mutex<MiddlewareRegistry>> {
    MIDDLEWARE_REGISTRY.get()
}
pub fn init_registries() {
    let _ = FEATURE_REGISTRY.set(Mutex::new(FeatureRegistry::new()));
    let _ = MIDDLEWARE_REGISTRY.set(Mutex::new(MiddlewareRegistry::new()));
}
