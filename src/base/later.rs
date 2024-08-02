//! Allows to schedule a `Commands` a certain time in the future.
//! Based on the work by dylanj <https://discord.com/channels/691052431525675048/937158127491633152/1266369728402948136>

use bevy::ecs::system::EntityCommands;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, handle_later_commands);
}

/// Scheduled version of a `Commands` that runs after a timer is done.
///
/// # Examples
///
/// ```
/// use game::prelude::*;
///
/// fn system(mut cmd: Commands) {
///     cmd.later(1., |cmd| {
///         info!("Hi from the future :3");
///     });
/// }
/// ```
#[derive(Component)]
pub struct LaterCommand {
    cmd: Box<dyn FnMut(&mut Commands) + Send + Sync + 'static>,
    delay: Timer,
}

impl LaterCommand {
    /// Creates a new `LaterCommand` from a duration in seconds and a closure.
    pub fn new(secs: f32, command: impl FnMut(&mut Commands) + Send + Sync + 'static) -> Self {
        Self {
            cmd: Box::new(command),
            delay: Timer::from_seconds(secs, TimerMode::Once),
        }
    }
}

/// Ticks `LaterCommand` timers and executes the scheduled commands after the
/// timers run out.
fn handle_later_commands(
    mut cmd: Commands,
    mut later: Query<(Entity, &mut LaterCommand)>,
    time: Res<Time>,
) {
    for (entity, mut later) in &mut later {
        if !later.delay.tick(time.delta()).just_finished() {
            continue;
        }
        (later.cmd)(&mut cmd);
        cmd.entity(entity).despawn_recursive();
    }
}

/// Convenience function that allows to call `cmd.later(...)`.
pub trait LaterCommandExt {
    /// Spawns a `LaterCommand` with the specified duration and callback.
    fn later(
        &mut self,
        secs: f32,
        cmd: impl FnMut(&mut Commands) + Send + Sync + 'static,
    ) -> EntityCommands;
}

impl LaterCommandExt for Commands<'_, '_> {
    fn later(
        &mut self,
        secs: f32,
        cmd: impl FnMut(&mut Commands) + Send + Sync + 'static,
    ) -> EntityCommands {
        self.spawn(LaterCommand::new(secs, cmd))
    }
}
