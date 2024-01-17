#![feature(type_changing_struct_update)]

mod assets;
mod audio;
mod camera;
mod data;
mod input;
mod ui;

use bevy::{asset::AssetMetaCheck, log::LogPlugin, prelude::*, window::WindowResolution};

// TODO: Add a lot of comments

// Exports for examples
pub use crate::{
    assets::{CoreAssets, ExampleAssets},
    camera::{init_camera, FinalCamera, GameCamera},
    data::{GameOptions, Keybinds},
    input::{InputMovement, KeyBind},
};

// Game state
// Indicates at which point the game is. Very useful for controlling which
// systems run when (in_state) and to create transitions (OnEnter/OnExit)
#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Play,
    End,
}

// Static configuration
// Allows to pass options to the game plugin such as the title and resolution.
// Must be added before the plugin
#[derive(Resource, Clone)]
pub struct GameAppConfig {
    pub game_title: &'static str,
    pub initial_window_res: WindowResolution,
    #[cfg(feature = "pixel_perfect")]
    pub initial_game_res: Vec2,
}

impl Default for GameAppConfig {
    fn default() -> Self {
        Self {
            game_title: "Hello bevy!",
            initial_window_res: Vec2::new(600., 600.).into(),
            #[cfg(feature = "pixel_perfect")]
            initial_game_res: Vec2::new(600., 600.),
        }
    }
}

// Main game plugin
// This template is structured using plugins. A plugin makes changes to the app,
// usually adding systems and resources. This is the main plugin that
// initializes all subsistems. Each plugin is defined in a submodule (mod ***)
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.world
            .get_resource_or_insert_with(GameAppConfig::default);
        let config = app.world.resource::<GameAppConfig>().clone();

        // Fix web builds for now
        app.insert_resource(AssetMetaCheck::Never);

        // Release only plugins (embedded assets)
        #[cfg(not(debug_assertions))]
        {
            use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
            app.add_plugins(EmbeddedAssetPlugin {
                mode: PluginMode::ReplaceDefault,
            });
        }

        // Default plugins

        // Window
        // Controls initial resolution, resizing
        let window_plugin = WindowPlugin {
            primary_window: Some(Window {
                title: config.game_title.into(),
                resolution: config.initial_window_res,
                resizable: cfg!(feature = "resizable"),
                canvas: Some("#bevy".to_string()),
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        };

        // Image
        // Sets the interpolation (nearest for pixel art, default otherwise)
        let image_plugin = if cfg!(feature = "pixel_perfect") {
            ImagePlugin::default_nearest()
        } else {
            ImagePlugin::default()
        };

        // Log
        // Modifies the logging to the console. More verbose when running debug builds
        let log_plugin = if cfg!(debug_assertions) {
            LogPlugin {
                level: bevy::log::Level::DEBUG,
                filter: "info,wgpu_core=warn,wgpu_hal=warn,calloop=error,hello-bevy=debug".into(),
                ..default()
            }
        } else {
            LogPlugin {
                level: bevy::log::Level::INFO,
                filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
                ..default()
            }
        };

        // Asset
        // In the future, it will use processed assets with Bevy Asset v2.
        // For now this is disabled since it is very early in development
        let asset_plugin = AssetPlugin {
            // mode: AssetMode::Processed,
            ..default()
        };

        // Add default bevy plugins with our overrides
        app.add_plugins(
            DefaultPlugins
                .set(window_plugin)
                .set(image_plugin)
                .set(log_plugin)
                .set(asset_plugin),
        );

        // Add the rest of the plugins
        app.init_state::<GameState>().add_plugins((
            data::DataPlugin,
            assets::AssetLoaderPlugin,
            input::InputPlugin,
            ui::UiPlugin,
            audio::AudioPlugin,
            camera::CameraPlugin,
        ));
    }
}
