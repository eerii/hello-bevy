#![allow(clippy::too_many_arguments)]

use bevy::{
    core_pipeline::bloom::BloomSettings,
    prelude::*,
};
use bevy_persistent::Persistent;
use hello_bevy::{
    GameAppConfig,
    GameCamera,
    GameOptions,
    GamePlugin,
    GameState,
};

fn main() {
    App::new()
        .insert_resource(GameAppConfig {
            initial_window_res: Vec2::new(600., 600.),
            initial_game_res: Vec2::new(64., 64.),
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
        app.add_systems(
            PreUpdate,
            init_sample.run_if(in_state(GameState::Play).and_then(run_once())),
        )
        .add_systems(
            Update,
            update_sample.run_if(in_state(GameState::Play)),
        );
    }
}

// ··········
// Components
// ··········

#[derive(Component)]
struct Sphere;

// ·······
// Systems
// ·······

fn init_sample(
    mut cmd: Commands,
    opts: Res<Persistent<GameOptions>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cam: Query<(Entity, &mut Camera, &mut Transform), With<GameCamera>>,
) {
    // Background
    cmd.spawn(PbrBundle {
        mesh: meshes.add(shape::Quad::new(Vec2::splat(100.)).into()),
        material: materials.add(opts.color.dark.into()),
        transform: Transform::from_xyz(0., 0., -5.),
        ..default()
    });

    // Sphere
    cmd.spawn((
        PbrBundle {
            mesh: meshes.add(
                shape::Icosphere::default()
                    .try_into()
                    .expect("icosphere should exist"),
            ),
            material: materials.add(StandardMaterial {
                emissive: opts.color.mid.into(),
                ..default()
            }),
            ..default()
        },
        Sphere,
    ));

    // Lights
    cmd.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.,
            shadows_enabled: true,
            color: Color::rgb(1.0, 0.9, 0.5),
            ..default()
        },
        transform: Transform::from_xyz(-4., 4., 4.),
        ..default()
    });

    cmd.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 200.,
            shadows_enabled: true,
            color: Color::rgb(0.5, 0.9, 1.0),
            ..default()
        },
        transform: Transform::from_xyz(4., -8., 4.),
        ..default()
    });

    if let Ok((entity, mut cam, mut trans)) = cam.get_single_mut() {
        cam.hdr = true;
        trans.translation.z = 6.;
        cmd.entity(entity).insert(BloomSettings::default());
    }
}

fn update_sample(time: Res<Time>, mut sphere: Query<&mut Transform, With<Sphere>>) {
    for mut trans in sphere.iter_mut() {
        trans.translation.y = (time.elapsed_seconds() * 0.5).sin();
    }
}
