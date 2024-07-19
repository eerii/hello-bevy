//! Menu module

use bevy::prelude::*;
use bevy_alt_ui_navigation_lite::prelude::*;

#[cfg(feature = "tts")]
use crate::data::{GameOptions, Persistent};
use crate::GameState;

mod main;
mod mappings;
mod options;

const UI_GAP: Val = Val::Px(16.);

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
            .add_systems(OnEnter(MenuState::Main), main::open)
            .add_systems(
                OnEnter(MenuState::Options),
                options::open,
            )
            .add_systems(
                OnEnter(MenuState::Mappings),
                mappings::open,
            )
            .add_systems(
                OnEnter(MenuState::Refresh),
                refresh_state,
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
pub(super) enum MenuState {
    /// Main menu screen, allows to play or exit the game and access further
    /// options
    #[default]
    Main,
    /// Menu screen to customize game options
    /// (There are no options at the moment)
    Options,
    /// Menu screen to view keys assigned to actions
    Mappings,
    /// Refresh the menu state by exiting and entering again
    /// Uses `MenuRefreshState` to indicate the next state
    Refresh,
}

// ··········
// Components
// ··········

/// Marker for the menu buttons
#[derive(Component)]
enum MenuButton {
    /// Start or resume the game, transitions to `GameState::Play`
    Play,
    /// See other options, transitions to `MenuState::Options`
    Options,
    /// Toggle text to speech
    #[cfg(feature = "tts")]
    Speech,
    /// Remap keys, transitions to `MenuState::Mappings`
    Mappings,
    /// Exit the game or go back a menu
    ExitOrBack,
    /// Placeholder button, does nothing
    #[allow(dead_code)]
    None,
}

/// Indicates what is the state being refreshed
#[derive(Component)]
struct MenuRefreshState(MenuState);

// ·······
// Systems
// ·······

/// This checks NavEvents and reacts to them
/// They can happen when an action on a button is requested, or when the user
/// wants to go back We are not using the bevy Interaction system, we are using
/// NavEvents instead for accesibility and convenience
fn handle_buttons(
    #[cfg(feature = "tts")] mut cmd: Commands,
    buttons: Query<&MenuButton>,
    #[cfg(feature = "tts")] mut options: ResMut<Persistent<GameOptions>>,
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
                let Ok(buttons) = buttons.get(*from.first()) else { continue };

                // Do something based on the button type
                match buttons {
                    MenuButton::Play => {
                        next_state.set(GameState::Play);
                    },
                    MenuButton::Options => {
                        next_menu_state.set(MenuState::Options);
                    },
                    #[cfg(feature = "tts")]
                    MenuButton::Speech => {
                        let _ = options.update(|options| {
                            options.text_to_speech = !options.text_to_speech;
                        });
                        next_menu_state.set(MenuState::Refresh);
                        cmd.spawn((
                            MenuRefreshState(MenuState::Options),
                            StateScoped(MenuState::Refresh),
                        ));
                    },
                    MenuButton::Mappings => {
                        next_menu_state.set(MenuState::Mappings);
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
                MenuState::Mappings => next_menu_state.set(MenuState::Options),
                MenuState::Refresh => {},
            },
            _ => {},
        }
    }
}

fn refresh_state(
    refresh_state: Query<&MenuRefreshState>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
) {
    let Ok(next) = refresh_state.get_single() else {
        next_menu_state.set(MenuState::default());
        return;
    };
    next_menu_state.set(next.0.clone());
}
