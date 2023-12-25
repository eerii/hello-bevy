#![allow(clippy::too_many_arguments)]

use bevy::{
    prelude::*,
    render::render_resource::{
        AsBindGroup,
        ShaderRef,
    },
    sprite::{
        Material2d,
        Material2dPlugin,
        MaterialMesh2dBundle,
    },
};
use hello_bevy::{
    GameAppConfig,
    GamePlugin,
    GameState,
};

fn main() {
    App::new()
        .insert_resource(GameAppConfig {
            initial_window_res: Vec2::new(600., 600.),
            ..default()
        })
        .add_plugins((GamePlugin, SampleGamePlugin))
        .run();
}

// ······
// Plugin
// ······

pub struct SampleGamePlugin;

impl Plugin for SampleGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<CustomMaterial>::default())
            .add_systems(
                PreUpdate,
                init_sample.run_if(in_state(GameState::Play).and_then(run_once())),
            );
    }
}

// ·········
// Materials
// ·········

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef { "shaders/test.wgsl".into() }
}

// ·······
// Systems
// ·······

fn init_sample(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // Shader quad
    cmd.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(1000.)),
        material: materials.add(CustomMaterial { color: Color::BLUE }),
        ..default()
    });
}
