mod config;
mod debug;
mod load;
mod menu;
mod sample_game;

use bevy::prelude::*;
use debug::save_schedule;

// Game state
#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    Loading,
    Menu,
    Play,
}

// Colors
pub const COLOR_LIGHT: Color = Color::rgb(245.0 / 255.0, 237.0 / 255.0, 200.0 / 255.0);
pub const COLOR_MID: Color = Color::rgb(69.0 / 255.0, 173.0 / 255.0, 118.0 / 255.0);
pub const COLOR_DARK: Color = Color::rgb(43.0 / 255.0, 115.0 / 255.0, 77.0 / 255.0);
pub const COLOR_DARKER: Color = Color::rgb(55.0 / 255.0, 84.0 / 255.0, 70.0 / 255.0);

// Main game plugin
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            load::LoadPlugin,
            menu::MenuPlugin,
            config::ConfigPlugin,
            sample_game::SampleGamePlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins(debug::DebugPlugin);
            save_schedule(app);
        }
    }
}
