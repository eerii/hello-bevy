use std::collections::HashMap;

use bevy::{
    input::{
        gamepad::{
            GamepadAxisChangedEvent,
            GamepadButtonInput,
        },
        keyboard::KeyboardInput,
        mouse::{
            MouseButtonInput,
            MouseMotion,
        },
        ButtonState,
    },
    prelude::*,
};
use bevy_persistent::Persistent;
use serde::{
    Deserialize,
    Serialize,
};

use crate::Keybinds;

// TODO: Add touch input

// ······
// Plugin
// ······

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Input::<KeyBind>::default())
            .insert_resource(Movement::default())
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

#[derive(Resource)]
pub struct Movement {
    map: HashMap<MoveBind, f32>,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl Movement {
    fn add(&mut self, bind: MoveBind, value: f32) {
        if value.abs() < 0.1 {
            self.map.remove(&bind);
        } else {
            self.map.insert(bind, value);
        }
    }

    // Don't clear gamepad
    pub fn clear(&mut self) {
        self.map.retain(|bind, _| match bind {
            MoveBind::Gamepad(_) => true,
            _ => false,
        });
    }

    pub fn get(&self, bind: MoveBind) -> f32 { self.map.get(&bind).copied().unwrap_or(0.) }
}

// ·······
// Systems
// ·······

fn handle_input_keyboard(
    mut input: ResMut<Input<KeyBind>>,
    mut movement: ResMut<Movement>,
    keybinds: Res<Persistent<Keybinds>>,
    mut keyboard: EventReader<KeyboardInput>,
) {
    for event in keyboard.read() {
        let Some(event_key) = event.key_code else { continue };

        for bind in keybinds.keys() {
            match bind {
                KeyBind::Key(key) => {
                    if key != &event_key {
                        continue;
                    }
                    match event.state {
                        ButtonState::Pressed => input.press(*bind),
                        ButtonState::Released => input.release(*bind),
                    }
                },
                _ => (),
            }
        }

        for bind in keybinds.moves() {
            match bind {
                MoveBind::KeyAxis(a, b) => {
                    if a == &event_key {
                        match event.state {
                            ButtonState::Pressed => input.press(KeyBind::Key(*a)),
                            ButtonState::Released => input.release(KeyBind::Key(*a)),
                        }
                    } else if b == &event_key {
                        match event.state {
                            ButtonState::Pressed => input.press(KeyBind::Key(*b)),
                            ButtonState::Released => input.release(KeyBind::Key(*b)),
                        }
                    }
                },
                _ => (),
            }
        }
    }

    for bind in keybinds.moves() {
        match bind {
            MoveBind::KeyAxis(a, b) => {
                let mut value = 0.;
                if input.pressed(KeyBind::Key(*a)) {
                    value += 1.
                } else if input.pressed(KeyBind::Key(*b)) {
                    value -= 1.
                };
                movement.add(*bind, value);
            },
            _ => (),
        }
    }
}

fn handle_input_mouse(
    mut input: ResMut<Input<KeyBind>>,
    mut movement: ResMut<Movement>,
    keybinds: Res<Persistent<Keybinds>>,
    mut mouse: EventReader<MouseButtonInput>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    for event in mouse.read() {
        for bind in keybinds.keys() {
            match bind {
                KeyBind::Mouse(button) => {
                    if button != &event.button {
                        continue;
                    }
                    match event.state {
                        ButtonState::Pressed => input.press(*bind),
                        ButtonState::Released => input.release(*bind),
                    }
                },
                _ => (),
            }
        }
    }

    for event in mouse_motion.read() {
        for bind in keybinds.moves() {
            match bind {
                MoveBind::MouseAxis(axis) => {
                    let value = match axis {
                        MouseAxis::X => event.delta.x,
                        MouseAxis::Y => event.delta.y,
                    };
                    movement.add(*bind, value);
                },
                _ => (),
            }
        }
    }
}

fn handle_input_gamepad(
    mut input: ResMut<Input<KeyBind>>,
    mut movement: ResMut<Movement>,
    keybinds: Res<Persistent<Keybinds>>,
    mut gamepad_buttons: EventReader<GamepadButtonInput>,
    mut gamepad_axis: EventReader<GamepadAxisChangedEvent>,
) {
    for event in gamepad_buttons.read() {
        for bind in keybinds.keys() {
            match bind {
                KeyBind::Gamepad(button) => {
                    if button != &event.button.button_type {
                        continue;
                    }
                    match event.state {
                        ButtonState::Pressed => input.press(*bind),
                        ButtonState::Released => input.release(*bind),
                    }
                },
                _ => (),
            }
        }
    }

    for event in gamepad_axis.read() {
        for bind in keybinds.moves() {
            match bind {
                MoveBind::Gamepad(axis) => {
                    if axis != &event.axis_type {
                        continue;
                    }
                    movement.add(*bind, event.value);
                },
                _ => (),
            }
        }
    }
}

fn clear_input(mut input: ResMut<Input<KeyBind>>, mut movement: ResMut<Movement>) {
    input.clear();
    movement.clear();
}

// ·····
// Extra
// ·····

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum KeyBind {
    Key(KeyCode),
    Mouse(MouseButton),
    Gamepad(GamepadButtonType),
}

impl ToString for KeyBind {
    fn to_string(&self) -> String {
        // TODO: Replace this with a key icon lookup, maybe use the ones from kenney once released
        match self {
            KeyBind::Key(key) => format!("{:?}", key),
            KeyBind::Mouse(button) => format!("m{:?}", button),
            KeyBind::Gamepad(button) => format!("g{:?}", button).replace("DPad", ""),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum MouseAxis {
    X,
    Y,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum MoveBind {
    KeyAxis(KeyCode, KeyCode),
    MouseAxis(MouseAxis),
    Gamepad(GamepadAxisType),
}
