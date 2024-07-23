use bevy::prelude::*;

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
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PlaySet {
    /// Tick timers and other `Time` based systems.
    Timers,
    /// Systems that handle input.
    Input,
    /// General gameplay systems.
    Update,
    /// Systems that read sent events before this.
    ReadEvents,
    /// Animations and other systems that happen after everything is calculated.
    Animation,
}
