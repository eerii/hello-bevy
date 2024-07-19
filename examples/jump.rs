#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use bevy::{math::bounding::*, prelude::*};
use hello_bevy::{
    assets::CoreAssets,
    camera::GameCamera,
    data::{GameOptions, Persistent},
    input::{Action, ActionState},
    AppConfig, GamePlugin, GameState,
};
use rand::Rng;

const LEVEL_SIZE: Vec2 = Vec2::new(600., 600.);
const PLAYER_SIZE: Vec2 = Vec2::new(72., 72.);
const PLATFORM_SIZE: Vec2 = Vec2::new(125., 25.);
const SPACE_BETWEEN_PLATFORMS: u32 = 150;

const INITIAL_VEL: Vec2 = Vec2::new(0., 1000.);
const GRAVITY: f32 = -8000.;
const JUMP_VEL: f32 = 1800.;
const MOVE_VEL: f32 = 800.;
const BOUNCE_CUTOFF: f32 = 150.;
const BOUNCE_FACTOR: f32 = 0.3;
const MOVE_CUTOFF: f32 = 100.;
const MOVE_FACTOR: f32 = 0.75;
const MAX_JUMPS: u8 = 1;

const CAMERA_VEL: f32 = 20.;
const JUMP_BUFFER: f32 = 0.1;

fn main() {
    App::new()
        .insert_resource(AppConfig::default())
        .add_plugins((GamePlugin, SampleGamePlugin))
        .run();
}

// ······
// Plugin
// ······

pub struct SampleGamePlugin;

impl Plugin for SampleGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlatformInfo::default())
            .add_systems(
                OnEnter(GameState::Play),
                init_sample.run_if(run_once()),
            )
            .add_systems(
                Update,
                (
                    update_player,
                    update_camera,
                    update_counter,
                    spawn_platforms,
                    check_game_over,
                )
                    .run_if(in_state(GameState::Play)),
            )
            .add_systems(OnEnter(GameState::End), restart_game);
    }
}

// ·········
// Resources
// ·········

#[derive(Resource, Default)]
struct PlatformInfo {
    last_platform: u32,
}

// ··········
// Components
// ··········

#[derive(Component, Default)]
struct Player {
    velocity: Vec2,
    remainder: Vec2,
    max_height: f32,
    jumps_left: u8,
    jump_buffer: Option<Timer>,
}

#[derive(Component)]
struct Platform;

#[derive(Component)]
struct Floor;

#[derive(Component, Default)]
struct Counter(u32);

#[derive(Component, Default)]
struct CameraFollow {
    target_pos: f32,
}

// ·······
// Systems
// ·······

fn init_sample(
    mut cmd: Commands,
    assets: Res<CoreAssets>,
    options: Res<Persistent<GameOptions>>,
    cam: Query<Entity, With<GameCamera>>,
) {
    // Player
    cmd.spawn((
        SpriteBundle {
            texture: assets.bevy_icon.clone(),
            transform: Transform::from_translation(Vec3::new(0., -32., 1.)),
            sprite: Sprite {
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            ..default()
        },
        Player {
            velocity: INITIAL_VEL,
            ..default()
        },
    ));

    // Floor
    cmd.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: options.base_color.darker(0.2),
                custom_size: Some(Vec2::new(LEVEL_SIZE.x, 32.)),
                ..default()
            },
            transform: Transform::from_xyz(0., -LEVEL_SIZE.y * 0.5 + 16., 0.),
            ..default()
        },
        Platform,
        Floor,
    ));

    // Counter
    cmd.spawn((
        Text2dBundle {
            text: Text::from_section("0", TextStyle {
                font: assets.font.clone(),
                font_size: 150.,
                color: Color::WHITE,
            }),
            transform: Transform::from_xyz(5.3, 0.3, -1.),
            ..default()
        },
        Counter::default(),
        CameraFollow::default(),
    ));

    let Ok(cam) = cam.get_single() else { return };
    cmd.entity(cam).insert(CameraFollow::default());
}

fn update_player(
    time: Res<Time>,
    input: Query<&ActionState<Action>>,
    mut player: Query<(&mut Player, &mut Transform)>,
    platforms: Query<(&Sprite, &Transform), (With<Platform>, Without<Player>)>,
) {
    let Ok((mut player, mut trans)) = player.get_single_mut() else { return };

    let Ok(input) = input.get_single() else { return };

    let mut pos = trans.translation.xy();
    pos += player.remainder;

    // Gravity
    player.velocity.y += GRAVITY * time.delta_seconds();

    // Jump
    if input.just_pressed(&Action::Jump) {
        if player.jumps_left > 0 {
            player.velocity.y = JUMP_VEL;
            player.jumps_left -= 1;
        } else {
            player.jump_buffer = Some(Timer::from_seconds(
                JUMP_BUFFER,
                TimerMode::Once,
            ));
        }
    }

    if let Some(buffer) = player.jump_buffer.as_mut() {
        if !buffer.tick(time.delta()).finished() && player.jumps_left > 0 {
            player.velocity.y = JUMP_VEL;
            player.jumps_left -= 1;
        }
    };

    // Move
    let axis = input.clamped_axis_pair(&Action::Move);
    let dir = axis.unwrap_or_default().x();
    if dir.abs() > 0. {
        player.velocity.x = dir * MOVE_VEL;
    } else if player.velocity.x.abs() > MOVE_CUTOFF {
        player.velocity.x *= MOVE_FACTOR;
    } else {
        player.velocity.x = 0.;
    }

    // Update position based on velocity and add bounds
    pos += player.velocity * time.delta_seconds();
    pos.y = pos.y.max(-LEVEL_SIZE.y * 0.4);
    pos.x = (pos.x + LEVEL_SIZE.x * 0.5).rem_euclid(LEVEL_SIZE.x) - LEVEL_SIZE.x * 0.5;

    // Check collisions
    if player.velocity.y <= 0. {
        let player_bounds = Aabb2d::new(
            trans.translation.truncate(),
            PLAYER_SIZE * 0.5,
        );

        for (sprite, platform) in platforms.iter() {
            let platform_bounds = Aabb2d::new(
                platform.translation.truncate(),
                sprite.custom_size.unwrap_or(PLATFORM_SIZE) * 0.5,
            );

            let collision = {
                if !player_bounds.intersects(&platform_bounds) {
                    false
                } else {
                    let closest = platform_bounds.closest_point(player_bounds.center());
                    let offset = player_bounds.center() - closest;
                    offset.y > 0.
                }
            };

            if collision {
                pos.y =
                    platform.translation.y + platform_bounds.half_size().y + PLAYER_SIZE.y * 0.5;
                player.jumps_left = MAX_JUMPS;

                if player.velocity.y.abs() > BOUNCE_CUTOFF {
                    player.velocity.y = player.velocity.y.abs() * BOUNCE_FACTOR;
                } else {
                    player.velocity.y = 0.;
                }
            }
        }
    }

    // Floor and save remainder
    trans.translation = pos.floor().extend(1.);
    player.remainder = pos - trans.translation.xy();

    // Update max height
    player.max_height = player
        .max_height
        .max(trans.translation.y + LEVEL_SIZE.y * 0.5);
}

fn update_camera(
    time: Res<Time>,
    mut cam: Query<(&mut Transform, &mut CameraFollow)>,
    player: Query<&Player>,
) {
    let Ok(player) = player.get_single() else { return };

    for (mut trans, mut follow) in cam.iter_mut() {
        let vel = (CAMERA_VEL * follow.target_pos / LEVEL_SIZE.y).powf(0.8);

        follow.target_pos = (follow.target_pos + vel * time.delta_seconds())
            .max(player.max_height - LEVEL_SIZE.y * 0.5);

        trans.translation.y = lerp(
            trans.translation.y,
            follow.target_pos,
            0.5,
        );
    }
}

fn update_counter(mut counter: Query<(&mut Counter, &mut Text)>, player: Query<&Player>) {
    let Ok((mut counter, mut text)) = counter.get_single_mut() else { return };
    let Ok(player) = player.get_single() else { return };

    counter.0 = (player.max_height as u32 / SPACE_BETWEEN_PLATFORMS).saturating_sub(1);
    text.sections[0].value = counter.0.to_string();
}

fn spawn_platforms(
    mut cmd: Commands,
    options: Res<Persistent<GameOptions>>,
    mut info: ResMut<PlatformInfo>,
    player: Query<&Player>,
) {
    let Ok(player) = player.get_single() else { return };

    while info.last_platform * SPACE_BETWEEN_PLATFORMS
        < (player.max_height + LEVEL_SIZE.y * 0.5) as u32
    {
        info.last_platform += 1;
        let x = (rand::thread_rng().gen::<f32>() - 0.5) * LEVEL_SIZE.x;

        cmd.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: options
                        .base_color
                        .lighter(rand::random::<f32>() * 0.2 - 0.1),
                    custom_size: Some(PLATFORM_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(
                    x.round(),
                    info.last_platform as f32 * SPACE_BETWEEN_PLATFORMS as f32 - LEVEL_SIZE.y * 0.5,
                    0.,
                ),
                ..default()
            },
            Platform,
        ));
    }
}

fn check_game_over(
    mut state: ResMut<NextState<GameState>>,
    player: Query<&Transform, With<Player>>,
    cam: Query<&CameraFollow, With<GameCamera>>,
) {
    let Ok(player) = player.get_single() else { return };
    let Ok(cam) = cam.get_single() else { return };

    if player.translation.y < cam.target_pos - LEVEL_SIZE.y * 0.5 {
        state.set(GameState::End);
    }
}

fn restart_game(
    mut cmd: Commands,
    mut state: ResMut<NextState<GameState>>,
    mut info: ResMut<PlatformInfo>,
    mut player: Query<(&mut Player, &mut Transform)>,
    mut follow: Query<&mut CameraFollow>,
    platforms: Query<Entity, (With<Platform>, Without<Floor>)>,
) {
    let Ok((mut player, mut trans)) = player.get_single_mut() else { return };

    player.max_height = 0.;
    trans.translation.y = -32.;

    for mut follow in follow.iter_mut() {
        *follow = CameraFollow::default();
    }

    info.last_platform = 0;
    for platform in platforms.iter() {
        cmd.entity(platform).despawn();
    }

    state.set(GameState::Play);
}

// ·····
// Extra
// ·····

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
