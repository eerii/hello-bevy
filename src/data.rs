use std::path::Path;

use bevy::prelude::*;
pub use bevy_persistent::prelude::*;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    input::Keybind,
    ui::{
        FONT_MULTIPLIERS,
        FONT_SIZES,
    },
    GameState,
};

// ······
// Plugin
// ······

pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Loading),
            init_persistence,
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
            title: FONT_SIZES[2] * FONT_MULTIPLIERS[0],
            text: FONT_SIZES[2] * FONT_MULTIPLIERS[1],
            button_text: FONT_SIZES[2] * FONT_MULTIPLIERS[2],
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
            light: Color::rgb(
                245.0 / 255.0,
                237.0 / 255.0,
                200.0 / 255.0,
            ),
            mid: Color::rgb(
                69.0 / 255.0,
                173.0 / 255.0,
                118.0 / 255.0,
            ),
            dark: Color::rgb(
                43.0 / 255.0,
                115.0 / 255.0,
                77.0 / 255.0,
            ),
            darker: Color::rgb(55.0 / 255.0, 84.0 / 255.0, 70.0 / 255.0),
        }
    }
}

#[derive(Resource, Serialize, Deserialize, Reflect, Default)]
pub struct GameOptions {
    pub font_size: FontSize,
    pub color: ColorPalette,
}

#[derive(Resource, Serialize, Deserialize, Reflect)]
pub struct Keybinds {
    pub up: Vec<Keybind>,
    pub down: Vec<Keybind>,
    pub left: Vec<Keybind>,
    pub right: Vec<Keybind>,
    pub jump: Vec<Keybind>,
    pub interact: Vec<Keybind>,
    pub inventory: Vec<Keybind>,
    pub pause: Vec<Keybind>,
}

impl Keybinds {
    pub fn all(&self) -> Vec<&Keybind> {
        self.iter_fields()
            .filter_map(|f| f.downcast_ref::<Vec<Keybind>>())
            .flatten()
            .collect()
    }
}

impl Default for Keybinds {
    fn default() -> Self {
        Self {
            up: vec![
                Keybind::Key(KeyCode::W),
                Keybind::Gamepad(GamepadButtonType::DPadUp),
            ],
            down: vec![
                Keybind::Key(KeyCode::S),
                Keybind::Gamepad(GamepadButtonType::DPadDown),
            ],
            left: vec![
                Keybind::Key(KeyCode::A),
                Keybind::Gamepad(GamepadButtonType::DPadLeft),
            ],
            right: vec![
                Keybind::Key(KeyCode::D),
                Keybind::Gamepad(GamepadButtonType::DPadRight),
            ],
            jump: vec![
                Keybind::Key(KeyCode::Space),
                Keybind::Gamepad(GamepadButtonType::South),
            ],
            interact: vec![
                Keybind::Key(KeyCode::E),
                Keybind::Mouse(MouseButton::Left),
                Keybind::Gamepad(GamepadButtonType::East),
            ],
            inventory: vec![
                Keybind::Key(KeyCode::Tab),
                Keybind::Gamepad(GamepadButtonType::West),
            ],
            pause: vec![
                Keybind::Key(KeyCode::Escape),
                Keybind::Gamepad(GamepadButtonType::Start),
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
    let config_dir = Path::new("local");

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
