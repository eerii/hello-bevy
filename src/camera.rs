#[cfg(feature = "pixel_perfect")]
use bevy::window::WindowResized;
use bevy::{prelude::*, render::view::RenderLayers};

use crate::{GameAppConfig, GameState};

// ······
// Plugin
// ······

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Play), init_camera)
            .add_systems(OnExit(GameState::Play), pause_camera);

        #[cfg(feature = "pixel_perfect")]
        app.add_systems(Update, on_resize.run_if(in_state(GameState::Play)));
    }
}

// ··········
// Components
// ··········

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct FinalCamera;

#[cfg(feature = "pixel_perfect")]
#[derive(Component)]
struct PixelCamera;

#[cfg(feature = "pixel_perfect")]
#[derive(Component)]
struct PixelRenderPlane;

// ·······
// Systems
// ·······

fn init_camera(
    mut cmd: Commands,
    _app_config: Res<GameAppConfig>,
    mut _images: ResMut<Assets<Image>>,
    mut cam: Query<&mut Camera, With<GameCamera>>,
) {
    if let Ok(mut cam) = cam.get_single_mut() {
        cam.is_active = true;
        return;
    }

    let mut _camera = Camera::default();

    #[cfg(feature = "pixel_perfect")]
    {
        use bevy::render::{
            camera::RenderTarget,
            render_resource::{
                Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
            },
        };

        const PIXEL_CAMERA_LAYER: RenderLayers = RenderLayers::layer(1);

        let render_extent = Extent3d {
            width: _app_config.initial_game_res.x as u32,
            height: _app_config.initial_game_res.y as u32,
            depth_or_array_layers: 1,
        };

        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size: render_extent,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            ..default()
        };
        image.resize(render_extent);
        let image_handle = _images.add(image);

        // Main camera
        _camera.target = RenderTarget::Image(image_handle.clone());

        // Pixel upscaling camera
        cmd.spawn((
            Camera2dBundle {
                camera: Camera {
                    order: 1,
                    ..default()
                },
                ..default()
            },
            PIXEL_CAMERA_LAYER,
            PixelCamera,
            FinalCamera,
            UiCameraConfig { show_ui: false },
        ));

        // Plane
        let scale = scale_factor(
            &_app_config.initial_game_res,
            &_app_config.initial_window_res,
        );
        cmd.spawn((
            SpriteBundle {
                texture: image_handle,
                sprite: Sprite {
                    custom_size: Some(_app_config.initial_game_res * scale as f32),
                    ..default()
                },
                ..default()
            },
            PIXEL_CAMERA_LAYER,
            PixelRenderPlane,
        ));
    }

    #[cfg(not(feature = "3d_camera"))]
    let camera_bundle = Camera2dBundle {
        camera: _camera,
        ..default()
    };

    #[cfg(feature = "3d_camera")]
    let camera_bundle = Camera3dBundle {
        camera: _camera,
        transform: Transform::from_xyz(0.0, 0.0, 10.0),
        ..default()
    };

    cmd.spawn((
        camera_bundle,
        GameCamera,
        UiCameraConfig { show_ui: false },
        RenderLayers::layer(0),
        #[cfg(not(feature = "pixel_perfect"))]
        FinalCamera,
    ));
}

#[cfg(feature = "pixel_perfect")]
fn on_resize(
    app_config: Res<GameAppConfig>,
    mut plane: Query<&mut Sprite, With<PixelRenderPlane>>,
    mut event_resize: EventReader<WindowResized>,
) {
    for e in event_resize.read() {
        let scale = scale_factor(&app_config.initial_game_res, &Vec2::new(e.width, e.height));

        for mut sprite in plane.iter_mut() {
            sprite.custom_size = Some(app_config.initial_game_res * scale as f32);
        }
    }
}

fn pause_camera(mut cam: Query<&mut Camera, With<GameCamera>>) {
    if let Ok(mut cam) = cam.get_single_mut() {
        cam.is_active = false;
    }
}

// ·····
// Extra
// ·····

#[cfg(feature = "pixel_perfect")]
fn scale_factor(game: &Vec2, view: &Vec2) -> u32 {
    (view.x / game.x).min(view.y / game.y).floor() as u32
}
