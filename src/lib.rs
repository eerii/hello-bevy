mod debug;

pub use debug::{save_schedule, DEBUG};

use bevy::prelude::*;

// Game state
#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    Loading,
    Menu,
    Play,
}

// Main game plugin
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, game: &mut App) {
        game.add_state::<GameState>();

        #[cfg(debug_assertions)]
        game.add_plugin(debug::DebugPlugin);
    }
}
