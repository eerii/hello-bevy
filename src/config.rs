use std::path::Path;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::GameState;

pub use bevy_persistent::prelude::*;

// ······
// Plugin
// ······

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), init_persistence);
    }
}

// ·········
// Resources
// ·········

#[derive(Resource, Serialize, Deserialize)]
pub struct GameOptions {
    pub test: bool,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self { test: true }
    }
}

// Keybinds
#[derive(Resource, Serialize, Deserialize)]
pub struct Keybinds {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub jump: KeyCode,
    pub interact: KeyCode,
    pub inventory: KeyCode,
    pub pause: KeyCode,
}

impl Default for Keybinds {
    fn default() -> Self {
        Self {
            up: KeyCode::W,
            down: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            jump: KeyCode::Space,
            interact: KeyCode::E,
            inventory: KeyCode::Tab,
            pause: KeyCode::Escape,
        }
    }
}

// ·······
// Systems
// ·······

fn init_persistence(mut cmd: Commands) {
    #[cfg(not(target_arch = "wasm32"))]
    let config_dir = Path::new(".data");
    #[cfg(target_arch = "wasm32")]
    let config_dir = Path::new("session");

    cmd.insert_resource(
        Persistent::<GameOptions>::builder()
            .name("options")
            .format(StorageFormat::Toml)
            .path(config_dir.join("options.toml"))
            .default(GameOptions::default())
            .revertible(true)
            .build()
            .expect("failed to initialize game options"),
    );

    cmd.insert_resource(
        Persistent::<Keybinds>::builder()
            .name("keybinds")
            .format(StorageFormat::Toml)
            .path(config_dir.join("keybinds.toml"))
            .default(Keybinds::default())
            .revertible(true)
            .revert_to_default_on_deserialization_errors(true)
            .build()
            .expect("failed to initialize keybinds"),
    );
}
