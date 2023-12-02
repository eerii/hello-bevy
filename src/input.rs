use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// TODO: Mouse movement and gamepad axis

// ·····
// Extra
// ·····

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
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

pub enum InputError {
    BindNotFound,
}

pub struct InputState<'a> {
    keyboard: Option<&'a Input<KeyCode>>,
    mouse: Option<&'a Input<MouseButton>>,
    gamepads: Option<&'a Gamepads>,
    gamepad_buttons: Option<&'a Input<GamepadButton>>,
}

impl<'a> InputState<'a> {
    pub fn new(
        keyboard: &'a Input<KeyCode>,
        mouse: &'a Input<MouseButton>,
        gamepads: &'a Gamepads,
        gamepad_buttons: &'a Input<GamepadButton>,
    ) -> Self {
        Self {
            keyboard: Some(keyboard),
            mouse: Some(mouse),
            gamepads: Some(gamepads),
            gamepad_buttons: Some(gamepad_buttons),
        }
    }

    pub fn new_opt(
        keyboard: Option<&'a Input<KeyCode>>,
        mouse: Option<&'a Input<MouseButton>>,
        gamepads: Option<&'a Gamepads>,
        gamepad_buttons: Option<&'a Input<GamepadButton>>,
    ) -> Self {
        Self {
            keyboard,
            mouse,
            gamepads,
            gamepad_buttons,
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
                Bind::Gamepad(button) => {
                    if let Some(gamepads) = self.gamepads {
                        for gamepad in gamepads.iter() {
                            if let Some(buttons) = self.gamepad_buttons {
                                if buttons.pressed(GamepadButton::new(gamepad, *button)) {
                                    return Ok(true);
                                }
                            } else {
                                return Err(InputError::BindNotFound);
                            }
                        }
                        Ok(false)
                    } else {
                        Err(InputError::BindNotFound)
                    }
                }
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
                Bind::Gamepad(button) => {
                    if let Some(gamepads) = self.gamepads {
                        for gamepad in gamepads.iter() {
                            if let Some(buttons) = self.gamepad_buttons {
                                if buttons.just_pressed(GamepadButton::new(gamepad, *button)) {
                                    return Ok(true);
                                }
                            } else {
                                return Err(InputError::BindNotFound);
                            }
                        }
                        Ok(false)
                    } else {
                        Err(InputError::BindNotFound)
                    }
                }
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
                Bind::Gamepad(button) => {
                    if let Some(gamepads) = self.gamepads {
                        for gamepad in gamepads.iter() {
                            if let Some(buttons) = self.gamepad_buttons {
                                if buttons.just_released(GamepadButton::new(gamepad, *button)) {
                                    return Ok(true);
                                }
                            } else {
                                return Err(InputError::BindNotFound);
                            }
                        }
                        Ok(false)
                    } else {
                        Err(InputError::BindNotFound)
                    }
                }
            }?;
            if is_pressed {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
