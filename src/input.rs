use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// TODO: Mouse and gamepad axis
// TODO: Check when gamepad is connected/disconnected
//          - Create gamepad buttons automatically for that gamepad

// ·····
// Extra
// ·····

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum Bind {
    Key(KeyCode),
    Mouse(MouseButton),
    Gamepad(GamepadButton),
}

impl Bind {
    pub fn name(&self) -> String {
        match self {
            Bind::Key(key) => format!("{:?}", key),
            Bind::Mouse(button) => format!("{:?}", button),
            Bind::Gamepad(button) => format!("{:?}", button),
        }
    }
}

pub enum InputError {
    BindNotFound,
}

pub struct InputState<'a> {
    keyboard: Option<&'a Input<KeyCode>>,
    mouse: Option<&'a Input<MouseButton>>,
    gamepad: Option<&'a Input<GamepadButton>>,
}

impl<'a> InputState<'a> {
    pub fn new(
        keyboard: &'a Input<KeyCode>,
        mouse: &'a Input<MouseButton>,
        gamepad: &'a Input<GamepadButton>,
    ) -> Self {
        Self {
            keyboard: Some(keyboard),
            mouse: Some(mouse),
            gamepad: Some(gamepad),
        }
    }

    pub fn new_opt(
        keyboard: Option<&'a Input<KeyCode>>,
        mouse: Option<&'a Input<MouseButton>>,
        gamepad: Option<&'a Input<GamepadButton>>,
    ) -> Self {
        Self {
            keyboard,
            mouse,
            gamepad,
        }
    }

    pub fn pressed(&self, binds: &[Bind]) -> Result<bool, InputError> {
        for bind in binds {
            let is_pressed = match bind {
                Bind::Key(key) => self
                    .keyboard
                    .map_or(Err(InputError::BindNotFound), |keyboard| {
                        Ok(keyboard.pressed(*key))
                    }),
                Bind::Mouse(button) => self.mouse.map_or(Err(InputError::BindNotFound), |mouse| {
                    Ok(mouse.pressed(*button))
                }),
                Bind::Gamepad(button) => self
                    .gamepad
                    .map_or(Err(InputError::BindNotFound), |gamepad| {
                        Ok(gamepad.pressed(*button))
                    }),
            }?;
            if is_pressed {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn just_pressed(&self, binds: &[Bind]) -> Result<bool, InputError> {
        for bind in binds {
            let is_pressed = match bind {
                Bind::Key(key) => self
                    .keyboard
                    .map_or(Err(InputError::BindNotFound), |keyboard| {
                        Ok(keyboard.just_pressed(*key))
                    }),
                Bind::Mouse(button) => self.mouse.map_or(Err(InputError::BindNotFound), |mouse| {
                    Ok(mouse.just_pressed(*button))
                }),
                Bind::Gamepad(button) => self
                    .gamepad
                    .map_or(Err(InputError::BindNotFound), |gamepad| {
                        Ok(gamepad.just_pressed(*button))
                    }),
            }?;
            if is_pressed {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn just_released(&self, binds: &[Bind]) -> Result<bool, InputError> {
        for bind in binds {
            let is_pressed = match bind {
                Bind::Key(key) => self
                    .keyboard
                    .map_or(Err(InputError::BindNotFound), |keyboard| {
                        Ok(keyboard.just_released(*key))
                    }),
                Bind::Mouse(button) => self.mouse.map_or(Err(InputError::BindNotFound), |mouse| {
                    Ok(mouse.just_released(*button))
                }),
                Bind::Gamepad(button) => self
                    .gamepad
                    .map_or(Err(InputError::BindNotFound), |gamepad| {
                        Ok(gamepad.just_released(*button))
                    }),
            }?;
            if is_pressed {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
