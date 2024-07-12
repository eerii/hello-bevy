use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};
use hello_bevy::{
    assets::{CoreAssets, ExampleAssets},
    camera::GameCamera,
    AppConfig, GamePlugin, GameState,
};

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
        app.add_event::<CollisionEvent>()
            .add_systems(
                OnEnter(GameState::Play),
                init_sample.run_if(run_once()),
            )
            .add_systems(
                Update,
                (update_sample, on_collision, on_resize).run_if(in_state(GameState::Play)),
            );
    }
}

// ·········
// Resources
// ·········

#[derive(Resource)]
struct Bounds(Vec2);

// ··········
// Components
// ··········

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Counter(u32);

#[derive(Component)]
struct Background;

// ······
// Events
// ······

#[derive(Event)]
struct CollisionEvent(Entity);

// ·······
// Systems
// ·······

fn init_sample(
    mut cmd: Commands,
    assets: Res<CoreAssets>,
    win: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(win) = win.get_single() else { return };

    let size = Vec2::new(win.width(), win.height());
    cmd.insert_resource(Bounds(size));

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
                color: Color::WHITE,
            }),
            ..default()
        },
        Counter(0),
    ));
}

fn update_sample(
    time: Res<Time>,
    bounds: Res<Bounds>,
    mut objects: Query<(
        Entity,
        &mut Transform,
        &mut Velocity,
        &Sprite,
    )>,
    mut event_collision: EventWriter<CollisionEvent>,
) {
    let win_bound = Rect::from_center_size(Vec2::ZERO, bounds.0);

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

        // If it is completely gone, teleport to the start
        if t.x.abs() > bounds.0.x * 0.5 || t.y.abs() > bounds.0.y * 0.5 {
            t.x = 0.;
            t.y = 0.;
        }
    }
}

fn on_collision(
    mut cmd: Commands,
    mut objects: Query<&mut Sprite>,
    mut counter: Query<(&mut Text, &mut Counter)>,
    mut event_collision: EventReader<CollisionEvent>,
    assets: Res<ExampleAssets>,
) {
    let (mut text, mut counter) = counter.single_mut();

    // On each collision, increase the counter, change the color and play audio
    for CollisionEvent(e) in event_collision.read() {
        counter.0 += 1;
        text.sections[0].value = counter.0.to_string();

        if let Ok(mut sprite) = objects.get_mut(*e) {
            sprite.color = random_color();

            cmd.spawn(AudioBundle {
                source: assets.boing.clone(),
                settings: PlaybackSettings::DESPAWN,
            });
        }
    }
}

fn on_resize(
    mut bounds: ResMut<Bounds>,
    cam: Query<&Camera, With<GameCamera>>,
    win: Query<&mut Window, With<PrimaryWindow>>,
    mut bg: Query<&mut Sprite, With<Background>>,
    mut event_resize: EventReader<WindowResized>,
) {
    let Ok(win) = win.get_single() else { return };
    let Ok(cam) = cam.get_single() else { return };

    for e in event_resize.read() {
        let size = if let Some(viewport) = cam.viewport.as_ref() {
            viewport.physical_size.as_vec2() / win.scale_factor()
        } else {
            Vec2::new(e.width, e.height)
        };

        bounds.0 = size;
        for mut sprite in bg.iter_mut() {
            sprite.custom_size = Some(size);
        }
    }
}

// ·······
// Helpers
// ·······

fn random_color() -> Color {
    Color::hsl(rand::random::<f32>() * 360., 0.8, 0.8)
}
