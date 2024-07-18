//! Options menu submodule

use bevy::prelude::*;
use sickle_ui::prelude::*;

use crate::{
    assets::CoreAssets,
    camera::BACKGROUND_LUMINANCE,
    data::{GameOptions, Persistent},
    ui::{
        menu::{MenuButton, MenuState},
        widgets::{UiButtonWidget, UiOptionRowWidget, UiTextWidget},
        UiRootContainer, UI_GAP,
    },
};

// ·······
// Systems
// ·······

/// Options menu screen
pub(super) fn open(
    mut cmd: Commands,
    root: Query<Entity, With<UiRootContainer>>,
    assets: Res<CoreAssets>,
    options: Res<Persistent<GameOptions>>,
) {
    let Ok(root) = root.get_single() else {
        return;
    };

    cmd.ui_builder(root)
        .column(|column| {
            column
                .style()
                .width(Val::Percent(100.))
                .align_items(AlignItems::Center)
                .justify_content(JustifyContent::Center)
                .row_gap(UI_GAP);

            column.title("Options".into(), assets.font.clone());

            #[cfg(feature = "tts")]
            column
                .option_row(
                    MenuButton::Speech,
                    "Speech".into(),
                    assets.font.clone(),
                )
                .insert(crate::ui::menu::Focusable::new().prioritized())
                .option_button(|button| {
                    button.text(
                        (if options.text_to_speech { "Enabled" } else { "Disabled" }).into(),
                        assets.font.clone(),
                    );
                });

            column
                .option_row(
                    MenuButton::Mappings,
                    "Mappings".into(),
                    assets.font.clone(),
                )
                .option_button(|button| {
                    button.text("View".into(), assets.font.clone());
                });

            column.button(MenuButton::ExitOrBack, |button| {
                button.text("Back".into(), assets.font.clone());
            });
        })
        .insert(StateScoped(MenuState::Options))
        .style()
        .background_color(options.base_color.with_luminance(BACKGROUND_LUMINANCE));
}
