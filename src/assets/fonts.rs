use bevy::prelude::*;

use crate::prelude::*;

/// Preloads the font assets when the game starts
pub(super) fn plugin(app: &mut App) {
    app.init_resource::<AssetMap<FontAssetKey>>();
}

/// Defines all of the font assets
/// Easy to access on any system using `Res<AssetMap<FontAssetKey>>`
#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum FontAssetKey {
    Main,
}

impl AssetKey for FontAssetKey {
    type Asset = Font;
}

impl FromWorld for AssetMap<FontAssetKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [(FontAssetKey::Main, asset_server.load("fonts/sans.ttf"))].into()
    }
}
