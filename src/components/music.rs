//! Background music for the game.

use bevy::audio::PlaybackMode;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Play), init);
}

/// Plays background music when the game starts playing and stops when it is
/// paused.
fn init(mut cmd: Commands, music_assets: Res<AssetMap<MusicAssetKey>>) {
    cmd.spawn((
        AudioPlayer(music_assets.get(&MusicAssetKey::Ambient)),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            paused: true,
            ..default()
        },
        // Here more complex logic might be useful to preserve play state and allow crossfades
        StateScoped(GameState::Play),
    ));
}
