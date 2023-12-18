use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_persistent::Persistent;
use hello_bevy::{
    CoreAssets,
    ExampleAssets,
    GameOptions,
    GamePlugin,
    GameState,
};

const SIZE: Vec2 = Vec2::new(600., 600.);

fn main() { App::new().add_plugins((GamePlugin, SampleGamePlugin)).run(); }

// ······
// Plugin
// ······

pub struct SampleGamePlugin;

impl Plugin for SampleGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_systems(
                OnEnter(GameState::Play),
                init_sample.run_if(run_once()),
            )
            .add_systems(
                Update,
                (update_sample, on_collision).run_if(in_state(GameState::Play)),
            );
    }
}

// ··········
// Components
// ··········

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Counter(u32);

// ······
// Events
// ······

#[derive(Event)]
struct CollisionEvent(Entity);

// ·······
// Systems
// ·······

fn init_sample(mut cmd: Commands, assets: Res<CoreAssets>, opts: Res<Persistent<GameOptions>>) {
    // Background
    cmd.spawn(SpriteBundle {
        sprite: Sprite {
            color: opts.color.dark,
            custom_size: Some(SIZE),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., -10.),
        ..default()
    });

    // Sprites
    for velocity in [
        Vec2::new(300., 250.),
        Vec2::new(-150., 400.),
        Vec2::new(200., -350.),
    ] {
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
            Velocity(velocity),
        ));
    }

    // Counter text
    cmd.spawn((
        Text2dBundle {
            text: Text::from_section("0", TextStyle {
                font: assets.font.clone(),
                font_size: 192.,
                color: opts.color.mid,
            }),
            ..default()
        },
        Counter(0),
    ));
}

fn update_sample(
    time: Res<Time>,
    mut objects: Query<(
        Entity,
        &mut Transform,
        &mut Velocity,
        &Sprite,
    )>,
    mut event_collision: EventWriter<CollisionEvent>,
) {
    let win_bound = Rect::from_center_size(Vec2::ZERO, SIZE);

    for (entity, mut trans, mut vel, sprite) in objects.iter_mut() {
        // Update position based on velocity
        let t = &mut trans.translation;
        *t += vel.0.extend(0.) * time.delta_seconds();

        // Calculate the sprite bound
        let obj_bound = Rect::from_center_size(
            Vec2::new(t.x, t.y),
            sprite.custom_size.expect("Sprite needs a custom size"),
        );

        // Calculate the interection with the level borders and send a collision event
        let intersection = win_bound.intersect(obj_bound).size();
        if intersection.x < obj_bound.width() {
            vel.0.x *= -1.;
            t.x += (obj_bound.width() - intersection.x) * vel.0.x.signum();
            event_collision.send(CollisionEvent(entity));
        }
        if intersection.y < obj_bound.height() {
            vel.0.y *= -1.;
            t.y += (obj_bound.height() - intersection.y) * vel.0.y.signum();
            event_collision.send(CollisionEvent(entity));
        }
    }
}

fn on_collision(
    mut objects: Query<&mut Sprite>,
    mut counter: Query<(&mut Text, &mut Counter)>,
    mut event_collision: EventReader<CollisionEvent>,
    audio: Res<Audio>,
    assets: Res<ExampleAssets>,
) {
    let (mut text, mut counter) = counter.single_mut();

    // On each collision, increase the counter, change the spirte color and play audio
    for CollisionEvent(entity) in event_collision.read() {
        counter.0 += 1;
        text.sections[0].value = counter.0.to_string();

        if let Ok(mut sprite) = objects.get_mut(*entity) {
            sprite.color = random_color();
        }

        audio.play(assets.boing.clone()).with_volume(0.3);
    }
}

// ·····
// Extra
// ·····

fn random_color() -> Color { Color::hsl(rand::random::<f32>() * 360., 0.8, 0.8) }
