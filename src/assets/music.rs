use crate::prelude::*;

/// Preloads the music assets when the game starts
pub(super) fn plugin(app: &mut App) {
    app.init_resource::<AssetMap<MusicAssetKey>>();
}

/// Defines all of the musical assets
/// Easy to access on any system using `Res<AssetMap<MusicAssetKey>>`
#[asset_key(AudioSource)]
pub enum MusicAssetKey {
    #[asset = "music/rain.ogg"]
    Ambient,
}
