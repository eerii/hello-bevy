use std::path::Path;

use bevy::prelude::*;
pub use bevy_persistent::prelude::*;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    input::{
        AxisBind,
        BindSet,
        InputAxis,
        KeyBind,
    },
    ui::{
        FONT_MULTIPLIERS,
        FONT_SIZES,
    },
};

// ······
// Plugin
// ······

pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) { app.add_systems(Startup, init_persistence); }
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
            dark: Color::rgb(55.0 / 255.0, 84.0 / 255.0, 70.0 / 255.0),
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
    #[cfg(not(target_arch = "wasm32"))]
    let config_dir = Path::new(".data");
    #[cfg(target_arch = "wasm32")]
    let config_dir = Path::new("local");

    // Append the package name to have unique configs (especially on web)
    let config_dir = config_dir.join(env!("CARGO_PKG_NAME"));

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
