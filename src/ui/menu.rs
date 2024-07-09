//! Menu module

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

/// Main menu
/// Provides options and a way to start/resume the game
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

/// Menu state
/// Useful for navigating submenus
#[derive(SubStates, Debug, Default, Clone, Eq, PartialEq, Hash)]
#[source(GameState = GameState::Menu)]
pub enum MenuState {
    /// Main menu screen, allows to play or exit the game and access further
    /// options
    #[default]
    Main,
    /// Menu screen to customize game options
    /// (There are no options at the moment)
    Options,
}

// ··········
// Components
// ··········

/// Marker for the menu buttons
#[derive(Component)]
pub enum MenuButton {
    /// Start or resume the game, transitions to `GameState::Play`
    Play,
    /// See other options, transitions to `MenuState::Options`
    Options,
    /// Exit the game or go back a menu
    ExitOrBack,
    /// Placeholder button, does nothing
    None,
}

// ·······
// Systems
// ·······

/// Main menu screen
/// This builds the menu on top of the Ui root node using the widgets we defined
/// It is state scoped, so once the main menu state exits, it will be cleaned
/// automatically
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
        .background_color(BACKGROUND_COLOR);
}

/// Options menu screen
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

            column.button(MenuButton::None, |button| {
                button.text("Option 1".into(), assets.font.clone());
            });

            column.button(MenuButton::None, |button| {
                button.text("Option 2".into(), assets.font.clone());
            });

            column.button(MenuButton::ExitOrBack, |button| {
                button.text("Back".into(), assets.font.clone());
            });
        })
        .insert(StateScoped(MenuState::Options))
        .style()
        .background_color(BACKGROUND_COLOR);
}

/// This checks NavEvents and reacts to them
/// They can happen when an action on a button is requested, or when the user
/// wants to go back We are not using the bevy Interaction system, we are using
/// NavEvents instead for accesibility and convenience
fn handle_buttons(
    buttons: Query<&MenuButton>,
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
                let Ok(buttons) = buttons.get(*from.first()) else {
                    continue;
                };

                // Do something based on the button type
                match buttons {
                    MenuButton::Play => {
                        next_state.set(GameState::Play);
                    },
                    MenuButton::Options => {
                        if curr_menu_state.get() == &MenuState::Main {
                            next_menu_state.set(MenuState::Options);
                        }
                    },
                    MenuButton::ExitOrBack => match curr_menu_state.get() {
                        MenuState::Main => {
                            app_exit_writer.send(AppExit::Success);
                        },
                        _ => {
                            nav_request_writer.send(NavRequest::Cancel);
                        },
                    },
                    MenuButton::None => {},
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
