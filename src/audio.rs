//! Audio loading module

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::{assets::ExampleAssets, GameState};

// ······
// Plugin
// ······

/// Audio
/// Uses bevy audio to play music or sounds. This contains some examples on how
/// to set up audio, but it is disabled by default because audio varies greatly
/// from project to project.
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Play), init);
    }
}

// ··········
// Components
// ··········

#[derive(Component)]
struct AmbientMusic;

// ·······
// Systems
// ·······

/// Audio is added using component bundles
/// Using PlaybackSettings you can specify if it only plays once, if it loops or
/// even more complex behaviour, for example, to despawn the entity when the
/// audio is finished
/// CHANGE: Enable or disable background music and other sounds
fn init(mut cmd: Commands, assets: Res<ExampleAssets>) {
    cmd.spawn((
        AudioBundle {
            source: assets.ambient_music.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new(0.1),
                paused: true, // Starts paused to avoid annoyances
                ..default()
            },
        },
        AmbientMusic,
        StateScoped(GameState::Play),
    ));
}
