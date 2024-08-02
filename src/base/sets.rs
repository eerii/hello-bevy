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
            PlaySet::Input,
            PlaySet::Update,
            PlaySet::ReadEvents,
            PlaySet::Animation,
        )
            .chain(),
    );
}

/// Main grouping of systems inside the `GameState::Play` state.
/// This allows to easily group systems inside the `Update` schedule.
#[derive(Default, SystemSet, Std!)]
pub enum PlaySet {
    /// Tick timers and other `Time` based systems.
    Timers,
    /// Systems that handle input.
    Input,
    /// General gameplay systems.
    #[default]
    Update,
    /// Systems that read sent events before this.
    ReadEvents,
    /// Animations and other systems that happen after everything is calculated.
    Animation,
}
