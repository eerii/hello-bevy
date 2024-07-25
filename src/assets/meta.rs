use bevy::prelude::*;
use macros::asset_key;

use crate::prelude::*;

/// Preloads the meta assets when the game starts
pub(super) fn plugin(app: &mut App) {
    app.init_resource::<AssetMap<MetaAssetKey>>();
}

/// Defines all of the meta assets
/// Easy to access on any system using `Res<AssetMap<MetaAssetKey>>`
#[asset_key(Image)]
pub enum MetaAssetKey {
    #[asset = "meta/bevy.png"]
    BevyLogo,
}
