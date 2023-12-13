mod assets;
mod audio;
mod data;
mod debug;
mod input;
mod ui;
mod utils;

use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    window::WindowResolution,
};

// Exports for examples
pub use crate::{
    assets::{
        CoreAssets,
        ExampleAssets,
    },
    data::{
        GameOptions,
        Keybinds,
    },
    input::Keybind,
};

// TODO: Port improvements from the game jam
// TODO: Add compilation guards for extra features (such as pixel art or resizable)
// TODO: Option for pixel perfect upscaling camera

// Game state
#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Play,
}

// Main game plugin
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Fix web builds for now
        app.insert_resource(AssetMetaCheck::Never);

        // Release only plugins (embedded assets)
        #[cfg(not(debug_assertions))]
        {
            use bevy_embedded_assets::{
                EmbeddedAssetPlugin,
                PluginMode,
            };
            app.add_plugins(EmbeddedAssetPlugin {
                mode: PluginMode::ReplaceDefault,
            });
        }

        // Default plugins
        app.add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Hello Bevy!".to_string(), // [CHANGE]: Game title
                    resolution: WindowResolution::new(600., 600.),
                    resizable: false, // Or use fit_canvas_to_parent: true for resizing on the web
                    canvas: Some("#bevy".to_string()),
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            //.set(ImagePlugin::default_nearest()), // [CHANGE]: Use if your game is pixel art
        );

        // Game
        app.add_state::<GameState>().add_plugins((
            assets::AssetLoaderPlugin,
            ui::UIPlugin,
            data::DataPlugin,
            input::InputPlugin,
            audio::AudioPlugin,
        ));

        // Debug only plugins
        #[cfg(debug_assertions)]
        {
            app.add_plugins(debug::DebugPlugin);
            debug::save_schedule(app);
        }
    }
}
