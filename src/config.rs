use std::path::Path;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{input::Bind, GameState};

pub use bevy_persistent::prelude::*;

// TODO: Accesibility
//          - Change font size
//          - Change color palette

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

#[derive(Resource, Serialize, Deserialize, Reflect)]
pub struct GameOptions {
    pub test: bool,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self { test: true }
    }
}

// Keybinds

#[derive(Resource, Serialize, Deserialize, Reflect)]
pub struct Keybinds {
    pub up: Vec<Bind>,
    pub down: Vec<Bind>,
    pub left: Vec<Bind>,
    pub right: Vec<Bind>,
    pub jump: Vec<Bind>,
    pub interact: Vec<Bind>,
    pub inventory: Vec<Bind>,
    pub pause: Vec<Bind>,
}

impl Default for Keybinds {
    fn default() -> Self {
        Self {
            up: vec![Bind::Key(KeyCode::W)],
            down: vec![Bind::Key(KeyCode::S)],
            left: vec![Bind::Key(KeyCode::A)],
            right: vec![Bind::Key(KeyCode::D)],
            jump: vec![Bind::Key(KeyCode::Space)],
            interact: vec![Bind::Key(KeyCode::E)],
            inventory: vec![Bind::Key(KeyCode::Tab)],
            pause: vec![Bind::Key(KeyCode::Escape)],
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
