//! Music assets.

use crate::prelude::*;

/// Preloads the music assets when the game starts.
pub(super) fn plugin(app: &mut App) {
    app.load_asset::<MusicAssetKey>();
}

/// Defines all of the musical assets.
#[asset_key(AudioSource)]
pub enum MusicAssetKey {
    /// Placeholder background music.
    #[asset = "music/rain.ogg"]
    Ambient,
}
