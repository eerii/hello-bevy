//! Camera module

use bevy::prelude::*;

use crate::{
    data::{init_data, GameOptions, Persistent},
    GameState,
};

/// The luminance of the background color
pub const BACKGROUND_LUMINANCE: f32 = 0.05;

// ······
// Plugin
// ······

/// Camera
/// Creates the main game camera, marked by `GameCamera`
/// Depending on the 3d_camera feature it will be 2d or 3d
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Startup),
            init.after(init_data),
        );
    }
}

// ··········
// Components
// ··········

/// The camera where the game is being rendered
#[derive(Component)]
pub struct GameCamera;

/// The camera that renders everything to the screen
/// It can be different from the GameCamera if doing any kind of
/// deferred rendering or pixel scaling
#[derive(Component)]
pub struct FinalCamera;

// ·······
// Systems
// ·······

/// Creates the main cameras before the game starts
fn init(mut cmd: Commands, options: Res<Persistent<GameOptions>>) {
    let clear_color =
        ClearColorConfig::Custom(options.base_color.with_luminance(BACKGROUND_LUMINANCE));

    #[cfg(not(feature = "3d_camera"))]
    let camera_bundle = Camera2dBundle {
        camera: Camera {
            clear_color,
            ..default()
        },
        ..default()
    };

    #[cfg(feature = "3d_camera")]
    let camera_bundle = Camera3dBundle {
        camera: Camera {
            clear_color,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 10.0),
        ..default()
    };

    cmd.spawn((camera_bundle, GameCamera, FinalCamera));
}
