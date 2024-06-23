#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use bevy::prelude::*;

// const NEXT_STATE: GameState =
//     if cfg!(feature = "menu") { GameState::Menu } else { GameState::Play };

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
    fn build(&self, _app: &mut App) {}
}

// ·········
// Resources
// ·········

// #[derive(Resource, Debug)]
// struct LoadingData {
//     loading_assets: Vec<UntypedHandle>,
//     target: usize,
//     current: usize,
// }

// impl LoadingData {
//     fn new(target: usize) -> Self {
//         Self {
//             loading_assets: Vec::new(),
//             target,
//             current: 0,
//         }
//     }
// }

// Assets for the splash screen and menus
// They are loaded inmediately after the app is fired, so they have no effect on
// loading state
#[derive(Resource)]
pub struct CoreAssets {
    pub bevy_icon: Handle<Image>,
    pub font: Handle<Font>,
}

// Example assets
#[derive(Resource)]
pub struct ExampleAssets {
    pub boing: Handle<AudioSource>,
    pub ambient_music: Handle<AudioSource>,
}
