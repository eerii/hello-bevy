//! Game camera.

use crate::prelude::*;

#[cfg(feature = "deferred")]
pub mod deferred;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Startup), init);

    #[cfg(feature = "deferred")]
    app.add_plugins(deferred::plugin);
}

// Components
// ---

/// The camera where the game is being rendered.
#[derive(Component)]
pub struct GameCamera;

/// The camera that renders everything to the screen.
/// It can be different from the `GameCamera` if doing any kind of
/// deferred rendering or pixel scaling.
#[derive(Component)]
pub struct FinalCamera;

// Systems
// ---

/// Spawn the main cameras.
fn init(mut cmd: Commands, options: Res<GameOptions>) {
    cmd.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(options.palette.darker),
                ..default()
            },
            ..default()
        },
        GameCamera,
        FinalCamera,
    ));
}
