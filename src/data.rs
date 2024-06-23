use bevy::prelude::*;

// ······
// Plugin
// ······

// Game data
// Uses bevy_persistent to create serializable game data, including options and
// keybinds. These are accesible using resources by any system, using
// Res<Persistent<...>>.
pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, _app: &mut App) {
    }
}

// ·········
// Resources
// ·········

// Game options
// Useful for accesibility and the settings menu
#[derive(Resource, Reflect, Default)]
pub struct GameOptions {}
