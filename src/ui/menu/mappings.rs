//! Key mappings menu submodule

// TODO: Allow remapping
// TODO: Mouse and other icons

use bevy::{prelude::*, reflect::Enum};
use itertools::Itertools;
use leafwing_input_manager::prelude::*;
use sickle_ui::prelude::*;

use crate::{
    assets::CoreAssets,
    camera::BACKGROUND_LUMINANCE,
    data::{GameOptions, Persistent},
    input::Action,
    ui::{
        menu::{MenuButton, MenuState, UI_GAP},
        navigation::FocusableHoverFill,
        widgets::{UiButtonWidget, UiImageWidget, UiOptionRowWidget, UiTextWidget},
        UiRootContainer,
    },
};

// ·······
// Systems
// ·······

/// Mappings menu screen
pub(super) fn open(
    mut cmd: Commands,
    root: Query<Entity, With<UiRootContainer>>,
    input_map: Query<&InputMap<Action>>,
    asset_server: Res<AssetServer>,
    assets: Res<CoreAssets>,
    options: Res<Persistent<GameOptions>>,
) {
    let Ok(root) = root.get_single() else { return };

    cmd.ui_builder(root)
        .column(|column| {
            column
                .style()
                .width(Val::Percent(100.))
                .align_items(AlignItems::Center)
                .justify_content(JustifyContent::Center)
                .row_gap(UI_GAP);

            column.title("Mappings".into(), assets.font.clone());

            if let Ok(input_map) = input_map.get_single() {
                for (action, maps) in input_map
                    .iter()
                    .sorted_by_key(|(&a, _)| a.variant_name().to_string())
                {
                    let mut row = column.option_row(
                        MenuButton::None,
                        action.variant_name().into(),
                        assets.font.clone(),
                    );

                    for map in maps {
                        row_mapping(
                            (**map).as_reflect(),
                            &mut row,
                            &asset_server,
                        );
                    }
                }
            };

            column.button(MenuButton::ExitOrBack, |button| {
                button.text("Back".into(), assets.font.clone());
            });
        })
        .insert(StateScoped(MenuState::Mappings))
        .style()
        .background_color(options.base_color.with_luminance(BACKGROUND_LUMINANCE));
}

// ·······
// Helpers
// ·······

fn row_mapping(map: &dyn Reflect, row: &mut UiBuilder<Entity>, asset_server: &AssetServer) {
    let prompts = if let Some(key) = map.downcast_ref::<KeyCode>() {
        [keycode_prompt(key)].iter().flatten().cloned().collect()
    } else if let Some(button) = map.downcast_ref::<GamepadButtonType>() {
        [gamepad_button_prompt(button)]
            .iter()
            .flatten()
            .cloned()
            .collect()
    } else if let Some(dpad) = map.downcast_ref::<KeyboardVirtualDPad>() {
        dpad.raw_inputs()
            .keycodes
            .iter()
            .filter_map(keycode_prompt)
            .collect()
    } else if let Some(stick) = map.downcast_ref::<GamepadStick>() {
        // Ugly workaround because the methods are private
        let name = if format!("{:?}", stick).contains("Right") { "r" } else { "l" };

        vec![format!(
            "icons/input/controller/switch_stick_{}.png",
            name
        )]
    } else {
        vec!["icons/input/keyboard/keyboard_0.png".into()]
    };

    for prompt in prompts {
        // Dynamic loading to avoid having all icons in memory
        row.option_button(|button| {
            button.image(asset_server.load(&prompt));
        })
        .insert(BorderRadius::all(Val::Px(16.)))
        .insert(BorderColor::from(Srgba::NONE))
        .insert(BackgroundColor::from(Srgba::NONE))
        .insert(FocusableHoverFill)
        .style()
        .width(Val::Px(64.))
        .height(Val::Px(64.));
    }
}

fn keycode_prompt(key: &KeyCode) -> Option<String> {
    let name = match key {
        KeyCode::Digit0 => todo!(),
        KeyCode::Digit1 => todo!(),
        KeyCode::Digit2 => todo!(),
        KeyCode::Digit3 => todo!(),
        KeyCode::Digit4 => todo!(),
        KeyCode::Digit5 => todo!(),
        KeyCode::Digit6 => todo!(),
        KeyCode::Digit7 => todo!(),
        KeyCode::Digit8 => todo!(),
        KeyCode::Digit9 => todo!(),
        KeyCode::KeyA => "a",
        KeyCode::KeyB => "b",
        KeyCode::KeyC => "c",
        KeyCode::KeyD => "d",
        KeyCode::KeyE => "e",
        KeyCode::KeyF => "f",
        KeyCode::KeyG => "g",
        KeyCode::KeyH => "h",
        KeyCode::KeyI => "i",
        KeyCode::KeyJ => "j",
        KeyCode::KeyK => "k",
        KeyCode::KeyL => "l",
        KeyCode::KeyM => "m",
        KeyCode::KeyN => "n",
        KeyCode::KeyO => "o",
        KeyCode::KeyP => "p",
        KeyCode::KeyQ => "q",
        KeyCode::KeyR => "r",
        KeyCode::KeyS => "s",
        KeyCode::KeyT => "t",
        KeyCode::KeyU => "u",
        KeyCode::KeyV => "v",
        KeyCode::KeyW => "w",
        KeyCode::KeyX => "x",
        KeyCode::KeyY => "y",
        KeyCode::KeyZ => "z",
        KeyCode::Enter => "enter",
        KeyCode::Space => "space",
        KeyCode::ArrowDown => "arrow_down",
        KeyCode::ArrowLeft => "arrow_left",
        KeyCode::ArrowRight => "arrow_right",
        KeyCode::ArrowUp => "arrow_up",
        KeyCode::Escape => "escape",
        KeyCode::F1 => "f1",
        KeyCode::F2 => "f2",
        KeyCode::F3 => "f3",
        KeyCode::F4 => "f4",
        KeyCode::F5 => "f5",
        KeyCode::F6 => "f6",
        KeyCode::F7 => "f7",
        KeyCode::F8 => "f8",
        KeyCode::F9 => "f9",
        KeyCode::F10 => "f10",
        KeyCode::F11 => "f11",
        KeyCode::F12 => "f12",
        _ => "",
    };

    if name.is_empty() {
        None
    } else {
        Some(format!(
            "icons/input/keyboard/keyboard_{}.png",
            name
        ))
    }
}

fn gamepad_button_prompt(button: &GamepadButtonType) -> Option<String> {
    let name = match button {
        GamepadButtonType::South => "buttons_down",
        GamepadButtonType::East => "buttons_right",
        GamepadButtonType::North => "buttons_up",
        GamepadButtonType::West => "buttons_left",
        GamepadButtonType::LeftTrigger => "button_l",
        GamepadButtonType::RightTrigger => "button_r",
        GamepadButtonType::LeftThumb => "button_zl",
        GamepadButtonType::RightThumb => "button_zr",
        GamepadButtonType::Select => "button_plus",
        GamepadButtonType::Start => "button_home",
        GamepadButtonType::Mode => "button_minux",
        GamepadButtonType::DPadUp => "dpad_up",
        GamepadButtonType::DPadDown => "dpad_down",
        GamepadButtonType::DPadLeft => "dpad_left",
        GamepadButtonType::DPadRight => "dpad_right",
        _ => "",
    };

    if name.is_empty() {
        None
    } else {
        Some(format!(
            "icons/input/controller/switch_{}.png",
            name
        ))
    }
}
