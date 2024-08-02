//! Font assets.

use crate::prelude::*;

/// Preloads the font assets when the game starts.
pub(super) fn plugin(app: &mut App) {
    app.load_asset::<FontAssetKey>();
}

/// Defines all of the font assets.
#[asset_key(Font)]
pub enum FontAssetKey {
    /// The font used in the Ui.
    #[asset = "fonts/sans.ttf"]
    Main,
}
