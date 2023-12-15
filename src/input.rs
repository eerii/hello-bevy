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
        touch::TouchPhase,
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
                    handle_input_touch,
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
pub struct Movement {
    map: HashMap<MoveBind, f32>,
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
        self.map
            .retain(|bind, _| matches!(bind, MoveBind::Gamepad(_)));
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
            if let KeyBind::Key(key) = bind {
                if key != &event_key {
                    continue;
                }
                match event.state {
                    ButtonState::Pressed => input.press(*bind),
                    ButtonState::Released => input.release(*bind),
                }
            }
        }

        for bind in keybinds.moves() {
            if let MoveBind::KeyAxis(a, b) = bind {
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
            }
        }
    }

    for bind in keybinds.moves() {
        if let MoveBind::KeyAxis(a, b) = bind {
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

fn handle_input_mouse(
    mut input: ResMut<Input<KeyBind>>,
    mut movement: ResMut<Movement>,
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
            if let MoveBind::MouseAxis(axis) = bind {
                let value = match axis {
                    MovementAxis::X => event.delta.x,
                    MovementAxis::Y => event.delta.y,
                };
                movement.add(*bind, value);
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
            if let MoveBind::Gamepad(axis) = bind {
                if axis != &event.axis_type {
                    continue;
                }
                movement.add(*bind, event.value);
            }
        }
    }
}

fn handle_input_touch(
    mut input: ResMut<Input<KeyBind>>,
    mut movement: ResMut<Movement>,
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
            if let MoveBind::TouchAxis(axis) = bind {
                if !input.pressed(KeyBind::TouchPress) {
                    continue;
                }

                let prev = prev_pos.get_or_insert(event.position);
                let delta = event.position - *prev;
                let value = match axis {
                    MovementAxis::X => delta.x,
                    MovementAxis::Y => delta.y,
                };

                movement.add(*bind, value);
                *prev = event.position;
            }
        }
    }
}

#[cfg(feature = "mock_touch")]
fn mock_touch(
    mouse: Res<Input<MouseButton>>,
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
        })
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
    TouchPress,
}

impl ToString for KeyBind {
    fn to_string(&self) -> String {
        // TODO: Replace this with a key icon lookup, maybe use the ones from kenney once released
        match self {
            KeyBind::Key(key) => format!("{:?}", key),
            KeyBind::Mouse(button) => format!("m{:?}", button),
            KeyBind::Gamepad(button) => format!("g{:?}", button).replace("DPad", ""),
            KeyBind::TouchPress => "press".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum MovementAxis {
    X,
    Y,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum MoveBind {
    KeyAxis(KeyCode, KeyCode),
    MouseAxis(MovementAxis),
    Gamepad(GamepadAxisType),
    TouchAxis(MovementAxis),
}

#[derive(Debug, Serialize, Deserialize, Reflect)]
pub struct BindList<T>(pub Vec<T>);

impl BindList<KeyBind> {
    pub fn pressed(&self, input: &Input<KeyBind>) -> bool {
        self.0.iter().any(|bind| input.pressed(*bind))
    }

    pub fn just_pressed(&self, input: &Input<KeyBind>) -> bool {
        self.0.iter().any(|bind| input.just_pressed(*bind))
    }

    pub fn just_released(&self, input: &Input<KeyBind>) -> bool {
        self.0.iter().any(|bind| input.just_released(*bind))
    }
}

impl BindList<MoveBind> {
    pub fn get(&self, movement: &Movement) -> f32 {
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
            .filter_map(|f| f.downcast_ref::<BindList<KeyBind>>())
            .flat_map(|f| &f.0)
            .collect()
    }

    pub fn moves(&self) -> Vec<&MoveBind> {
        self.iter_fields()
            .filter_map(|f| f.downcast_ref::<BindList<MoveBind>>())
            .flat_map(|f| &f.0)
            .collect()
    }
}
