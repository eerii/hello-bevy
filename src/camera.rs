use bevy::prelude::*;

use crate::GameState;

// ······
// Plugin
// ······

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Play), init_camera)
            .add_systems(OnExit(GameState::Play), pause_camera);
    }
}

// ··········
// Components
// ··········

#[derive(Component)]
pub struct GameCamera;

// ·······
// Systems
// ·······

fn init_camera(mut cmd: Commands, mut cam: Query<&mut Camera, With<GameCamera>>) {
    if let Ok(mut cam) = cam.get_single_mut() {
        cam.is_active = true;
    } else {
        // TODO: Option for 3d camera
        cmd.spawn((Camera2dBundle::default(), GameCamera));
    }
}

fn pause_camera(mut cam: Query<&mut Camera, With<GameCamera>>) {
    if let Ok(mut cam) = cam.get_single_mut() {
        cam.is_active = false;
    }
}
