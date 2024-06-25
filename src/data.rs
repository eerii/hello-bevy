use std::{fs, path::PathBuf};

use bevy::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::GameState;

// ······
// Plugin
// ······

// Game data
pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), init);
    }
}

// ·········
// Resources
// ·········

// Game options
// Useful for accesibility and the settings menu
#[derive(Debug, Default, Resource, Reflect, Serialize, Deserialize)]
pub struct GameOptions {
    test: bool,
}

// ·······
// Systems
// ·······

fn init(mut cmd: Commands) {
    #[cfg(feature = "persist")]
    {
        // Initialize a data storage and load game options from disk if they exist
        let path = if cfg!(target_arch = "wasm32") { "local" } else { ".data" };
        let data = DataStorage::new(path.into()).expect("couldn't initialize data storage");
        let options: GameOptions = data.read("options.toml").unwrap_or_default();
        // data.write("options.toml", &options);

        cmd.insert_resource(data);
        cmd.insert_resource(options);
    }
}

// ·······
// Helpers
// ·······

// Saves and loads persistent data under a directory
// TODO: Propper error handling
#[cfg(feature = "persist")]
#[derive(Debug, Default, Resource, Reflect, Clone)]
struct DataStorage {
    path: PathBuf,
}

#[cfg(feature = "persist")]
impl DataStorage {
    fn new(path: PathBuf) -> Option<Self> {
        fs::create_dir_all(path.clone()).ok()?;
        Some(Self { path })
    }

    fn read<R: DeserializeOwned>(&self, name: &str) -> Option<R> {
        let path = self.path.join(name);
        let Ok(data) = fs::read_to_string(path) else {
            return None;
        };
        toml::from_str::<R>(&data).ok()
    }

    #[allow(dead_code)]
    fn write<R: Serialize>(&self, name: &str, value: &R) -> Option<()> {
        let path = self.path.join(name);
        let data = toml::to_string(value).ok()?;
        fs::write(path, data).ok()?;
        Some(())
    }
}
