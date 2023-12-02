pub mod audio;
pub mod config;
mod debug;
pub mod input;
pub mod load;
mod menu;
mod ui;

use bevy::prelude::*;

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
        app.add_state::<GameState>().add_plugins((
            load::LoadPlugin,
            ui::UIPlugin,
            menu::MenuPlugin,
            config::ConfigPlugin,
            input::InputPlugin,
            audio::AudioPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins(debug::DebugPlugin);
            debug::save_schedule(app);
        }
    }
}
