use crate::prelude::*;

/// Preloads the sound assets when the game starts
pub(super) fn plugin(app: &mut App) {
    app.init_resource::<AssetMap<SoundAssetKey>>();
}

/// Defines all of the sound effects
/// Easy to access on any system using `Res<AssetMap<SoundAssetKey>>`
#[asset_key(AudioSource)]
pub enum SoundAssetKey {
    #[asset = "sound/boing.ogg"]
    Boing,
}
