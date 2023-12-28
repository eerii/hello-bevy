#![feature(type_changing_struct_update)]

mod assets;
mod audio;
mod camera;
mod data;
mod input;
mod ui;

use bevy::{asset::AssetMetaCheck, log::LogPlugin, prelude::*};

// TODO: Add a lot of comments

// Exports for examples
pub use crate::{
    assets::{CoreAssets, ExampleAssets},
    camera::{FinalCamera, GameCamera},
    data::{GameOptions, Keybinds},
    input::{InputMovement, KeyBind},
};

// Game state
#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Play,
    End,
}

// Main game plugin
pub struct GamePlugin;

#[derive(Resource, Clone)]
pub struct GameAppConfig {
    pub game_title: &'static str,
    pub initial_window_res: Vec2,
    #[cfg(feature = "pixel_perfect")]
    pub initial_game_res: Vec2,
}

impl Default for GameAppConfig {
    fn default() -> Self {
        Self {
            game_title: "Hello bevy!",
            initial_window_res: Vec2::new(600., 600.),
            #[cfg(feature = "pixel_perfect")]
            initial_game_res: Vec2::new(600., 600.),
        }
    }
}

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
        #[allow(unused_mut)]
        let mut window_plugin = WindowPlugin {
            primary_window: Some(Window {
                title: config.game_title.into(),
                resolution: config.initial_window_res.into(),
                resizable: false,
                canvas: Some("#bevy".to_string()),
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        };

        #[cfg(feature = "resizable")]
        {
            let win = window_plugin.primary_window.as_mut().unwrap();
            win.resizable = true;
        }

        #[cfg(not(feature = "pixel_perfect"))]
        let image_plugin = ImagePlugin::default();

        #[cfg(feature = "pixel_perfect")]
        let image_plugin = ImagePlugin::default_nearest();

        #[cfg(debug_assertions)]
        let log_plugin = LogPlugin {
            level: bevy::log::Level::DEBUG,
            filter: "info,wgpu_core=warn,wgpu_hal=warn,calloop=error,hello-bevy=debug".into(),
        };

        #[cfg(not(debug_assertions))]
        let log_plugin = LogPlugin {
            level: bevy::log::Level::INFO,
            filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
        };

        app.add_plugins(
            DefaultPlugins
                .set(window_plugin)
                .set(image_plugin)
                .set(log_plugin),
        );

        // Game
        app.init_state::<GameState>().add_plugins((
            assets::AssetLoaderPlugin,
            ui::UiPlugin,
            data::DataPlugin,
            input::InputPlugin,
            audio::AudioPlugin,
            camera::CameraPlugin,
        ));
    }
}
