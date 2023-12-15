use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
};
use bevy_persistent::Persistent;

use crate::{
    GameOptions,
    GameState,
};

// TODO: Option for pixel perfect upscaling camera

// ······
// Plugin
// ······

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Play), init_camera)
            .add_systems(
                Update,
                change_background.run_if(
                    in_state(GameState::Play).and_then(resource_changed::<
                        Persistent<GameOptions>,
                    >()),
                ),
            )
            .add_systems(OnExit(GameState::Play), pause_camera);
    }
}

// ··········
// Components
// ··········

#[derive(Component)]
struct GameCamera;

// ·······
// Systems
// ·······

fn init_camera(mut cmd: Commands, mut cam: Query<&mut Camera, With<GameCamera>>) {
    if let Ok(mut cam) = cam.get_single_mut() {
        cam.is_active = true;
    } else {
        cmd.spawn((Camera2dBundle::default(), GameCamera));
    }
}

fn change_background(
    opts: Res<Persistent<GameOptions>>,
    mut cam: Query<&mut Camera2d, With<GameCamera>>,
) {
    if let Ok(mut cam) = cam.get_single_mut() {
        cam.clear_color = ClearColorConfig::Custom(opts.color.dark);
    }
}

fn pause_camera(mut cam: Query<&mut Camera, With<GameCamera>>) {
    if let Ok(mut cam) = cam.get_single_mut() {
        cam.is_active = false;
    }
}
