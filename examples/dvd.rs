use game::prelude::*;
use rand::{distributions::Standard, prelude::*};

fn main() {
    App::new().add_plugins((GamePlugin, plugin)).run();
}

fn plugin(app: &mut App) {
    app.add_event::<CollisionEvent>()
        .add_systems(OnEnter(GameState::Play), init.run_if(run_once))
        .add_systems(
            Update,
            (
                update_velocity.in_set(PlaySet::Update),
                on_collision
                    .in_set(PlaySet::ReadEvents)
                    .run_if(on_event::<CollisionEvent>),
            ),
        );
}

// Components
// ---

/// Applies a velocity to an entity every frame.
#[derive(Component)]
struct Velocity(Vec2);

/// Marker for the score counter entity.
#[derive(Component)]
struct Counter(u32);

// Events
// ---

/// Event that is triggered when an entity collides with the screen border.
#[derive(Event)]
struct CollisionEvent;

// Systems
// ---

/// Spawn the initial objects.
fn init(
    mut cmd: Commands,
    options: Res<GameOptions>,
    meta_assets: Res<AssetMap<MetaAssetKey>>,
    font_assets: Res<AssetMap<FontAssetKey>>,
) {
    // Moving balls
    for velocity in [
        Vec2::new(300., 250.),
        Vec2::new(-150., 400.),
        Vec2::new(200., -350.),
    ] {
        cmd.spawn((
            Sprite {
                image: meta_assets.get(&MetaAssetKey::BevyLogo).clone_weak(),
                custom_size: Some(Vec2::splat(96.)),
                ..default()
            },
            Transform::from_translation(Vec3::new(0., 0., 1.)),
            Velocity(velocity),
        ));
    }

    // Counter text
    cmd.spawn((
        Text2d::new("0"),
        TextFont {
            font: font_assets.get(&FontAssetKey::Main).clone_weak(),
            font_size: 192.,
            ..default()
        },
        TextColor(options.palette.light),
        Counter(0),
    ));
}

/// Update the position of the objects with the `Velocity` component and check
/// for collisions with the window border
fn update_velocity(
    time: Res<Time>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut objects: Query<(&mut Velocity, &mut Transform, &mut Sprite)>,
    mut collision_writer: EventWriter<CollisionEvent>,
) {
    let mut rng = rand::thread_rng();
    let window = single!(window);
    let win_bound = Rect::from_center_size(Vec2::ZERO, window.size());

    for (mut vel, mut trans, mut sprite) in objects.iter_mut() {
        // Update position based on velocity
        let t = &mut trans.translation;
        *t += vel.0.extend(0.) * time.delta_secs();

        // Calculate the sprite bound
        let obj_bound = Rect::from_center_size(
            trans.translation.truncate(),
            sprite.custom_size.expect("Sprite needs a custom size"),
        );

        // Calculate the interection with the level borders and check if there was a
        // collision
        let intersection = win_bound.intersect(obj_bound).size();
        let mut collision = false;
        if intersection.x < obj_bound.width() {
            vel.0.x *= -1.;
            trans.translation.x += (obj_bound.width() - intersection.x) * vel.0.x.signum();
            collision = true;
        }
        if intersection.y < obj_bound.height() {
            vel.0.y *= -1.;
            trans.translation.y += (obj_bound.height() - intersection.y) * vel.0.y.signum();
            collision = true;
        }

        // If there is a collision, create a new random color and send a collision event
        if collision {
            sprite.color = *rng.gen::<ColorWrapper>();
            collision_writer.send(CollisionEvent);
        }
    }
}

/// When there is a collision, increase the counder and play a bounce sound.
fn on_collision(
    mut cmd: Commands,
    mut counter: Query<(&mut Text2d, &mut Counter)>,
    sound_assets: Res<AssetMap<SoundAssetKey>>,
    mut collision_reader: EventReader<CollisionEvent>,
) {
    let (mut text, mut counter) = single_mut!(counter);

    for CollisionEvent in collision_reader.read() {
        counter.0 += 1;
        text.0 = counter.0.to_string();

        cmd.spawn(AudioPlayer(
            sound_assets.get(&SoundAssetKey::Boing).clone_weak(),
        ));
    }
}

// Helpers
// ---

/// Wrapper for color to be able to derive traits for it.
struct ColorWrapper(Color);

/// Generate a random color from a hue.
impl Distribution<ColorWrapper> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ColorWrapper {
        ColorWrapper(Color::hsl(rng.gen::<f32>() * 360., 0.8, 0.8))
    }
}

impl std::ops::Deref for ColorWrapper {
    type Target = Color;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
