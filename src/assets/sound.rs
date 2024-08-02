//! Sound assets.

use crate::prelude::*;

/// Preloads the sound assets when the game starts.
pub(super) fn plugin(app: &mut App) {
    app.load_asset::<SoundAssetKey>();
}

/// Defines all of the sound effects.
#[asset_key(AudioSource)]
pub enum SoundAssetKey {
    /// Placeholder sound effect.
    #[asset = "sound/boing.ogg"]
    Boing,
}
