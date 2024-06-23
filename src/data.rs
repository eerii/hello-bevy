use bevy::prelude::*;

// ······
// Plugin
// ······

// Game data
pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, _app: &mut App) {}
}

// ·········
// Resources
// ·········

// Game options
// Useful for accesibility and the settings menu
#[derive(Resource, Reflect, Default)]
pub struct GameOptions {}
