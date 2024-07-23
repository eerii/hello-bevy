use bevy::prelude::*;

use crate::prelude::*;

/// Preloads the music assets when the game starts
pub(super) fn plugin(app: &mut App) {
    app.init_resource::<AssetMap<MusicAssetKey>>();
}

/// Defines all of the musical assets
/// Easy to access on any system using `Res<AssetMap<MusicAssetKey>>`
#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum MusicAssetKey {
    Ambient,
}

impl AssetKey for MusicAssetKey {
    type Asset = AudioSource;
}

impl FromWorld for AssetMap<MusicAssetKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [(MusicAssetKey::Ambient, asset_server.load("music/rain.ogg"))].into()
    }
}
