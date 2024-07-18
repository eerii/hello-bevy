//! Main menu submodule

use bevy::prelude::*;
use sickle_ui::prelude::*;

use crate::{
    assets::CoreAssets,
    camera::BACKGROUND_LUMINANCE,
    data::{GameOptions, Persistent},
    ui::{
        menu::{MenuButton, MenuState},
        widgets::{UiButtonWidget, UiTextWidget},
        UiRootContainer, UI_GAP,
    },
};

// ·······
// Systems
// ·······

/// Main menu screen
/// This builds the menu on top of the Ui root node using the widgets we defined
/// It is state scoped, so once the main menu state exits, it will be cleaned
/// automatically
pub(super) fn open(
    mut cmd: Commands,
    root: Query<Entity, With<UiRootContainer>>,
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

            column.title("Title".into(), assets.font.clone());

            column.button(MenuButton::Play, |button| {
                button.text("Play".into(), assets.font.clone());
            });

            column.button(MenuButton::Options, |button| {
                button.text("Options".into(), assets.font.clone());
            });

            #[cfg(not(target_arch = "wasm32"))]
            column.button(MenuButton::ExitOrBack, |button| {
                button.text("Exit".into(), assets.font.clone());
            });
        })
        .insert(StateScoped(MenuState::Main))
        .style()
        .background_color(
            options
                .base_color
                .with_luminance(BACKGROUND_LUMINANCE)
                .with_alpha(0.8),
        );
}
