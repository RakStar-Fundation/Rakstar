mod command_feature;

pub use command_feature::CommandFeature;

use crate::FeatureRegistry;

pub fn register_internal_features(registry: &mut FeatureRegistry) {
    registry.register(CommandFeature::new());
    println!("[RakStar] Registered internal feature: CommandFeature");
}
