use bevy::{audio::PlaybackMode, prelude::*};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Play), init);
}

fn init(mut cmd: Commands, music_assets: Res<AssetMap<MusicAssetKey>>) {
    cmd.spawn((
        AudioBundle {
            source: music_assets[&MusicAssetKey::Ambient].clone_weak(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                paused: true,
                ..default()
            },
        },
        StateScoped(GameState::Play),
    ));
}
