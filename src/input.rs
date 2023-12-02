use bevy::{
    input::{
        gamepad::GamepadButtonInput, keyboard::KeyboardInput, mouse::MouseButtonInput, ButtonState,
    },
    prelude::*,
};
use bevy_persistent::Persistent;
use serde::{Deserialize, Serialize};

use crate::config::Keybinds;

// TODO: Mouse movement and gamepad axis

// ······
// Plugin
// ······

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Input::<Bind>::default())
            .add_systems(
                PreUpdate,
                (
                    handle_input_keyboard,
                    handle_input_mouse,
                    handle_input_gamepad,
                )
                    .run_if(resource_exists::<Persistent<Keybinds>>()),
            )
            .add_systems(PostUpdate, clear_input);
    }
}

// ·········
// Resources
// ·········

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum Bind {
    Key(KeyCode),
    Mouse(MouseButton),
    Gamepad(GamepadButtonType),
}

impl ToString for Bind {
    fn to_string(&self) -> String {
        match self {
            Bind::Key(key) => format!("{:?}", key),
            Bind::Mouse(button) => format!("m{:?}", button),
            Bind::Gamepad(button) => format!("g{:?}", button).replace("DPad", ""),
        }
    }
}

// ·······
// Systems
// ·······

fn handle_input_keyboard(
    mut input: ResMut<Input<Bind>>,
    keybinds: Res<Persistent<Keybinds>>,
    mut keyboard: EventReader<KeyboardInput>,
) {
    for event in keyboard.read() {
        if let Some(event_key) = event.key_code {
            if let Some(keybind) = keybinds.all().iter().find(|bind| match bind {
                Bind::Key(key) => key == &event_key,
                _ => false,
            }) {
                match event.state {
                    ButtonState::Pressed => input.press(**keybind),
                    ButtonState::Released => input.release(**keybind),
                }
            }
        }
    }
}

fn handle_input_mouse(
    mut input: ResMut<Input<Bind>>,
    keybinds: Res<Persistent<Keybinds>>,
    mut mouse: EventReader<MouseButtonInput>,
) {
    for event in mouse.read() {
        if let Some(keybind) = keybinds.all().iter().find(|bind| match bind {
            Bind::Mouse(button) => button == &event.button,
            _ => false,
        }) {
            match event.state {
                ButtonState::Pressed => input.press(**keybind),
                ButtonState::Released => input.release(**keybind),
            }
        }
    }
}

fn handle_input_gamepad(
    mut input: ResMut<Input<Bind>>,
    keybinds: Res<Persistent<Keybinds>>,
    mut gamepad_buttons: EventReader<GamepadButtonInput>,
) {
    for event in gamepad_buttons.read() {
        if let Some(keybind) = keybinds.all().iter().find(|bind| match bind {
            Bind::Gamepad(button) => button == &event.button.button_type,
            _ => false,
        }) {
            match event.state {
                ButtonState::Pressed => input.press(**keybind),
                ButtonState::Released => input.release(**keybind),
            }
        }
    }
}

fn clear_input(mut input: ResMut<Input<Bind>>) {
    input.clear();
}
