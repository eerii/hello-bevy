use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::{ExampleAssets, GameState};

// ······
// Plugin
// ······

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Play), init_music)
            .add_systems(OnExit(GameState::Play), pause_music);
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

fn init_music(
    mut cmd: Commands,
    assets: Res<ExampleAssets>,
    ambient: Query<&AudioSink, With<AmbientMusic>>,
) {
    match ambient.get_single() {
        Ok(_) => {
            // a.play();
        },
        Err(_) => {
            cmd.spawn((
                AudioBundle {
                    source: assets.ambient_music.clone(),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Loop,
                        volume: Volume::new_relative(0.1),
                        paused: true, // [CHANGE]: Ambient music starts paused
                        ..default()
                    },
                },
                AmbientMusic,
            ));
        },
    }
}

fn pause_music(music: Query<&AudioSink>) {
    for music in music.iter() {
        music.pause();
    }
}
