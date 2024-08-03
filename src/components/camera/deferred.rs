//! Deferred rendering for the camera.

use bevy::{
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d,
            TextureDescriptor,
            TextureDimension,
            TextureFormat,
            TextureUsages,
        },
        view::RenderLayers,
    },
    window::WindowResized,
};

use crate::prelude::*;

/// Render layers for high-resolution rendering.
const HIGH_RES_LAYER: RenderLayers = RenderLayers::layer(1);

/// When using the pixel perfect mode, the resolution of the canvas.
#[cfg(feature = "pixel_perfect")]
const CANVAS_RESOLUTION: UVec2 = UVec2::new(64, 64);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Startup), init_canvas.after(super::init))
        .add_systems(Update, on_resize.run_if(on_event::<WindowResized>()));
}

// Components
// ---

/// The intermediate texture that contains the low resulution rendering when
/// using deferred mode.
#[derive(Component)]
struct Canvas;

// Systems
// ---

/// Create the deferred canvas and update the game camera to use it.
fn init_canvas(
    mut cmd: Commands,
    mut camera: Query<(Entity, &mut Camera), With<GameCamera>>,
    mut images: ResMut<Assets<Image>>,
    #[cfg(not(feature = "pixel_perfect"))] window: Query<
        &Window,
        With<bevy::window::PrimaryWindow>,
    >,
) {
    // Calculate the size of the canvas
    #[cfg(not(feature = "pixel_perfect"))]
    let extent = {
        let res = &single!(window).resolution;
        Extent3d {
            width: res.width() as u32,
            height: res.height() as u32,
            ..default()
        }
    };
    #[cfg(feature = "pixel_perfect")]
    let size = Extent3d {
        width: CANVAS_RESOLUTION.x,
        height: CANVAS_RESOLUTION.y,
        ..default()
    };

    // This image is the intermidiate deferred rendering target
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
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
    canvas.resize(size);
    let render_target = images.add(canvas);

    cmd.spawn((
        SpriteBundle {
            texture: render_target.clone(),
            ..default()
        },
        Canvas,
        HIGH_RES_LAYER,
    ));

    let (entity, mut camera) = single_mut!(camera);
    camera.target = RenderTarget::Image(render_target);

    // Create a new final camera that renders the deferred image to the window
    cmd.entity(entity).remove::<FinalCamera>();
    cmd.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 1, // After the main game camera
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            ..default()
        },
        FinalCamera,
        HIGH_RES_LAYER,
    ));
}

/// When the window is resized, changes the canvas size to match.
#[cfg(not(feature = "pixel_perfect"))]
fn on_resize(
    mut resize_events: EventReader<WindowResized>,
    canvas: Query<&Handle<Image>, With<Canvas>>,
    mut images: ResMut<Assets<Image>>,
) {
    let canvas = single!(canvas);
    let Some(image) = images.get_mut(canvas) else { return };
    for event in resize_events.read() {
        image.resize(Extent3d {
            width: event.width as u32,
            height: event.height as u32,
            ..default()
        });
    }
}

/// When the window is resized, changes the projection of the final camera to an
/// integer scale that makes the canvas fit in the window.
#[cfg(feature = "pixel_perfect")]
fn on_resize(
    mut resize_events: EventReader<WindowResized>,
    mut projections: Query<&mut OrthographicProjection, With<FinalCamera>>,
) {
    let mut projection = single_mut!(projections);
    for event in resize_events.read() {
        let h_scale = event.width / CANVAS_RESOLUTION.x as f32;
        let v_scale = event.height / CANVAS_RESOLUTION.y as f32;
        projection.scale = 1. / h_scale.min(v_scale).round();
    }
}
