use bevy::{prelude::*, window::WindowResolution};
use bevy_kira_audio::prelude::*;
use hello_bevy::{GameAssets, GamePlugin, GameState, SampleAssets, COLOR_DARKER};

fn main() {
    App::new()
        .insert_resource(ClearColor(COLOR_DARKER))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "DVD Screensaver".to_string(),
                    resolution: WindowResolution::new(600., 600.),
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
        app.add_event::<CollisionEvent>()
            .add_systems(OnEnter(GameState::Play), init_sample)
            .add_systems(
                Update,
                (update_sample, on_collision).run_if(in_state(GameState::Play)),
            )
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
struct Velocity(Vec2);

#[derive(Component)]
struct Counter(u32);

#[derive(Component)]
struct GameCamera;

// ······
// Events
// ······

#[derive(Event)]
struct CollisionEvent(Entity);

// ·······
// Systems
// ·······

fn init_sample(mut cmd: Commands, assets: Res<GameAssets>, info: Option<Res<GameInfo>>) {
    cmd.spawn((Camera2dBundle::default(), GameCamera));

    if info.is_some() {
        debug!("Game already initialized");
        return;
    }
    cmd.insert_resource(GameInfo);

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

    cmd.spawn((
        Text2dBundle {
            text: Text::from_section(
                "0",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 192.,
                    color: Color::WHITE,
                },
            ),
            ..default()
        },
        Counter(0),
    ));
}

fn update_sample(
    time: Res<Time>,
    window: Query<&Window>,
    mut objects: Query<(Entity, &mut Transform, &mut Velocity, &Sprite)>,
    mut event_collision: EventWriter<CollisionEvent>,
) {
    let window = window.single();
    let win_bound = Rect::from_center_size(Vec2::ZERO, Vec2::new(window.width(), window.height()));

    for (entity, mut trans, mut vel, sprite) in objects.iter_mut() {
        let t = &mut trans.translation;
        *t += vel.0.extend(0.) * time.delta_seconds();
        let obj_bound = Rect::from_center_size(
            Vec2::new(t.x, t.y),
            sprite.custom_size.expect("Sprite has a custom size"),
        );

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
    assets: Res<SampleAssets>,
) {
    let (mut text, mut counter) = counter.single_mut();

    for CollisionEvent(entity) in event_collision.read() {
        counter.0 += 1;
        text.sections[0].value = counter.0.to_string();

        if let Ok(mut sprite) = objects.get_mut(*entity) {
            sprite.color = random_color();
        }

        audio.play(assets.boing.clone()).with_volume(0.3);
    }
}

fn random_color() -> Color {
    Color::hsl(rand::random::<f32>() * 360., 0.8, 0.8)
}

fn pause_game(mut cmd: Commands, query: Query<Entity, With<GameCamera>>) {
    for entity in query.iter() {
        cmd.entity(entity).despawn_recursive();
    }
}
