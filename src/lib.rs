mod debug;
mod load;
mod menu;
mod save;

pub use debug::{save_schedule, DEBUG};

use bevy::prelude::*;

// Game state
#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    Loading,
    Menu,
    Play,
    Fail,
}

// Main game plugin
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(load::LoadPlugin)
            .add_plugin(save::SavePlugin)
            .add_plugin(menu::MenuPlugin);

        #[cfg(debug_assertions)]
        app.add_plugin(debug::DebugPlugin);

        // Sample systems
        app.add_systems(OnEnter(GameState::Play), init_sample)
            .add_systems(Update, update_sample.run_if(in_state(GameState::Play)));
    }
}

// ---
// An example

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Counter(u32);

fn init_sample(mut cmd: Commands, assets: Res<load::SplashAssets>) {
    cmd.spawn(Camera2dBundle::default());

    for velocity in [
        Vec2::new(300., 250.),
        Vec2::new(-150., 400.),
        Vec2::new(200., -350.),
    ] {
        cmd.spawn((
            SpriteBundle {
                texture: assets.bevy_icon.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(128., 128.)),
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
            transform: Transform::from_translation(Vec3::new(32., 0., 0.)),
            ..default()
        },
        Counter(0),
    ));
}

fn random_color() -> Color {
    Color::hsl(rand::random::<f32>() * 360., 0.5, 0.7)
}

fn update_sample(
    time: Res<Time>,
    window: Query<&Window>,
    mut counter: Query<(&mut Text, &mut Counter)>,
    mut objects: Query<(&mut Transform, &mut Velocity, &mut Sprite)>,
) {
    for (mut trans, mut vel, mut sprite) in objects.iter_mut() {
        let window = window.single();
        let win_bound =
            Rect::from_center_size(Vec2::ZERO, Vec2::new(window.width(), window.height()));

        let t = &mut trans.translation;
        *t += vel.0.extend(0.) * time.delta_seconds();
        let obj_bound = Rect::from_center_size(
            Vec2::new(t.x, t.y),
            sprite.custom_size.expect("Sprite has a custom size"),
        );

        let intersection = win_bound.intersect(obj_bound).size();
        let (_, mut counter) = counter.single_mut();
        if intersection.x < obj_bound.width() {
            vel.0.x *= -1.;
            t.x += (obj_bound.width() - intersection.x) * vel.0.x.signum();
            sprite.color = random_color();
            counter.0 += 1;
        }
        if intersection.y < obj_bound.height() {
            vel.0.y *= -1.;
            t.y += (obj_bound.height() - intersection.y) * vel.0.y.signum();
            sprite.color = random_color();
            counter.0 += 1;
        }
    }

    if let Ok((mut text, counter)) = counter.get_single_mut() {
        text.sections[0].value = counter.0.to_string();
    }
}
