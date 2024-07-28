use crate::prelude::*;

/// Preloads the meta assets when the game starts
pub(super) fn plugin(app: &mut App) {
    app.load_asset::<MetaAssetKey>();
}

/// Defines all of the meta assets
#[asset_key(Image)]
pub enum MetaAssetKey {
    #[asset = "meta/bevy.png"]
    BevyLogo,
}
