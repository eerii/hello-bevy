//! Data persistence module

use bevy::prelude::*;
#[cfg(feature = "persist")]
pub use bevy_persistent::prelude::Persistent;
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "persist"))]
pub use self::alt::Persistent;
use crate::GameState;

// ······
// Plugin
// ······

/// Data persistence
/// Used to create persistent serialized files with options or save data
/// It saves and loads from toml any resource that needs to survive app reloads
pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Startup), init_data);
    }
}

// ·········
// Resources
// ·········

/// Game options
/// Useful for accesibility and the settings menu
/// CHANGE: Add any configurable game options here
#[derive(Resource, Serialize, Deserialize)]
pub struct GameOptions {
    /// Base color of the game, used for backgrounds, etc
    pub base_color: Color,
    /// Accent color, meant to contrast with the base color
    pub accent_color: Color,

    /// Controlls if text to speech is enabled for menu navigation
    #[cfg(feature = "tts")]
    pub text_to_speech: bool,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            base_color: Color::srgb(0.3, 0.5, 0.9),
            accent_color: Color::srgb(0.3, 0.5, 0.9),
            #[cfg(feature = "tts")]
            text_to_speech: default(),
        }
    }
}

/// Save data
/// A place to save the player's progress
/// CHANGE: Add relevant save data here
#[derive(Default, Resource, Serialize, Deserialize)]
pub struct SaveData {
    name: String,
}

/// When persist is not enabled, this wrapper just serves
/// as a placeholder to allow to use the resouces regularlly
#[cfg(not(feature = "persist"))]
mod alt {
    use std::ops::{Deref, DerefMut};

    use super::*;

    /// Placeholder persistent resource for when the persist feature is disabled
    /// This does nothing, just derefs to the inner value
    #[derive(Resource)]
    pub struct Persistent<T>(pub T);

    impl<T> Deref for Persistent<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> DerefMut for Persistent<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl<T> Persistent<T> {
        /// Updates the inner resource with a closure
        #[allow(clippy::result_unit_err)]
        pub fn update(&mut self, updater: impl Fn(&mut T)) -> Result<(), ()> {
            updater(&mut self.0);
            Ok(())
        }
    }
}

// ·······
// Systems
// ·······
#[cfg(feature = "persist")]
pub(crate) fn init_data(mut cmd: Commands) {
    let path = std::path::Path::new(if cfg!(target_arch = "wasm32") { "local" } else { ".data" });
    info!("{:?}", path);

    cmd.insert_resource(
        Persistent::<GameOptions>::builder()
            .name("game options")
            .format(bevy_persistent::StorageFormat::Toml)
            .path(path.join("options.toml"))
            .default(GameOptions::default())
            .revertible(true)
            .revert_to_default_on_deserialization_errors(true)
            .build()
            .expect("failed to initialize game options"),
    );

    cmd.insert_resource(
        Persistent::<SaveData>::builder()
            .name("save data")
            .format(bevy_persistent::StorageFormat::Toml)
            .path(path.join("save.toml"))
            .default(SaveData::default())
            .revertible(true)
            .revert_to_default_on_deserialization_errors(true)
            .build()
            .expect("failed to initialize save data"),
    );
}

#[cfg(not(feature = "persist"))]
pub(crate) fn init_data(mut cmd: Commands) {
    cmd.insert_resource(Persistent(GameOptions::default()));
    cmd.insert_resource(Persistent(SaveData::default()));
}
