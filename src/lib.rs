mod assets;
mod audio;
mod camera;
mod data;
mod input;
mod ui;

use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
};

// Exports for examples
pub use crate::{
    assets::{
        CoreAssets,
        ExampleAssets,
    },
    camera::GameCamera,
    data::{
        GameOptions,
        Keybinds,
    },
    input::{
        InputMovement,
        KeyBind,
    },
};

// [CHANGE]: Game title and resolution
pub const GAME_TITLE: &str = "Hello Bevy!";
pub const INITIAL_RESOLUTION: Vec2 = Vec2::new(600., 600.);

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
        #[allow(unused_mut)]
        let mut window_plugin = WindowPlugin {
            primary_window: Some(Window {
                title: GAME_TITLE.into(),
                resolution: INITIAL_RESOLUTION.into(),
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
            win.fit_canvas_to_parent = true;
        }

        #[cfg(not(feature = "pixel_perfect"))]
        let image_plugin = ImagePlugin::default();

        #[cfg(feature = "pixel_perfect")]
        let image_plugin = ImagePlugin::default_nearest();

        app.add_plugins(DefaultPlugins.set(window_plugin).set(image_plugin));

        // Game
        app.add_state::<GameState>().add_plugins((
            assets::AssetLoaderPlugin,
            ui::UIPlugin,
            data::DataPlugin,
            input::InputPlugin,
            audio::AudioPlugin,
            camera::CameraPlugin,
        ));
    }
}
