use crate::prelude::*;

mod main;

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<MenuState>()
        .enable_state_scoped_entities::<MenuState>()
        .add_plugins(main::plugin);
}

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
