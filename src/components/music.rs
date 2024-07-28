use bevy::audio::PlaybackMode;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Play), init);
}

fn init(mut cmd: Commands, music_assets: Res<AssetMap<MusicAssetKey>>) {
    cmd.spawn((
        AudioBundle {
            source: music_assets.get(&MusicAssetKey::Ambient),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                paused: true,
                ..default()
            },
        },
        StateScoped(GameState::Play),
    ));
}
