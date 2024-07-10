//! Options menu submodule

use bevy::prelude::*;
use sickle_ui::prelude::*;

#[cfg(feature = "tts")]
use crate::data::{GameOptions, Persistent};
use crate::{
    assets::CoreAssets,
    ui::{
        menu::{MenuButton, MenuState, BACKGROUND_COLOR, UI_GAP},
        widgets::{UiButtonWidget, UiTextWidget},
        UiRootContainer,
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
    #[cfg(feature = "tts")] options: Res<Persistent<GameOptions>>,
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

            // TODO: Refactor into propper options
            #[cfg(feature = "tts")]
            column.button(MenuButton::Speech, |button| {
                button.text(
                    format!(
                        "Speech: {}",
                        if options.text_to_speech { "Enabled" } else { "Disabled" }
                    ),
                    assets.font.clone(),
                );
            });

            column.button(MenuButton::Mappings, |button| {
                button.text("Mappings".into(), assets.font.clone());
            });

            column.button(MenuButton::ExitOrBack, |button| {
                button.text("Back".into(), assets.font.clone());
            });
        })
        .insert(StateScoped(MenuState::Options))
        .style()
        .background_color(BACKGROUND_COLOR);
}
