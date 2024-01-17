#[cfg(feature = "pixel_perfect")]
use bevy::window::WindowResized;
use bevy::{prelude::*, render::view::RenderLayers};

// use bevy_pixels::PixelCameraPlugin;
use crate::{GameAppConfig, GameState};

// ······
// Plugin
// ······

// TODO: Move the pixel perfect camera to a different crate

// Camera
// WIP, add comments after deciding on a final design
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Play), init_camera)
            .add_systems(OnExit(GameState::Play), pause_camera);

        // #[cfg(feature = "pixel_perfect")]
        // app.add_plugins(PixelCameraPlugin).add_systems(
        //     Update,
        //     on_resize.run_if(in_state(GameState::Play)),
        // );
    }
}

// ··········
// Components
// ··········

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct FinalCamera;

// ·······
// Systems
// ·······

pub fn init_camera(
    mut cmd: Commands,
    _app_config: Res<GameAppConfig>,
    mut _images: ResMut<Assets<Image>>,
    mut cam: Query<&mut Camera, With<GameCamera>>,
) {
    if let Ok(mut cam) = cam.get_single_mut() {
        cam.is_active = true;
        return;
    }

    let camera = Camera::default();

    #[cfg(not(feature = "3d_camera"))]
    let camera_bundle = Camera2dBundle {
        camera,
        ..default()
    };

    #[cfg(feature = "3d_camera")]
    let camera_bundle = Camera3dBundle {
        camera,
        transform: Transform::from_xyz(0.0, 0.0, 10.0),
        ..default()
    };

    cmd.spawn((
        camera_bundle,
        GameCamera,
        RenderLayers::layer(0),
        #[cfg(not(feature = "pixel_perfect"))]
        FinalCamera,
    ));
}

fn pause_camera(mut cam: Query<&mut Camera, With<GameCamera>>) {
    if let Ok(mut cam) = cam.get_single_mut() {
        cam.is_active = false;
    }
}
