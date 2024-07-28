use crate::prelude::*;

/// Preloads the font assets when the game starts
pub(super) fn plugin(app: &mut App) {
    app.load_asset::<FontAssetKey>();
}

/// Defines all of the font assets
/// Easy to access on any system using `Res<AssetMap<FontAssetKey>>`
#[asset_key(Font)]
pub enum FontAssetKey {
    #[asset = "fonts/sans.ttf"]
    Main,
}
