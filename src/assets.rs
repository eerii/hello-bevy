#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;

use crate::GameState;

// When not using the menu, skip to Play once Loading is done
const NEXT_STATE: GameState =
    if cfg!(feature = "menu") { GameState::Menu } else { GameState::Play };

// ······
// Plugin
// ······

// Asset loader
// Configures bevy_asset_loader to create collections that make it easy to
// specify which assets to load. These are then accesible through resources by
// any system. Also tracks asset loading progress through iyes_progress, making
// it easy to add a progress bar (see src/ui/loading.rs)
pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).load_collection::<ExampleAssets>(),
        )
        .init_collection::<CoreAssets>()
        .add_plugins(
            ProgressPlugin::new(GameState::Loading)
                .continue_to(NEXT_STATE)
                .track_assets(),
        );
    }
}

// ·········
// Resources
// ·········

// Assets for the splash screen and menus
// They are loaded inmediately after the app is fired, so they have no effect on
// loading state
#[derive(AssetCollection, Resource)]
pub struct CoreAssets {
    #[cfg(not(feature = "pixel_perfect"))]
    #[asset(path = "icons/bevy.png")]
    pub bevy_icon: Handle<Image>,

    #[cfg(not(feature = "pixel_perfect"))]
    #[asset(path = "fonts/sans.ttf")]
    pub font: Handle<Font>,

    #[cfg(feature = "pixel_perfect")]
    #[asset(path = "icons/pixelbevy.png")]
    pub bevy_icon: Handle<Image>,

    #[cfg(feature = "pixel_perfect")]
    #[asset(path = "fonts/pixel.ttf")]
    pub font: Handle<Font>,
}

// Example assets
// This is how an asset collection would look, defining assets with the #[asset]
// directive. See bevy_asset_loader for all options. You can create multiple
// collections.
#[derive(AssetCollection, Resource)]
pub struct ExampleAssets {
    #[asset(path = "sounds/boing.ogg")]
    pub boing: Handle<AudioSource>,

    #[asset(path = "music/soundscape.ogg")]
    pub ambient_music: Handle<AudioSource>,
}
