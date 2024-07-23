use bevy::prelude::*;

use crate::base::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Startup), init);
}

/// The camera where the game is being rendered
#[derive(Component)]
pub struct GameCamera;

/// The camera that renders everything to the screen
/// It can be different from the `GameCamera` if doing any kind of
/// deferred rendering or pixel scaling
#[derive(Component)]
pub struct FinalCamera;

/// Spawn the main cameras
fn init(mut cmd: Commands) {
    let camera_bundle = Camera2dBundle::default();
    cmd.spawn((camera_bundle, GameCamera, FinalCamera));
}
