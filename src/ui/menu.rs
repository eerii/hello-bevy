//! The main menu of the game.

use crate::prelude::*;

mod main;
mod mappings;
mod options;

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<MenuState>()
        .enable_state_scoped_entities::<MenuState>()
        .add_plugins((main::plugin, mappings::plugin, options::plugin))
        .add_systems(
            Update,
            (
                handle_back.run_if(in_state(GameState::Menu)),
                handle_pause.run_if(in_state(GameState::Play)),
            ),
        );
}

/// Substate to handle the different menu screens.
#[derive(SubStates, Std!, Default)]
#[source(GameState = GameState::Menu)]
pub enum MenuState {
    /// Main menu screen, used to play or exit the game and access other options
    #[default]
    Main,
    /// Menu screen to customize game options
    Options,
    /// Menu screen to view keys assigned to actions
    Mappings,
}

/// When the `Pause` key is pressed in the menu, go to the previous menu screen.
/// If the player is in the main menu screen, resume the game.
fn handle_back(
    input: Query<&ActionState<Action>>,
    menu_state: Res<State<MenuState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
) {
    let input = single!(input);
    if input.just_pressed(&Action::Pause) {
        match menu_state.get() {
            MenuState::Main => next_state.set(GameState::Play),
            MenuState::Options => next_menu_state.set(MenuState::Main),
            MenuState::Mappings => next_menu_state.set(MenuState::Options),
        }
    }
}

/// When the `Pause` key is pressed while playing, open the menu.
fn handle_pause(input: Query<&ActionState<Action>>, mut next_state: ResMut<NextState<GameState>>) {
    let input = single!(input);
    if input.just_pressed(&Action::Pause) {
        next_state.set(GameState::Menu);
    }
}
