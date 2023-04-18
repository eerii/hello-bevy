mod debug;
mod load;
mod save;
mod menu;

pub use debug::{save_schedule, DEBUG};

use bevy::prelude::*;
use bevy_tweening::TweeningPlugin;

// Game state
#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    Loading,
    Menu,
    Play,
    Fail,
}

// Main game plugin
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(TweeningPlugin)
            .add_plugin(load::LoadPlugin)
            .add_plugin(save::SavePlugin)
            .add_plugin(menu::MenuPlugin);

        #[cfg(debug_assertions)]
        app.add_plugin(debug::DebugPlugin);
    }
}
