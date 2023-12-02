#![allow(clippy::too_many_arguments)]

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::WindowResolution};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_persistent::Persistent;
use hello_bevy::{
    config::{GameOptions, Keybinds},
    input::InputState,
    load::GameAssets,
    GamePlugin, GameState,
};

const SIZE: Vec2 = Vec2::new(600., 600.);
const INITIAL_VEL: Vec2 = Vec2::new(0., 250.);
const GRAVITY: f32 = -10000.;
const JUMP_VEL: f32 = 2000.;
const MOVE_VEL: f32 = 700.;
const BOUNCE_CUTOFF: f32 = 100.;
const BOUNCE_FACTOR: f32 = 0.2;
const MOVE_CUTOFF: f32 = 100.;
const MOVE_FACTOR: f32 = 0.85;

fn main() {
    App::new()
        .add_plugins((
            EmbeddedAssetPlugin {
                mode: PluginMode::ReplaceDefault,
            },
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Endless jump".to_string(),
                    resolution: WindowResolution::new(SIZE.x, SIZE.y),
                    resizable: false,
                    canvas: Some("#bevy".to_string()),
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            GamePlugin,
            SampleGamePlugin,
        ))
        // Run
        .run();
}

// ······
// Plugin
// ······

pub struct SampleGamePlugin;

impl Plugin for SampleGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Play), init_sample)
            .add_systems(Update, update_sample.run_if(in_state(GameState::Play)))
            .add_systems(OnExit(GameState::Play), pause_game);
    }
}

// ·········
// Resources
// ·········

#[derive(Resource)]
struct GameInfo;

// ··········
// Components
// ··········

#[derive(Component)]
struct Player {
    velocity: Vec2,
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
    assets: Res<GameAssets>,
    opts: Res<Persistent<GameOptions>>,
    info: Option<Res<GameInfo>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    cmd.spawn((Camera2dBundle::default(), GameCamera));

    if info.is_some() {
        debug!("Game already initialized");
        return;
    }
    cmd.insert_resource(GameInfo);

    // Background
    cmd.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::from_xyz(0., 0., -10.).with_scale(SIZE.extend(1.)),
        material: materials.add(ColorMaterial::from(opts.color.darker)),
        ..default()
    });

    // Player
    cmd.spawn((
        SpriteBundle {
            texture: assets.bevy_icon.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(96., 96.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..default()
        },
        Player {
            velocity: INITIAL_VEL,
        },
    ));

    // Counter
    cmd.spawn((
        Text2dBundle {
            text: Text::from_section(
                "0",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 192.,
                    color: opts.color.mid,
                },
            ),
            ..default()
        },
        Counter(0),
    ));
}

fn update_sample(
    time: Res<Time>,
    mut objects: Query<(&mut Player, &mut Transform)>,
    mut counter: Query<(&mut Text, &mut Counter)>,
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    gamepads: Res<Gamepads>,
    gamepad_buttons: Res<Input<GamepadButton>>,
    keybinds: Res<Persistent<Keybinds>>,
) {
    let input = InputState::new(&keyboard, &mouse, &gamepads, &gamepad_buttons);

    for (mut player, mut trans) in objects.iter_mut() {
        let t = &mut trans.translation;

        // Gravity
        if t.y > -SIZE.y * 0.4 {
            player.velocity.y += GRAVITY * time.delta_seconds();
        } else {
            t.y = -SIZE.y * 0.4;
            if player.velocity.y.abs() > BOUNCE_CUTOFF {
                player.velocity.y = player.velocity.y.abs() * BOUNCE_FACTOR;
            } else {
                player.velocity.y = 0.;
            }
        }

        // Jump
        if input.just_pressed(&keybinds.jump).unwrap_or(false) {
            player.velocity.y = JUMP_VEL;

            let (mut text, mut counter) = counter.single_mut();
            counter.0 += 1;
            text.sections[0].value = counter.0.to_string();
        }

        // Move
        if input.pressed(&keybinds.left).unwrap_or(false) {
            player.velocity.x = -MOVE_VEL;
        } else if input.pressed(&keybinds.right).unwrap_or(false) {
            player.velocity.x = MOVE_VEL;
        } else if player.velocity.x.abs() > MOVE_CUTOFF {
            player.velocity.x *= MOVE_FACTOR;
        } else {
            player.velocity.x = 0.;
        }

        // Move
        *t += player.velocity.extend(0.) * time.delta_seconds();
        t.y = t.y.max(-SIZE.y * 0.4);
        t.x = (t.x + SIZE.x * 0.5).rem_euclid(SIZE.x) - SIZE.x * 0.5;
    }
}

fn pause_game(mut cmd: Commands, query: Query<Entity, With<GameCamera>>) {
    for entity in query.iter() {
        cmd.entity(entity).despawn_recursive();
    }
}
