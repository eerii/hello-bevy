use std::path::Path;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{input::Bind, GameState};

pub use bevy_persistent::prelude::*;

// ······
// Plugin
// ······

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), init_persistence)
            .add_systems(
                PostUpdate,
                change_options.run_if(resource_changed::<Persistent<GameOptions>>()),
            );
    }
}

// ·········
// Resources
// ·········

#[derive(Serialize, Deserialize, Reflect)]
pub struct FontSize {
    pub title: f32,
    pub text: f32,
    pub button_text: f32,
}

impl Default for FontSize {
    fn default() -> Self {
        Self {
            title: 48.0,
            text: 24.0,
            button_text: 20.0,
        }
    }
}

#[derive(Serialize, Deserialize, Reflect)]
pub struct ColorPalette {
    pub light: Color,
    pub mid: Color,
    pub dark: Color,
    pub darker: Color,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            light: Color::rgb(245.0 / 255.0, 237.0 / 255.0, 200.0 / 255.0),
            mid: Color::rgb(69.0 / 255.0, 173.0 / 255.0, 118.0 / 255.0),
            dark: Color::rgb(43.0 / 255.0, 115.0 / 255.0, 77.0 / 255.0),
            darker: Color::rgb(55.0 / 255.0, 84.0 / 255.0, 70.0 / 255.0),
        }
    }
}

#[derive(Resource, Serialize, Deserialize, Reflect, Default)]
pub struct GameOptions {
    pub font_size: FontSize,
    pub color: ColorPalette,
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
            up: vec![
                Bind::Key(KeyCode::W),
                Bind::Gamepad(GamepadButtonType::DPadUp),
            ],
            down: vec![
                Bind::Key(KeyCode::S),
                Bind::Gamepad(GamepadButtonType::DPadDown),
            ],
            left: vec![
                Bind::Key(KeyCode::A),
                Bind::Gamepad(GamepadButtonType::DPadLeft),
            ],
            right: vec![
                Bind::Key(KeyCode::D),
                Bind::Gamepad(GamepadButtonType::DPadRight),
            ],
            jump: vec![
                Bind::Key(KeyCode::Space),
                Bind::Gamepad(GamepadButtonType::South),
            ],
            interact: vec![
                Bind::Key(KeyCode::E),
                Bind::Mouse(MouseButton::Left),
                Bind::Gamepad(GamepadButtonType::East),
            ],
            inventory: vec![
                Bind::Key(KeyCode::Tab),
                Bind::Gamepad(GamepadButtonType::West),
            ],
            pause: vec![
                Bind::Key(KeyCode::Escape),
                Bind::Gamepad(GamepadButtonType::Start),
            ],
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
            .revert_to_default_on_deserialization_errors(true)
            .build()
            .expect("Failed to initialize game options"),
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
            .expect("Failed to initialize keybinds"),
    );
}

fn change_options(mut cmd: Commands, options: Res<Persistent<GameOptions>>) {
    cmd.insert_resource(ClearColor(options.color.darker));
}
