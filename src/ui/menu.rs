use bevy::prelude::*;
use bevy_alt_ui_navigation_lite::prelude::*;
use sickle_ui::prelude::*;

use crate::{
    assets::CoreAssets,
    ui::{
        widgets::{UiButtonWidget, UiTextWidget},
        UiRootContainer,
    },
    GameState,
};

const UI_GAP: Val = Val::Px(16.);
const BACKGROUND_COLOR: Color = Color::srgba(0.0, 0.05, 0.1, 0.8);

// ······
// Plugin
// ······

// Main menu
// Provides options and a way to start/resume the game
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MenuState>()
            .enable_state_scoped_entities::<MenuState>()
            .add_systems(OnEnter(MenuState::Main), open_menu)
            .add_systems(
                OnEnter(MenuState::Options),
                open_options,
            )
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

#[derive(Component)]
pub struct ButtonExit;

// Query to differentiate between button types
type Buttons<'a> = (
    Option<&'a ButtonPlay>,
    Option<&'a ButtonOptions>,
    Option<&'a ButtonExit>,
);

// ·······
// Systems
// ·······

// Main menu screen
// This builds the menu on top of the Ui root node using the widgets we defined
// It is state scoped, so once the main menu state exits, it will be cleaned automatically
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

            column.button(ButtonExit, |button| {
                button.text("Exit".into(), assets.font.clone());
            });
        })
        .insert(StateScoped(MenuState::Main))
        .style()
        .background_color(BACKGROUND_COLOR);
}

// Options menu screen
fn open_options(
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

            column.title("Options".into(), assets.font.clone());

            column.button(ButtonOptions, |button| {
                button.text("Option 1".into(), assets.font.clone());
            });

            column.button(ButtonOptions, |button| {
                button.text("Option 2".into(), assets.font.clone());
            });

            column.button(ButtonExit, |button| {
                button.text("Back".into(), assets.font.clone());
            });
        })
        .insert(StateScoped(MenuState::Options))
        .style()
        .background_color(BACKGROUND_COLOR);
}

// This checks NavEvents and reacts to them
// They can happen when an action on a button is requested, or when the user wants to go back
// We are not using the bevy Interaction system, we are using NavEvents instead for
// accesibility and convenience
fn handle_buttons(
    buttons: Query<Buttons>,
    mut next_state: ResMut<NextState<GameState>>,
    curr_menu_state: Res<State<MenuState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut nav_event_reader: EventReader<NavEvent>,
    mut nav_request_writer: EventWriter<NavRequest>,
    mut app_exit_writer: EventWriter<AppExit>,
) {
    for event in nav_event_reader.read() {
        match event {
            // Take an action on the focused element
            NavEvent::NoChanges {
                from,
                request: NavRequest::Action,
            } => {
                // If the action matches one of our buttons
                let Ok((play, options, exit)) = buttons.get(*from.first()) else {
                    continue;
                };

                // Do something based on the button type
                if play.is_some() {
                    button_play(&mut next_state);
                }
                if options.is_some() {
                    button_options(&curr_menu_state, &mut next_menu_state);
                }
                if exit.is_some() {
                    button_exit(
                        &curr_menu_state,
                        &mut nav_request_writer,
                        &mut app_exit_writer,
                    );
                }
            },
            // Go back to the previous menu or go back to playing
            NavEvent::NoChanges {
                from: _,
                request: NavRequest::Cancel,
            } => match curr_menu_state.get() {
                MenuState::Main => next_state.set(GameState::Play),
                MenuState::Options => next_menu_state.set(MenuState::Main),
            },
            _ => {},
        }
    }
}

// ·······
// Helpers
// ·······

fn button_play(next_state: &mut NextState<GameState>) {
    next_state.set(GameState::Play);
}

fn button_options(curr_menu_state: &State<MenuState>, next_menu_state: &mut NextState<MenuState>) {
    if curr_menu_state.get() == &MenuState::Main {
        next_menu_state.set(MenuState::Options);
    }
}

fn button_exit(
    curr_menu_state: &State<MenuState>,
    nav_request_writer: &mut EventWriter<NavRequest>,
    app_exit_writer: &mut EventWriter<AppExit>,
) {
    match curr_menu_state.get() {
        MenuState::Main => {
            app_exit_writer.send(AppExit::Success);
        },
        _ => {
            nav_request_writer.send(NavRequest::Cancel);
        },
    }
}
