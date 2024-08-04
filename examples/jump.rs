#![allow(clippy::type_complexity)]

use bevy::math::bounding::*;
use game::prelude::*;
use rand::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::splat(72.);
const PLATFORM_SIZE: Vec2 = Vec2::new(125., 25.);
const SPACE_BETWEEN_PLATFORMS: u32 = 150;
const GRAVITY: f32 = -8000.;
const JUMP_VEL: f32 = 1800.;
const MOVE_VEL: f32 = 800.;
const BOUNCE_CUTOFF: f32 = 150.;
const BOUNCE_FACTOR: f32 = 0.3;
const MOVE_CUTOFF: f32 = 100.;
const MOVE_FACTOR: f32 = 0.75;
const JUMP_BUFFER: f32 = 0.1;

fn main() {
    App::new().add_plugins((GamePlugin, plugin)).run();
}

fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Play), init.run_if(on_setup()))
        .add_systems(
            Update,
            (
                update_player.in_set(PlaySet::Update),
                (update_counter, update_camera).in_set(PlaySet::Animation),
                (check_collision, spawn_platforms, check_game_over).after(update_player),
            )
                .run_if(in_state(GameState::Play)),
        )
        .add_systems(OnEnter(GameState::End), reset);
}

// Resources
// ---

/// This could go in `SaveData`, but since this is an example we make it
/// separate.
#[derive(Resource, Default)]
struct ExampleData {
    last_platform: u32,
}

// Components
// ---

#[derive(Component, Default)]
struct Player {
    velocity: Vec2,
    max_height: f32,
    can_jump: bool,
    jump_buffer: Option<Timer>,
}

#[derive(Component)]
struct Platform;

#[derive(Component, Default)]
struct Counter(u32);

#[derive(Component)]
struct CameraFollow;

// Systems
// ---

/// Spawn the initial objects.
fn init(
    mut cmd: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    mut camera: Query<(Entity, &mut Transform), With<GameCamera>>,
    options: Res<GameOptions>,
    meta_assets: Res<AssetMap<MetaAssetKey>>,
    font_assets: Res<AssetMap<FontAssetKey>>,
) {
    let size = single!(window).size();

    // Player
    cmd.spawn((
        SpriteBundle {
            texture: meta_assets.get(&MetaAssetKey::BevyLogo).clone_weak(),
            transform: Transform::from_translation(Vec3::new(0., 0., 10.)),
            sprite: Sprite {
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            ..default()
        },
        Player::default(),
    ));

    // Floor
    cmd.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: options.palette.dark,
                custom_size: Some(Vec2::new(size.x, PLATFORM_SIZE.y * 2.)),
                ..default()
            },
            transform: Transform::from_xyz(0., -size.y / 2. + PLATFORM_SIZE.y, 5.),
            ..default()
        },
        Platform,
    ));

    // Counter
    cmd.spawn((
        Text2dBundle {
            text: Text::from_section("0", TextStyle {
                font: font_assets.get(&FontAssetKey::Main).clone_weak(),
                font_size: 150.,
                color: options.palette.light,
            }),
            ..default()
        },
        Counter::default(),
        CameraFollow,
    ));

    // Adds a camera follow to the initial camera
    let (entity, mut trans) = single_mut!(camera);
    cmd.entity(entity).insert(CameraFollow);
    trans.translation.y = 0.;

    // Keeps track of the last platform generated
    cmd.insert_resource(ExampleData::default())
}

fn update_player(
    time: Res<Time>,
    input: Query<&ActionState<Action>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut player: Query<(&mut Player, &mut Transform)>,
) {
    let (mut player, mut trans) = single_mut!(player);
    let input = single!(input);
    let size = single!(window).size();

    // Gravity
    player.velocity.y += GRAVITY * time.delta_seconds();

    // Jump
    let mut jump_input = input.just_pressed(&Action::Act);
    if jump_input && !player.can_jump {
        // Create a small time buffer for jump inputs
        player.jump_buffer = Some(Timer::from_seconds(JUMP_BUFFER, TimerMode::Once));
    }
    if let Some(buffer) = player.jump_buffer.as_mut() {
        if !buffer.tick(time.delta()).finished() {
            jump_input = true;
        }
    };
    if jump_input && player.can_jump {
        player.velocity.y = JUMP_VEL;
        player.can_jump = false;
    }

    // Move
    let dir = input.clamped_axis_pair(&Action::Move).x;
    if dir.abs() > 0.2 {
        player.velocity.x = dir * MOVE_VEL;
    } else if player.velocity.x.abs() > MOVE_CUTOFF {
        player.velocity.x *= MOVE_FACTOR;
    } else {
        player.velocity.x = 0.;
    }

    // Update position based on velocity and loop around
    trans.translation += player.velocity.extend(0.) * time.delta_seconds();
    trans.translation.x = (trans.translation.x + size.x / 2.).rem_euclid(size.x) - size.x / 2.;

    // Update max height
    player.max_height = player.max_height.max(trans.translation.y);
}

/// Checks collision between the player and the platforms
fn check_collision(
    mut player: Query<(&mut Player, &mut Transform)>,
    platforms: Query<(&Sprite, &Transform), (With<Platform>, Without<Player>)>,
) {
    let (mut player, mut trans) = single_mut!(player);

    // Only check for collisions if the player is going down (allows to pass
    // platforms from below)
    if player.velocity.y <= 0. {
        let player_bounds = Aabb2d::new(trans.translation.truncate(), PLAYER_SIZE / 2.);

        for (sprite, platform) in platforms.iter() {
            let platform_bounds = Aabb2d::new(
                platform.translation.truncate(),
                sprite.custom_size.unwrap_or(PLATFORM_SIZE) / 2.,
            );

            // Check the collision between the player and the platform bounds
            // Only register a collision if it happens with the player above
            let collision = {
                if player_bounds.intersects(&platform_bounds) {
                    let closest = platform_bounds.closest_point(player_bounds.center());
                    let offset = player_bounds.center() - closest;
                    offset.y > 0.
                } else {
                    false
                }
            };

            if collision {
                // Relocate the player perfectly on top of the platform
                trans.translation.y =
                    platform.translation.y + platform_bounds.half_size().y + PLAYER_SIZE.y / 2.;
                // If the player is going quick enough, bounce
                player.velocity.y = if player.velocity.y.abs() > BOUNCE_CUTOFF {
                    player.velocity.y.abs() * BOUNCE_FACTOR
                } else {
                    0.
                };
                player.can_jump = true;
            }
        }
    }
}

/// Creates platforms as the player goes up
fn spawn_platforms(
    mut cmd: Commands,
    player: Query<&Player>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut data: ResMut<ExampleData>,
    options: Res<GameOptions>,
) {
    let mut rng = rand::thread_rng();
    let player = single!(player);
    let size = single!(window).size();

    while data.last_platform * SPACE_BETWEEN_PLATFORMS < (player.max_height + size.y) as u32 {
        data.last_platform += 1;
        let x = (rng.gen::<f32>() - 0.5) * size.x;

        cmd.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: options.palette.dark.lighter(rng.gen::<f32>() * 0.2 - 0.05),
                    custom_size: Some(PLATFORM_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(
                    x.round(),
                    data.last_platform as f32 * SPACE_BETWEEN_PLATFORMS as f32 - size.y / 2.,
                    5.,
                ),
                ..default()
            },
            Platform,
        ));
    }
}

/// Makes the camera follow the player.
fn update_camera(player: Query<&Player>, mut camera: Query<&mut Transform, With<CameraFollow>>) {
    let player = single!(player);
    let target = player.max_height;

    for mut trans in camera.iter_mut() {
        trans.translation.y = trans.translation.y.lerp(target, 0.3);
    }
}

/// Updates the score counter
fn update_counter(mut counter: Query<(&mut Counter, &mut Text)>, player: Query<&Player>) {
    let (mut counter, mut text) = single_mut!(counter);
    let player = single!(player);

    counter.0 = (player.max_height as u32 / SPACE_BETWEEN_PLATFORMS).saturating_sub(1);
    if let Some(section) = text.sections.first_mut() {
        section.value = counter.0.to_string();
    }
}

/// Checks if the player fell off the screen and transitions to the end state.
fn check_game_over(
    mut state: ResMut<NextState<GameState>>,
    player: Query<&Transform, With<Player>>,
    camera: Query<&Transform, With<GameCamera>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let player = single!(player);
    let camera = single!(camera);
    let size = single!(window).size();

    if player.translation.y < camera.translation.y - size.y / 2. {
        state.set(GameState::End);
    }
}

/// When entering the end state, reset the setup of the game so that init can
/// run again and go back to the play state.
/// We could do this automatically with the new `StateScoped`, which is very
/// useful, but that takes away some flexibility. For example, going to the menu
/// would delete the entities. A mixture of both could be possible and
/// benefitial for larger games.
fn reset(
    mut cmd: Commands,
    mut state: ResMut<NextState<GameState>>,
    entities: Query<Entity, Or<(With<Player>, With<Counter>, With<Platform>)>>,
) {
    for entity in &entities {
        cmd.entity(entity).despawn();
    }

    cmd.reset_setup();
    state.set(GameState::Play);
}
