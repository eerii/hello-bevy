use bevy::prelude::*;

use crate::prelude::*;

/// Preloads the meta assets when the game starts
pub(super) fn plugin(app: &mut App) {
    app.init_resource::<AssetMap<MetaAssetKey>>();
}

/// Defines all of the meta assets
/// Easy to access on any system using `Res<AssetMap<MetaAssetKey>>`
#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum MetaAssetKey {
    BevyLogo,
}

impl AssetKey for MetaAssetKey {
    type Asset = Image;
}

impl FromWorld for AssetMap<MetaAssetKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [(MetaAssetKey::BevyLogo, asset_server.load("meta/bevy.png"))].into()
    }
}
