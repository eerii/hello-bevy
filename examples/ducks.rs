#![allow(clippy::too_many_arguments)]

use bevy::{
    core_pipeline::prepass::DepthPrepass,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_persistent::Persistent;
use hello_bevy::{
    init_camera, GameAppConfig, GameCamera, GameOptions, GamePlugin, GameState, InputMovement,
    Keybinds,
};

const CAMERA_OFFSET: Vec3 = Vec3::new(0., 5., -8.);
const PLAYER_VELOCITY: f32 = 10.;

fn main() {
    App::new()
        .insert_resource(GameAppConfig {
            initial_window_res: Vec2::new(800., 600.).into(),
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
        app.add_plugins(MaterialPlugin::<CustomMaterial>::default())
            .add_systems(
                OnEnter(GameState::Play),
                init_sample.after(init_camera).run_if(run_once()),
            )
            .add_systems(
                Update,
                (update_player, camera_follow).run_if(in_state(GameState::Play)),
            )
            .insert_resource(Msaa::Off);
    }
}

// ··········
// Components
// ··········

#[derive(Component)]
struct Player;

// ·········
// Materials
// ·········

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/simple.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/simple.wgsl".into()
    }
}

// ·······
// Systems
// ·······

fn init_sample(
    mut cmd: Commands,
    opts: Res<Persistent<GameOptions>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut cam: Query<(Entity, &mut Camera, &mut Transform), With<GameCamera>>,
) {
    // Floor
    cmd.spawn(MaterialMeshBundle {
        mesh: meshes.add(shape::Cube::default().into()),
        material: materials.add(CustomMaterial {
            color: Color::hex("#207345").unwrap(),
        }),
        transform: Transform::from_xyz(0., -2.0, 0.).with_scale(Vec3::new(30., 1., 30.)),
        ..default()
    });

    // Player
    cmd.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(shape::Capsule::default().into()),
            material: materials.add(CustomMaterial {
                color: opts.color.light,
            }),
            ..default()
        },
        Player,
    ));

    if let Ok((entity, mut cam, mut trans)) = cam.get_single_mut() {
        trans.translation = CAMERA_OFFSET;
        trans.look_at(Vec3::ZERO, Vec3::Y);
        cam.msaa_writeback = false;
        cmd.entity(entity).insert(DepthPrepass);
    }
}

fn update_player(
    time: Res<Time>,
    movement: Res<InputMovement>,
    keybinds: Res<Persistent<Keybinds>>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut trans) = player.get_single_mut() else { return };

    let input = Vec2::new(
        keybinds.x_axis.get(&movement),
        keybinds.y_axis.get(&movement),
    )
    .normalize_or_zero();

    // This is terrible
    trans.translation.x -= input.x * PLAYER_VELOCITY * time.delta_seconds();
    trans.translation.z += input.y * PLAYER_VELOCITY * time.delta_seconds();
    trans.translation.y = time.elapsed_seconds().sin() * 0.2;
}

fn camera_follow(
    player: Query<&Transform, With<Player>>,
    mut cam: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
) {
    let Ok(player) = player.get_single() else { return };
    let Ok(mut trans) = cam.get_single_mut() else { return };

    // This is even worse
    trans.translation = trans
        .translation
        .lerp(player.translation + CAMERA_OFFSET, 0.1);
    //trans.look_at(player.translation, Vec3::Y);
}
