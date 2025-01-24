//! `State`s are a FSM that allows to differentiate between scenarios and
//! conditionally run systems based on which is active.
//!
//! `SystemSet`s in bevy allow to group systems inside an `Schedule`, allowing
//! for global ordering between each group. This is very useful since some
//! systems need to happen before others, but it is not good to abuse it to
//! allow paralellization.

use crate::prelude::*;

/// Adds `GameState` and `PlaySet` to the `App`.
/// Also enables `StateScoped` for this state so enitities can be automatically
/// cleaned up.
pub(super) fn plugin(app: &mut App) {
    app.insert_state(GameState::default())
        .enable_state_scoped_entities::<GameState>()
        .configure_sets(
            Update,
            (
                PlaySet::Timers,
                PlaySet::Update,
                PlaySet::ReadEvents,
                PlaySet::Animation,
            )
                .chain()
                .run_if(in_state(GameState::Play)),
        );

    app.add_systems(PostUpdate, init.run_if(in_state(GameState::Startup)));
}

/// Indicates at which point the game is. Very useful for controlling which
/// systems run when (`in_state`) and to create transitions (`OnEnter`/`OnExit`)
/// You can also scope entities to a state with StateScoped, and they will
/// be deleted automatically when the state ends
#[derive(Default, States, Std!)]
pub enum GameState {
    /// The game starts on the `Startup` state.
    /// It runs before *anything*, including the `Startup` schedule.
    #[default]
    Startup,
    /// The main menu of the game. All of the game systems are paused.
    Menu,
    /// Main state representing the actual gameplay.
    Play,
    /// End of the `Play` state.
    /// It can be used to restart the game or handle win/lose conditions.
    End,
}

/// Main grouping of systems inside the `GameState::Play` state.
/// This allows to easily group systems inside the `Update` schedule.
#[derive(Default, SystemSet, Std!)]
pub enum PlaySet {
    /// Tick timers and other `Time` based systems.
    Timers,
    /// General gameplay systems.
    #[default]
    Update,
    /// Systems that read sent events before this.
    ReadEvents,
    /// Animations and other systems that happen after everything is calculated.
    Animation,
}

// Systems
// ---

/// Basic logic to transition to the play state automatically
/// This should be replaced by the loading system and menu
fn init(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Play);
}
