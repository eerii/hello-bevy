//! Speech module

use bevy::prelude::*;
#[cfg(feature = "navigation")]
use bevy_alt_ui_navigation_lite::prelude::*;
use tts::{Features, Tts};

use crate::data::{GameOptions, Persistent};

// ······
// Plugin
// ······

/// Text-to-speech
/// Uses tts-rs to generate text to speech commands for screen readers
pub struct SpeechPlugin;

impl Plugin for SpeechPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);

        #[cfg(feature = "navigation")]
        app.add_systems(
            Update,
            navigation_speech.run_if(in_state(crate::GameState::Menu)),
        );
    }
}

// ·········
// Resources
// ·········

/// Resource containing a text to speech system
/// It is used to read ui element names while navigating menus
#[derive(Resource)]
pub struct Speech {
    /// The core component of the tts package
    pub tts: Tts,
}

impl Speech {
    /// Say a text aloud, optionally interrupting the previous one
    pub fn speak(&mut self, text: &str, interrupt: bool) {
        let _ = self.tts.speak(text, interrupt);
    }
}

/// This component gives an element a text to speech description so that it can
/// be read aloud
#[derive(Component)]
pub struct SpeechTag(pub String);

// ·······
// Systems
// ·······

/// Create a text to speech system and save it as a resource
fn init(mut cmd: Commands) {
    let Ok(mut tts) = Tts::default() else {
        return;
    };

    info!(
        "{} screen reader is available on this platform.",
        if Tts::screen_reader_available() { "a" } else { "no" }
    );

    let Features { voice, .. } = tts.supported_features();

    let _ = tts.set_pitch(tts.normal_pitch());

    'v: {
        if !voice {
            break 'v;
        }
        let Ok(voices) = tts.voices() else {
            break 'v;
        };
        if let Some(voice) = voices
            .iter()
            .find(|&v| v.language().primary_language() == "en")
        {
            let _ = tts.set_voice(voice);
            break 'v;
        }
        if let Some(voice) = voices.first() {
            let _ = tts.set_voice(voice);
        }
    }

    cmd.insert_resource(Speech { tts });
}

#[cfg(feature = "navigation")]
fn navigation_speech(
    focusables: Query<Entity, With<Focusable>>,
    speech_tag: Query<(Entity, &SpeechTag, Option<&Parent>)>,
    options: Res<Persistent<GameOptions>>,
    speech: Option<ResMut<Speech>>,
    mut nav_event_reader: EventReader<NavEvent>,
) {
    let Some(mut speech) = speech else {
        return;
    };
    if !options.text_to_speech {
        return;
    }

    for event in nav_event_reader.read() {
        match event {
            NavEvent::FocusChanged { to, from: _ } => speak_focusable(
                to.first(),
                &focusables,
                &speech_tag,
                &mut speech,
            ),
            NavEvent::InitiallyFocused(to) => speak_focusable(
                to,
                &focusables,
                &speech_tag,
                &mut speech,
            ),
            _ => {},
        }
    }
}

// ·······
// Helpers
// ·······

fn speak_focusable(
    to: &Entity,
    focusables: &Query<Entity, With<Focusable>>,
    speech_tag: &Query<(Entity, &SpeechTag, Option<&Parent>)>,
    speech: &mut Speech,
) {
    for (entity, tag, parent) in speech_tag.iter() {
        let focus = focusables.get(entity).unwrap_or({
            let Some(parent) = parent else {
                continue;
            };
            let Ok(focus) = focusables.get(parent.get()) else {
                continue;
            };
            focus
        });
        if *to == focus {
            speech.speak(&tag.0, true);
            return;
        }
    }
}
