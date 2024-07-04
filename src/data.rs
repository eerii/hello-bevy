use std::{fs, path::PathBuf};

use bevy::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::GameState;

// ······
// Plugin
// ······

// Data persistence
// Used to create persistent serialized files with options or save data
// It saves and loads from toml any resource that needs to survive app reloads
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
#[derive(Debug, Default, Resource, Serialize, Deserialize)]
pub struct GameOptions {
    test: bool,
}

// Save data
// A place to save the player's progress
#[derive(Debug, Default, Resource, Serialize, Deserialize)]
pub struct SaveData {
    name: String,
}

// ·······
// Systems
// ·······

fn init(mut cmd: Commands) {
    #[cfg(feature = "persist")]
    {
        // Initialize a data storage and load game options from disk if they exist
        let path = if cfg!(target_arch = "wasm32") { "local" } else { ".data" };
        let Some(data) = DataStorage::new(path.into()) else {
            warn!("couldn't initialize data storage");
            return;
        };

        // Read the data if it exists
        let options: GameOptions = data.read("options.toml").unwrap_or_default();
        let save_data: GameOptions = data.read("save.toml").unwrap_or_default();

        // Write the new options
        // data.write("options.toml", &options);

        cmd.insert_resource(data);
        cmd.insert_resource(options);
        cmd.insert_resource(save_data);
    }
}

// ·······
// Helpers
// ·······

// Saves and loads persistent data under a directory
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
