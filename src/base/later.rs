//! Based on the work by dylanj https://discord.com/channels/691052431525675048/937158127491633152/1266369728402948136

use bevy::ecs::system::EntityCommands;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, handle_later_commands);
}

#[derive(Component)]
pub struct LaterCommand {
    pub cmd: Box<dyn FnMut(&mut Commands) + Send + Sync + 'static>,
    pub delay: Timer,
}

impl LaterCommand {
    pub fn new(secs: f32, command: impl FnMut(&mut Commands) + Send + Sync + 'static) -> Self {
        Self {
            cmd: Box::new(command),
            delay: Timer::from_seconds(secs, TimerMode::Once),
        }
    }
}

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

pub trait LaterCommandExt {
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
