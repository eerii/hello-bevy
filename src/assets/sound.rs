use bevy::prelude::*;

use crate::prelude::*;

/// Preloads the sound assets when the game starts
pub(super) fn plugin(app: &mut App) {
    app.init_resource::<AssetMap<SoundAssetKey>>();
}

/// Defines all of the sound effects
/// Easy to access on any system using `Res<AssetMap<SoundAssetKey>>`
#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SoundAssetKey {
    Boing,
}

impl AssetKey for SoundAssetKey {
    type Asset = AudioSource;
}

impl FromWorld for AssetMap<SoundAssetKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [(SoundAssetKey::Boing, asset_server.load("sound/boing.ogg"))].into()
    }
}
