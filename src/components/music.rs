//! Background music for the game.

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Play), init);
}

/// Plays background music when the game starts playing and stops when it is
/// paused. More complex logic might be useful to preserve play state and allow
/// crossfades.
fn init(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.spawn((
        AudioPlayer::new(assets.load("music/rain.ogg")),
        PlaybackSettings::LOOP.paused(),
        StateScoped(GameState::Play),
    ));
}
