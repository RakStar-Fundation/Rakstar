mod command_feature;
mod dialog_feature;

pub use command_feature::CommandFeature;
pub use dialog_feature::DialogFeature;

use crate::FeatureRegistry;

pub fn register_internal_features(registry: &mut FeatureRegistry) {
    registry.register(CommandFeature::new());
    println!("[RakStar] Registered internal feature: CommandFeature");

    registry.register(DialogFeature::new());
     println!("[RakStar] Registered internal feature: DialogFeature");

}
