#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;
use bevy_persistent::Persistent;
use hello_bevy::{
    GameAppConfig, GameCamera, GameOptions, GamePlugin, GameState, InputMovement, Keybinds,
};

const CAMERA_OFFSET: Vec3 = Vec3::new(0., 5., -8.);
const PLAYER_VELOCITY: f32 = 10.;

fn main() {
    App::new()
        .insert_resource(GameAppConfig {
            initial_window_res: Vec2::new(256. * 4., 192. * 4.).into(),
            // Quick hack to make the game run in my potato
            #[cfg(feature = "pixel_perfect")]
            initial_game_res: Vec2::new(256. * 2., 192. * 2.),
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
            (update_player, camera_follow).run_if(in_state(GameState::Play)),
        );
    }
}

// ··········
// Components
// ··········

#[derive(Component)]
struct Player;

// ·······
// Systems
// ·······

fn init_sample(
    mut cmd: Commands,
    opts: Res<Persistent<GameOptions>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cam: Query<(&mut Camera, &mut Transform), With<GameCamera>>,
) {
    // Floor
    cmd.spawn(PbrBundle {
        mesh: meshes.add(shape::Cube::default().into()),
        material: materials.add(StandardMaterial {
            base_color: Color::hex("#207345").unwrap(),
            ..default()
        }),
        transform: Transform::from_xyz(0., -1.5, 0.).with_scale(Vec3::new(30., 1., 30.)),
        ..default()
    });

    // Player
    cmd.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Capsule::default().into()),
            material: materials.add(StandardMaterial {
                base_color: opts.color.light,
                perceptual_roughness: 0.9,
                metallic: 0.2,
                ..default()
            }),
            ..default()
        },
        Player,
    ));

    // Directional light
    cmd.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: false,
            illuminance: 20000.,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            1.0,
            3.4,
            0.,
        )),
        ..default()
    });

    if let Ok((mut cam, mut trans)) = cam.get_single_mut() {
        trans.translation = CAMERA_OFFSET;
        trans.look_at(Vec3::ZERO, Vec3::Y);
        cam.msaa_writeback = false;
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
