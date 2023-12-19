#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;
use bevy_persistent::Persistent;
use hello_bevy::{
    CoreAssets,
    GameAppConfig,
    GameOptions,
    GamePlugin,
    GameState,
    InputMovement,
    KeyBind,
    Keybinds,
};

const INITIAL_VEL: Vec2 = Vec2::new(0., 250.);
const GRAVITY: f32 = -2000.;
const JUMP_VEL: f32 = 400.;
const MOVE_VEL: f32 = 250.;
const BOUNCE_CUTOFF: f32 = 50.;
const BOUNCE_FACTOR: f32 = 0.2;
const MOVE_CUTOFF: f32 = 50.;
const MOVE_FACTOR: f32 = 0.85;

fn main() {
    App::new()
        .insert_resource(GameAppConfig {
            initial_window_res: Vec2::new(600., 600.),
            initial_game_res: Vec2::new(128., 128.),
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
            OnEnter(GameState::Play),
            (
                init_sample.run_if(run_once()),
                resume_game,
            ),
        )
        .register_type::<Player>()
        .add_systems(
            Update,
            update_sample.run_if(in_state(GameState::Play)),
        )
        .add_systems(OnExit(GameState::Play), pause_game);
    }
}

// ··········
// Components
// ··········

#[derive(Reflect, Component, Default)]
struct Player {
    velocity: Vec2,
    remainder: Vec2,
}

#[derive(Component)]
struct Counter(u32);

#[derive(Component)]
struct GameCamera;

// ·······
// Systems
// ·······

fn init_sample(
    mut cmd: Commands,
    app_config: Res<GameAppConfig>,
    assets: Res<CoreAssets>,
    opts: Res<Persistent<GameOptions>>,
) {
    // Background
    cmd.spawn(SpriteBundle {
        sprite: Sprite {
            color: opts.color.dark,
            custom_size: Some(app_config.initial_game_res),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., -10.),
        ..default()
    });

    // Player
    cmd.spawn((
        SpriteBundle {
            texture: assets.bevy_icon.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..default()
        },
        Player {
            velocity: INITIAL_VEL,
            remainder: Vec2::ZERO,
        },
    ));

    // Counter
    cmd.spawn((
        Text2dBundle {
            text: Text::from_section("0", TextStyle {
                font: assets.font.clone(),
                font_size: 60.,
                color: opts.color.mid,
            }),
            transform: Transform::from_xyz(5.3, 0.3, 0.),
            ..default()
        },
        Counter(0),
    ));
}

fn update_sample(
    time: Res<Time>,
    app_config: Res<GameAppConfig>,
    input: Res<Input<KeyBind>>,
    movement: Res<InputMovement>,
    keybinds: Res<Persistent<Keybinds>>,
    mut objects: Query<(&mut Player, &mut Transform)>,
    mut counter: Query<(&mut Text, &mut Counter)>,
) {
    let Vec2 { x: gx, y: gy } = app_config.initial_game_res;

    for (mut player, mut trans) in objects.iter_mut() {
        let mut pos = trans.translation.xy();
        pos += player.remainder;

        // Gravity
        if pos.y > -gy * 0.4 {
            player.velocity.y += GRAVITY * time.delta_seconds();
        } else {
            pos.y = -gy * 0.4;
            if player.velocity.y.abs() > BOUNCE_CUTOFF {
                player.velocity.y = player.velocity.y.abs() * BOUNCE_FACTOR;
            } else {
                player.velocity.y = 0.;
            }
        }

        // Jump
        if keybinds.jump.just_pressed(&input) {
            player.velocity.y = JUMP_VEL;

            let (mut text, mut counter) = counter.single_mut();
            counter.0 += 1;
            text.sections[0].value = counter.0.to_string();
        }

        // Move
        let dir = keybinds.x_axis.get(&movement);
        if dir.abs() > 0. {
            player.velocity.x = dir * MOVE_VEL;
        } else if player.velocity.x.abs() > MOVE_CUTOFF {
            player.velocity.x *= MOVE_FACTOR;
        } else {
            player.velocity.x = 0.;
        }

        // Update position based on velocity and add bounds
        pos += player.velocity * time.delta_seconds();
        pos.y = pos.y.max(-gy * 0.4);
        pos.x = (pos.x + gx * 0.5).rem_euclid(gx) - gx * 0.5;

        // Floor and save remainder
        trans.translation = pos.floor().extend(1.);
        player.remainder = pos - trans.translation.xy();
    }
}

fn resume_game(mut cam: Query<&mut Camera, With<GameCamera>>) {
    if let Ok(mut cam) = cam.get_single_mut() {
        cam.is_active = true;
    }
}

fn pause_game(mut cam: Query<&mut Camera, With<GameCamera>>) {
    if let Ok(mut cam) = cam.get_single_mut() {
        cam.is_active = false;
    }
}
