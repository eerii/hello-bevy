use std::path::Path;

use bevy::prelude::*;
pub use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    input::{AxisBind, BindSet, InputAxis, KeyBind},
    ui::{FONT_MULTIPLIERS, FONT_SIZES},
};

// ······
// Plugin
// ······

// Game data
// Uses bevy_persistent to create serializable game data, including options and
// keybinds. These are accesible using resources by any system, using
// Res<Persistent<...>>.
pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_persistence);
    }
}

// ·········
// Resources
// ·········

// Game options
// Useful for accesibility and the settings menu
#[derive(Resource, Serialize, Deserialize, Reflect, Default)]
pub struct GameOptions {
    pub font_size: FontSize,
    pub color: ColorPalette,
}

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
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            light: Color::hex("#547bb3").unwrap(),
            mid: Color::hex("#3b4363").unwrap(),
            dark: Color::hex("#191d30").unwrap(),
        }
    }
}

// Keybinds
// Offers remappable keymaps, both for keys and for axis. Has controller, mouse
// and touch support. See src/input.rs
#[derive(Resource, Serialize, Deserialize, Reflect)]
pub struct Keybinds {
    pub x_axis: BindSet<AxisBind>,
    pub y_axis: BindSet<AxisBind>,
    pub jump: BindSet<KeyBind>,
    pub interact: BindSet<KeyBind>,
    pub inventory: BindSet<KeyBind>,
    pub pause: BindSet<KeyBind>,
}

impl Default for Keybinds {
    fn default() -> Self {
        Self {
            x_axis: BindSet(vec![
                AxisBind::Key(KeyCode::KeyD, KeyCode::KeyA),
                AxisBind::Gamepad(GamepadAxisType::LeftStickX),
                // MoveBind::MouseAxis(InputAxis::X),
                AxisBind::Touch(InputAxis::X),
            ]),
            y_axis: BindSet(vec![
                AxisBind::Key(KeyCode::KeyW, KeyCode::KeyS),
                AxisBind::Gamepad(GamepadAxisType::LeftStickY),
            ]),
            jump: BindSet(vec![
                KeyBind::Key(KeyCode::Space),
                KeyBind::Gamepad(GamepadButtonType::South),
                KeyBind::TouchPress,
            ]),
            interact: BindSet(vec![
                KeyBind::Key(KeyCode::KeyE),
                KeyBind::Mouse(MouseButton::Left),
                KeyBind::Gamepad(GamepadButtonType::East),
            ]),
            inventory: BindSet(vec![
                KeyBind::Key(KeyCode::Tab),
                KeyBind::Gamepad(GamepadButtonType::West),
            ]),
            pause: BindSet(vec![
                KeyBind::Key(KeyCode::Escape),
                KeyBind::Gamepad(GamepadButtonType::Start),
            ]),
        }
    }
}

// ·······
// Systems
// ·······

fn init_persistence(mut cmd: Commands) {
    // Select the config directory
    // Append the package name to have unique configs (especially on web)
    let config_dir = Path::new(if cfg!(target_arch = "wasm32") { "local" } else { ".data" })
        .join(env!("CARGO_PKG_NAME"));

    // Insert the persistent resources with custom options
    // Check bevy_persistent for all the configuration
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
