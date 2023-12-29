#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use iyes_progress::prelude::*;

use crate::GameState;

#[cfg(feature = "menu")]
const NEXT_STATE: GameState = GameState::Menu;
#[cfg(not(feature = "menu"))]
const NEXT_STATE: GameState = GameState::Play;

// ······
// Plugin
// ······

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
// They are loaded inmediately after the app is fired, no effect on loading
// state
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
#[derive(AssetCollection, Resource)]
pub struct ExampleAssets {
    #[asset(path = "sounds/boing.ogg")]
    pub boing: Handle<AudioSource>,

    #[asset(path = "music/soundscape.ogg")]
    pub ambient_music: Handle<AudioSource>,
}
