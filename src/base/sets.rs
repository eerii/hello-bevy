//! `SystemSet`s in bevy allow to group systems inside an `Schedule`, allowing
//! for global ordering between each group. This is very useful since some
//! systems need to happen before others, but it is not good to abuse it to
//! allow paralellization.

use crate::prelude::*;

/// Adds the `PlaySet` to the `App`.
pub(super) fn plugin(app: &mut App) {
    app.configure_sets(
        Update,
        (
            PlaySet::Timers,
            PlaySet::Update,
            PlaySet::ReadEvents,
            PlaySet::Animation,
        )
            .chain()
            .run_if(in_state(GameState::Play)),
    )
    .insert_resource(GameSetup);
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

/// Resource used to reset the state of `on_setup`.
#[derive(Resource)]
pub struct GameSetup;

/// This function is very similar to `run_once`, but it allows to reset the
/// state. Its purpose is to use it with systems that need to run only once for
/// setup purposes, but that can be triggered again in the future, for example,
/// when starting a new game.
///
/// It is an alternative to `StateScoped` that provides greater flexibility for
/// entities, it is recommended to use both depending on the needs of each
/// specific entities. For example, entities that should persist when pausing
/// the game should use `on_setup`, but entities that don't store any state and
/// can be recreated could use `StateScoped` since it simplifies the logic.
///
/// # Examples
///
/// ```
/// use game::prelude::*;
///
/// fn plugin(app: &mut App) {
///     app.add_systems(OnEnter(GameState::Play), init.run_if(on_setup()))
///         .add_systems(OnEnter(GameState::End), reset);
/// }
///
/// fn init() {
///     info!("hi! uwu");
/// }
///
/// fn reset(mut cmd: Commands, mut state: ResMut<NextState<GameState>>) {
///     cmd.reset_setup();
///     state.set(GameState::Play);
/// }
/// ```
pub fn on_setup() -> impl FnMut(Option<Res<'_, GameSetup>>) -> bool + Clone {
    resource_added::<GameSetup>
}

/// Convenience function that allows to call `cmd.reset_game(...)`.
pub trait SetupCommandExt {
    /// Readds the `GameSetup` resource to trigger again all of the systems
    /// conditioned by `on_setup`.
    fn reset_setup(&mut self) -> &mut Self;
}

impl SetupCommandExt for Commands<'_, '_> {
    fn reset_setup(&mut self) -> &mut Self {
        self.remove_resource::<GameSetup>();
        self.insert_resource(GameSetup);
        self
    }
}
