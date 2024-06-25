use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::assets::ExampleAssets;

// ······
// Plugin
// ······

// Audio
// Uses bevy audio to play music or sounds. This contains some examples on how
// to set up audio, but it is disabled by default because audio varies greatly
// from project to project.
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_systems(OnEnter(GameState::Play), init)
        //     .add_systems(OnExit(GameState::Play), pause);
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

#[allow(dead_code)]
fn init(
    mut cmd: Commands,
    assets: Res<ExampleAssets>,
    ambient: Query<&AudioSink, With<AmbientMusic>>,
) {
    match ambient.get_single() {
        Ok(a) => {
            a.play();
        },
        Err(_) => {
            // Audio is added using component bundles
            // Using PlaybackSettings you can specify if it only plays once,
            // if it loops or even more complex behaviour, for example, to
            // despawn the entity when the audio is finished
            cmd.spawn((
                AudioBundle {
                    source: assets.ambient_music.clone(),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Loop,
                        volume: Volume::new(0.1),
                        ..default()
                    },
                },
                AmbientMusic,
            ));
        },
    }
}

#[allow(dead_code)]
fn pause(music: Query<&AudioSink>) {
    for music in music.iter() {
        music.pause();
    }
}
