//! Speech module

use bevy::prelude::*;
#[cfg(feature = "navigation")]
use bevy_alt_ui_navigation_lite::prelude::*;
use tts::{Features, Tts};

use crate::{
    data::{GameOptions, Persistent},
    GameState,
};

// ······
// Plugin
// ······

/// Text-to-speech
/// Uses tts-rs to generate text to speech commands for screen readers
pub struct SpeechPlugin;

impl Plugin for SpeechPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), init);
        #[cfg(feature = "navigation")]
        app.add_systems(
            Update,
            navigation_speech.run_if(in_state(GameState::Menu)),
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
    let Ok(mut tts) = Tts::default() else { return };

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
        let Ok(voices) = tts.voices() else { break 'v };

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
    query: Query<
        (
            Entity,
            Option<&Focusable>,
            Option<&Children>,
        ),
        With<Node>,
    >,
    speech_tag: Query<&SpeechTag>,
    options: Res<Persistent<GameOptions>>,
    speech: Option<ResMut<Speech>>,
    mut nav_event_reader: EventReader<NavEvent>,
) {
    let Some(mut speech) = speech else { return };
    if !options.text_to_speech {
        return;
    }

    for event in nav_event_reader.read() {
        let to = match event {
            NavEvent::FocusChanged { to, from: _ } => to.first(),
            NavEvent::InitiallyFocused(to) => to,
            _ => {
                continue;
            },
        };
        for (entity, focusable, _) in query.iter() {
            if focusable.is_none() || entity != *to {
                continue;
            }
            speak_focusable(
                entity,
                &query,
                &speech_tag,
                &mut speech,
                true,
            );
        }
    }
}

// ·······
// Helpers
// ·······

fn speak_focusable(
    current: Entity,
    query: &Query<
        (
            Entity,
            Option<&Focusable>,
            Option<&Children>,
        ),
        With<Node>,
    >,
    speech_tag: &Query<&SpeechTag>,
    speech: &mut Speech,
    interrupt: bool,
) -> bool {
    let Ok((entity, _, children)) = query.get(current) else { return false };

    let mut interrupt = interrupt;
    if let Ok(tag) = speech_tag.get(entity) {
        speech.speak(&tag.0, interrupt);
        interrupt = false;
    }

    if let Some(children) = children {
        for &child in children {
            interrupt = speak_focusable(
                child, query, speech_tag, speech, interrupt,
            );
        }
    }

    interrupt
}
