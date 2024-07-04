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

const UI_GAP: Val = Val::Px(16.);

// ······
// Plugin
// ······

// Ui
// Uses bevy's Ui and Sickle to create beautiful interfaces
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), open_menu)
            .add_systems(
                Update,
                handle_buttons.run_if(in_state(GameState::Menu)),
            );
    }
}

// Menu state
// Useful for navigating submenus
#[derive(SubStates, Debug, Default, Clone, Eq, PartialEq, Hash)]
#[source(GameState = GameState::Menu)]
pub enum MenuState {
    #[default]
    Main,
    Options,
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

    cmd.ui_builder(root)
        .column(|column| {
            column
                .style()
                .width(Val::Percent(100.))
                .align_items(AlignItems::Center)
                .justify_content(JustifyContent::Center)
                .row_gap(UI_GAP);

            column.title("Title".into(), assets.font.clone());

            column.button(ButtonPlay, |button| {
                button.text("Play".into(), assets.font.clone());
            });

            column.button(ButtonOptions, |button| {
                button.text("Options".into(), assets.font.clone());
            });
        })
        .insert(StateScoped(GameState::Menu));
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
