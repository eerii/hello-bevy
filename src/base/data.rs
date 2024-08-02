use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::prelude::*;

const DATA_PATH: &str = ".data"; // If changed, update in `macros/lib.rs`

pub(super) fn plugin(app: &mut App) {
    #[cfg(not(target_arch = "wasm32"))]
    if let Err(e) = std::fs::create_dir_all(DATA_PATH) {
        warn!("Couldn't create the save directory {}: {}", DATA_PATH, e);
    };
    app.insert_resource(SaveData::load())
        .insert_resource(GameOptions::load());
}

/// Persistent data across game restarts.
/// Stores options that can be configured on the menu, related to accesibility
/// and customization.
#[derive(Default)]
#[persistent(name = "options")]
pub struct GameOptions {
    pub palette: ColorPalette,
}

/// Base colors used in the game and the ui.
#[derive(Reflect, Serialize, Deserialize, Copy!)]
pub struct ColorPalette {
    pub light: Color,
    pub primary: Color,
    pub dark: Color,
    pub darker: Color,
}

impl ColorPalette {
    pub fn monocrome(base: Color) -> Self {
        Self {
            light: base.with_luminance(0.7).lighter(0.6),
            primary: base.with_luminance(0.5),
            dark: base.with_luminance(0.3),
            darker: base.with_luminance(0.3).darker(0.07),
        }
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::monocrome(css::ROYAL_BLUE.into())
    }
}

/// Persistent data across game restarts.
/// Used to store information about the player, the level and game progress.
#[derive(Default)]
#[persistent(name = "save")]
pub struct SaveData {
    pub test: bool,
}

#[allow(dead_code)]
pub trait Persistent: Resource + Serialize + DeserializeOwned + Default {
    fn load() -> Self;
    fn reload(&mut self);
    fn persist(&self) -> Result<()>;
    fn update(&mut self, f: impl Fn(&mut Self)) -> Result<()>;
    fn reset(&mut self) -> Result<()>;
}
