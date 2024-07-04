use bevy::prelude::*;
use sickle_ui::prelude::*;

use crate::{
    assets::CoreAssets,
    ui::{
        widgets::{UiButtonWidget, UiTextWidget, BUTTON_COLOR},
        UiRootContainer,
    },
    GameState,
};

// ······
// Plugin
// ······

// Ui
// Uses bevy's Ui and Sickle to create beautiful interfaces
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), open_menu)
            .add_systems(
                Update,
                handle_buttons.run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit(GameState::Menu), close_menu);
    }
}

// ··········
// Components
// ··········

#[derive(Component)]
pub struct ButtonPlay;

#[derive(Component)]
pub struct ButtonOptions;

// ·······
// Systems
// ·······

fn open_menu(
    mut cmd: Commands,
    root: Query<Entity, With<UiRootContainer>>,
    assets: Res<CoreAssets>,
) {
    let Ok(root) = root.get_single() else {
        return;
    };

    let mut builder = cmd.ui_builder(root);

    builder.title("Title".into(), assets.font.clone());

    builder.button(ButtonPlay, |button| {
        button.text("Play".into(), assets.font.clone());
    });

    builder.button(ButtonOptions, |button| {
        button.text("Options".into(), assets.font.clone());
    });
}

fn handle_buttons(
    mut buttons: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&ButtonPlay>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, play) in buttons.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if let Some(_) = play {
                    next_state.set(GameState::Play);
                }
            },
            Interaction::Hovered => {
                *color = BUTTON_COLOR.lighter(0.1).into();
            },
            Interaction::None => {
                *color = BUTTON_COLOR.into();
            },
        }
    }
}

fn close_menu(mut cmd: Commands, root: Query<Entity, With<UiRootContainer>>) {
    let Ok(root) = root.get_single() else {
        return;
    };

    let Some(mut root) = cmd.get_entity(root) else {
        return;
    };

    root.despawn_descendants();
}
