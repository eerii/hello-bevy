use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

// Loading assets plugin
pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
        )
        .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading);
    }
}

// Test assets
#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/gb.ttf")]
    pub gameboy: Handle<Font>,
}
