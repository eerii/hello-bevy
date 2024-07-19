#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use bevy::{math::bounding::*, prelude::*, sprite::MaterialMesh2dBundle};
use hello_bevy::{
    assets::ExampleAssets,
    data::{GameOptions, Persistent},
    input::{Action, ActionState},
    AppConfig, GamePlugin, GameState,
};
use itertools::Itertools;

const PADDLE_SIZE: Vec2 = Vec2::new(125., 25.);
const BALL_SIZE: f32 = 30.;
const PADDLE_VEL: f32 = 1000.;
const BALL_INITIAL_VEL: f32 = 300.;
const BRICK_GAP: f32 = 5.;
const BRICK_COUNT: UVec2 = UVec2::new(5, 3);
const BRICK_WIDTH: f32 = (BOUNDS.x * 2. - BRICK_COUNT.x as f32 * BRICK_GAP) / BRICK_COUNT.x as f32;
const BRICK_HEIGHT: f32 = 25.;
const BOUNDS: Vec2 = Vec2::new(250., 250.);
const SPEEDUP: f32 = 1.02;

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
        app.configure_sets(
            FixedUpdate,
            (PlaySet::Move, PlaySet::Collision)
                .chain()
                .run_if(in_state(GameState::Play)),
        )
        .add_event::<CollisionEvent>()
        .add_systems(
            OnEnter(GameState::Play),
            init.run_if(run_once()),
        )
        .add_systems(
            Update,
            (on_collision, tick_break).run_if(in_state(GameState::Play)),
        )
        .add_systems(
            FixedUpdate,
            (
                (update_paddle, update_ball).in_set(PlaySet::Move),
                check_collisions.in_set(PlaySet::Collision),
            ),
        )
        .add_systems(OnEnter(GameState::End), reset);
    }
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum PlaySet {
    Move,
    Collision,
}

// ··········
// Components
// ··········

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Brick;

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct BreakTimer(Timer);

// ······
// Events
// ······

#[derive(Event)]
struct CollisionEvent;

enum Collision {
    North,
    South,
    East,
    West,
}

// ·······
// Systems
// ·······

fn init(
    mut cmd: Commands,
    options: Res<Persistent<GameOptions>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    cmd.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: options.accent_color,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., -BOUNDS.y, 0.))
                .with_scale(PADDLE_SIZE.extend(1.)),
            ..default()
        },
        Paddle,
        Collider,
    ));

    for (x, y) in [(-1., 0.), (1., 0.), (0., 1.)] {
        cmd.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: options.base_color.darker(0.2),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    x * (BOUNDS.x + 37.5),
                    y * (BOUNDS.y + 37.5),
                    0.,
                ))
                .with_scale(
                    Vec2::new(
                        y.abs() * (BOUNDS.x + 50.) * 2. + PADDLE_SIZE.y,
                        x.abs() * (BOUNDS.y + 50.) * 2. + PADDLE_SIZE.y,
                    )
                    .extend(1.),
                ),
                ..default()
            },
            Collider,
        ));
    }

    for (x, y) in (0..BRICK_COUNT.x).cartesian_product(0..BRICK_COUNT.y) {
        cmd.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: options.base_color.lighter(y as f32 * 0.04),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    x as f32 * (BRICK_WIDTH + BRICK_GAP) - BOUNDS.x + 50.,
                    BOUNDS.y - (y as f32 * (BRICK_HEIGHT + BRICK_GAP) + 25.),
                    0.,
                ))
                .with_scale(Vec2::new(BRICK_WIDTH, BRICK_HEIGHT).extend(1.)),
                ..default()
            },
            Brick,
            Collider,
        ));
    }

    cmd.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(options.base_color.lighter(0.2)),
            transform: Transform::default().with_scale(Vec2::splat(BALL_SIZE).extend(1.)),
            ..default()
        },
        Ball {
            velocity: Vec2::new(
                (rand::random::<f32>() - 0.5) * BALL_INITIAL_VEL,
                -BALL_INITIAL_VEL,
            ),
        },
    ));
}

fn update_paddle(
    time: Res<Time>,
    input: Query<&ActionState<Action>>,
    mut paddle: Query<&mut Transform, With<Paddle>>,
) {
    let Ok(mut trans) = paddle.get_single_mut() else { return };
    let Ok(input) = input.get_single() else { return };

    let axis = input.clamped_axis_pair(&Action::Move);
    let dir = axis.unwrap_or_default().x();

    trans.translation.x += dir * PADDLE_VEL * time.delta_seconds();
    trans.translation.x = trans.translation.x.clamp(
        -(BOUNDS.x - PADDLE_SIZE.x * 0.5),
        BOUNDS.x - PADDLE_SIZE.x * 0.5,
    );
}

fn update_ball(
    time: Res<Time>,
    mut ball: Query<(&mut Transform, &Ball)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok((mut trans, ball)) = ball.get_single_mut() else { return };
    trans.translation += (ball.velocity * time.delta_seconds()).extend(0.);
    if trans.translation.y < -BOUNDS.y - 50. {
        next_state.set(GameState::End);
    }
}

fn check_collisions(
    mut cmd: Commands,
    mut ball: Query<(&Transform, &mut Ball)>,
    colliders: Query<
        (
            Entity,
            &Transform,
            Option<&Paddle>,
            Option<&Brick>,
        ),
        With<Collider>,
    >,
    mut collision_writer: EventWriter<CollisionEvent>,
) {
    let Ok((trans, mut ball)) = ball.get_single_mut() else { return };
    let ball_bounds = BoundingCircle::new(
        trans.translation.truncate(),
        BALL_SIZE / 2.,
    );

    for (entity, collider_trans, paddle, brick) in colliders.iter() {
        let collider_bounds = Aabb2d::new(
            collider_trans.translation.truncate(),
            collider_trans.scale.truncate() / 2.,
        );

        let Some(collision) = collide(&ball_bounds, &collider_bounds) else {
            continue;
        };
        match collision {
            Collision::North => {
                if ball.velocity.y < 0. {
                    ball.velocity.y *= -SPEEDUP
                }
            },
            Collision::South => {
                if ball.velocity.y > 0. {
                    ball.velocity.y *= -SPEEDUP
                }
            },
            Collision::East => {
                if ball.velocity.x < 0. {
                    ball.velocity.x *= -SPEEDUP
                }
            },
            Collision::West => {
                if ball.velocity.x > 0. {
                    ball.velocity.x *= -SPEEDUP
                }
            },
        }
        if brick.is_some() {
            cmd.entity(entity)
                .remove::<Collider>()
                .insert(BreakTimer(Timer::from_seconds(
                    0.2,
                    TimerMode::Once,
                )));
        }
        if paddle.is_some() {
            let offset = ball_bounds.center() - collider_bounds.center();
            ball.velocity.x += offset.x;
        }
        collision_writer.send(CollisionEvent);
    }
}

fn on_collision(
    mut cmd: Commands,
    mut collision_reader: EventReader<CollisionEvent>,
    assets: Res<ExampleAssets>,
) {
    for CollisionEvent in collision_reader.read() {
        cmd.spawn(AudioBundle {
            source: assets.boing.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

fn tick_break(
    mut cmd: Commands,
    time: Res<Time>,
    mut break_timers: Query<(
        Entity,
        &mut BreakTimer,
        &mut Sprite,
        &mut Transform,
    )>,
) {
    for (entity, mut timer, mut sprite, mut trans) in break_timers.iter_mut() {
        let timer = timer.0.tick(time.delta());
        if timer.just_finished() {
            cmd.entity(entity).despawn();
            continue;
        }
        trans.scale.x = ((timer.fraction() * 3. + 0.5).sin() + 0.5) * BRICK_WIDTH;
        trans.scale.y = ((timer.fraction() * 2. - 0.5).cos() + 0.1) * BRICK_HEIGHT;
        sprite.color.set_alpha(timer.fraction_remaining());
    }
}

fn reset(
    mut cmd: Commands,
    ball: Query<Entity, With<Ball>>,
    colliders: Query<Entity, With<Collider>>,
    options: Res<Persistent<GameOptions>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for collider in colliders.iter() {
        cmd.entity(collider).despawn();
    }
    for ball in ball.iter() {
        cmd.entity(ball).despawn();
    }

    init(cmd, options, meshes, materials);
    next_state.set(if cfg!(feature = "menu") { GameState::Menu } else { GameState::Play });
}

// ·······
// Helpers
// ·······

fn collide(a: &BoundingCircle, b: &Aabb2d) -> Option<Collision> {
    if !a.intersects(b) {
        return None;
    }

    let offset = a.center() - b.closest_point(a.center());

    let collision = if offset.y.abs() > offset.x.abs() {
        if offset.y > 0. {
            Collision::North
        } else {
            Collision::South
        }
    } else if offset.x > 0. {
        Collision::East
    } else {
        Collision::West
    };

    Some(collision)
}
