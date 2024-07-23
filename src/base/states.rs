use bevy::prelude::*;

/// Adds the `GameState` to the `App`.
/// Also enables `StateScoped` for this state so enitities can be automatically cleaned up.
pub(super) fn plugin(app: &mut App) {
    app.insert_state(GameState::default())
        .enable_state_scoped_entities::<GameState>();
}

/// Indicates at which point the game is. Very useful for controlling which
/// systems run when (in_state) and to create transitions (OnEnter/OnExit)
/// You can also scope entities to a state with StateScoped, and they will
/// be deleted automatically when the state ends
#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    /// The game starts on the `Startup` state.
    /// It runs before *anything*, including the `Startup` schedule.
    /// It inmediately transitions to `Loading`.
    #[default]
    Startup,
    /// Handles splash screens and assets.
    /// The game stays here until all of the assets are ready.
    Loading,
    /// The main menu of the game. All of the game systems are paused.
    Menu,
    /// Main state representing the actual gameplay.
    Play,
    /// End of the `Play` state.
    /// It can be used to restart the game or handle win/lose conditions.
    End,
}
