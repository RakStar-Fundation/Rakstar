mod command_feature;
mod dialog_feature;

pub use command_feature::CommandFeature;
pub use dialog_feature::DialogFeature;

use crate::{FeatureRegistry, GameData};

pub fn register_internal_features<T: GameData>(registry: &mut FeatureRegistry<T>) {
    registry.register(CommandFeature::<T>::new());
    println!("[RakStar] Registered internal feature: CommandFeature");

    registry.register(DialogFeature::<T>::new());
    println!("[RakStar] Registered internal feature: DialogFeature");
}
