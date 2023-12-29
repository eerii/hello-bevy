use std::collections::HashMap;

use bevy::{
    input::{
        gamepad::{GamepadAxisChangedEvent, GamepadButtonInput},
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseMotion},
        touch::TouchPhase,
        ButtonState,
    },
    prelude::*,
};
use bevy_persistent::Persistent;
use serde::{Deserialize, Serialize};

use crate::Keybinds;

// ······
// Plugin
// ······

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ButtonInput::<KeyBind>::default())
            .insert_resource(InputMovement::default())
            .add_systems(
                PreUpdate,
                (
                    handle_keyboard_input,
                    handle_mouse_input,
                    handle_gamepad_input,
                    handle_touch_input,
                ),
            )
            .add_systems(PostUpdate, clear_input);

        #[cfg(feature = "mock_touch")]
        app.add_systems(Update, mock_touch);
    }
}

// ·········
// Resources
// ·········

#[derive(Resource, Default)]
pub struct InputMovement {
    map: HashMap<AxisBind, f32>,
}

impl InputMovement {
    fn add(&mut self, bind: AxisBind, value: f32) {
        if value.abs() < 0.1 {
            self.map.remove(&bind);
        } else {
            self.map.insert(bind, value);
        }
    }

    // Don't clear gamepad
    pub fn clear(&mut self) {
        self.map
            .retain(|bind, _| matches!(bind, AxisBind::Gamepad(_)));
    }

    pub fn get(&self, bind: AxisBind) -> f32 {
        self.map.get(&bind).copied().unwrap_or(0.)
    }
}

// ·······
// Systems
// ·······

fn handle_keyboard_input(
    mut input: ResMut<ButtonInput<KeyBind>>,
    mut movement: ResMut<InputMovement>,
    keybinds: Res<Persistent<Keybinds>>,
    mut keyboard: EventReader<KeyboardInput>,
) {
    for event in keyboard.read() {
        for bind in keybinds.keys() {
            if let KeyBind::Key(key) = bind {
                if key != &event.key_code {
                    continue;
                }
                match event.state {
                    ButtonState::Pressed => input.press(*bind),
                    ButtonState::Released => input.release(*bind),
                }
            }
        }

        for bind in keybinds.moves() {
            if let AxisBind::Key(a, b) = bind {
                if a == &event.key_code {
                    match event.state {
                        ButtonState::Pressed => input.press(KeyBind::Key(*a)),
                        ButtonState::Released => input.release(KeyBind::Key(*a)),
                    }
                } else if b == &event.key_code {
                    match event.state {
                        ButtonState::Pressed => input.press(KeyBind::Key(*b)),
                        ButtonState::Released => input.release(KeyBind::Key(*b)),
                    }
                }
            }
        }
    }

    for bind in keybinds.moves() {
        if let AxisBind::Key(a, b) = bind {
            let mut value = 0.;
            if input.pressed(KeyBind::Key(*a)) {
                value += 1.
            } else if input.pressed(KeyBind::Key(*b)) {
                value -= 1.
            };
            movement.add(*bind, value);
        }
    }
}

fn handle_mouse_input(
    mut input: ResMut<ButtonInput<KeyBind>>,
    mut movement: ResMut<InputMovement>,
    keybinds: Res<Persistent<Keybinds>>,
    mut mouse: EventReader<MouseButtonInput>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    for event in mouse.read() {
        for bind in keybinds.keys() {
            if let KeyBind::Mouse(button) = bind {
                if button != &event.button {
                    continue;
                }
                match event.state {
                    ButtonState::Pressed => input.press(*bind),
                    ButtonState::Released => input.release(*bind),
                }
            }
        }
    }

    for event in mouse_motion.read() {
        for bind in keybinds.moves() {
            if let AxisBind::Mouse(axis) = bind {
                let value = match axis {
                    InputAxis::X => event.delta.x,
                    InputAxis::Y => event.delta.y,
                };
                movement.add(*bind, value);
            }
        }
    }
}

fn handle_gamepad_input(
    mut input: ResMut<ButtonInput<KeyBind>>,
    mut movement: ResMut<InputMovement>,
    keybinds: Res<Persistent<Keybinds>>,
    mut gamepad_buttons: EventReader<GamepadButtonInput>,
    mut gamepad_axis: EventReader<GamepadAxisChangedEvent>,
) {
    for event in gamepad_buttons.read() {
        for bind in keybinds.keys() {
            if let KeyBind::Gamepad(button) = bind {
                if button != &event.button.button_type {
                    continue;
                }
                match event.state {
                    ButtonState::Pressed => input.press(*bind),
                    ButtonState::Released => input.release(*bind),
                }
            }
        }
    }

    for event in gamepad_axis.read() {
        for bind in keybinds.moves() {
            if let AxisBind::Gamepad(axis) = bind {
                if axis != &event.axis_type {
                    continue;
                }
                movement.add(*bind, event.value);
            }
        }
    }
}

fn handle_touch_input(
    mut input: ResMut<ButtonInput<KeyBind>>,
    mut movement: ResMut<InputMovement>,
    keybinds: Res<Persistent<Keybinds>>,
    mut touch: EventReader<TouchInput>,
    mut prev_pos: Local<Option<Vec2>>,
) {
    let mut moved = Vec::new();

    for event in touch.read() {
        match event.phase {
            TouchPhase::Started => input.press(KeyBind::TouchPress),
            TouchPhase::Ended => input.release(KeyBind::TouchPress),
            TouchPhase::Moved => moved.push(*event),
            _ => (),
        }
    }

    for event in moved {
        for bind in keybinds.moves() {
            if let AxisBind::Touch(axis) = bind {
                if !input.pressed(KeyBind::TouchPress) {
                    continue;
                }

                let prev = prev_pos.get_or_insert(event.position);
                let delta = event.position - *prev;
                let value = match axis {
                    InputAxis::X => delta.x,
                    InputAxis::Y => delta.y,
                };

                movement.add(*bind, value);
                *prev = event.position;
            }
        }
    }
}

#[cfg(feature = "mock_touch")]
fn mock_touch(
    mouse: Res<ButtonInput<MouseButton>>,
    mut touch_events: EventWriter<TouchInput>,
    win: Query<&Window>,
) {
    let Ok(win) = win.get_single() else { return };

    let touch_phase = if mouse.just_pressed(MouseButton::Left) {
        Some(TouchPhase::Started)
    } else if mouse.just_released(MouseButton::Left) {
        Some(TouchPhase::Ended)
    } else if mouse.pressed(MouseButton::Left) {
        Some(TouchPhase::Moved)
    } else {
        None
    };
    if let (Some(phase), Some(cursor_pos)) = (touch_phase, win.cursor_position()) {
        touch_events.send(TouchInput {
            phase,
            position: cursor_pos,
            force: None,
            id: 0,
        });
    }
}

fn clear_input(mut input: ResMut<ButtonInput<KeyBind>>, mut movement: ResMut<InputMovement>) {
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
    TouchPress,
}

impl ToString for KeyBind {
    fn to_string(&self) -> String {
        match self {
            KeyBind::Key(key) => format!("{:?}", key),
            KeyBind::Mouse(button) => format!("m{:?}", button),
            KeyBind::Gamepad(button) => format!("g{:?}", button).replace("DPad", ""),
            KeyBind::TouchPress => "press".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum InputAxis {
    X,
    Y,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum AxisBind {
    Key(KeyCode, KeyCode),
    Mouse(InputAxis),
    Gamepad(GamepadAxisType),
    Touch(InputAxis),
}

#[derive(Debug, Serialize, Deserialize, Reflect)]
pub struct BindSet<T>(pub Vec<T>);

impl BindSet<KeyBind> {
    pub fn pressed(&self, input: &ButtonInput<KeyBind>) -> bool {
        self.0.iter().any(|bind| input.pressed(*bind))
    }

    pub fn just_pressed(&self, input: &ButtonInput<KeyBind>) -> bool {
        self.0.iter().any(|bind| input.just_pressed(*bind))
    }

    pub fn just_released(&self, input: &ButtonInput<KeyBind>) -> bool {
        self.0.iter().any(|bind| input.just_released(*bind))
    }
}

impl BindSet<AxisBind> {
    pub fn get(&self, movement: &InputMovement) -> f32 {
        self.0
            .iter()
            .map(|bind| movement.get(*bind))
            .sum::<f32>()
            .clamp(-1., 1.)
    }
}

impl Keybinds {
    pub fn keys(&self) -> Vec<&KeyBind> {
        self.iter_fields()
            .filter_map(|f| f.downcast_ref::<BindSet<KeyBind>>())
            .flat_map(|f| &f.0)
            .collect()
    }

    pub fn moves(&self) -> Vec<&AxisBind> {
        self.iter_fields()
            .filter_map(|f| f.downcast_ref::<BindSet<AxisBind>>())
            .flat_map(|f| &f.0)
            .collect()
    }
}
