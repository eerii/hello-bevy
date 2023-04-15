mod debug;
mod load;

pub use debug::{save_schedule, DEBUG};

use bevy::prelude::*;

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
        app.add_state::<GameState>().add_plugin(load::LoadPlugin);

        #[cfg(debug_assertions)]
        app.add_plugin(debug::DebugPlugin);

        app.add_systems(Startup, init).add_systems(Update, update);
    }
}

// TODO: Move
fn init(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}
fn update() {}
