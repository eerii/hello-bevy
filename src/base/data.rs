//! Defines persistent data structures.
//! For a more complete solution, look at <https://github.com/umut-sahin/bevy-persistent>

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::prelude::*;

// TODO: Wasm data persistence

/// The directory where the persistent data should be saved in.
const DATA_PATH: &str = ".data"; // If changed, update in `macros/lib.rs`

pub(super) fn plugin(app: &mut App) {
    #[cfg(not(target_arch = "wasm32"))]
    if let Err(e) = std::fs::create_dir_all(DATA_PATH) {
        warn!("Couldn't create the save directory {}: {}", DATA_PATH, e);
    };
    app.insert_resource(SaveData::load())
        .insert_resource(GameOptions::load());
}

// Resources
// ---

/// Stores options that can be configured on the menu, related to accesibility
/// and customization.
#[persistent]
#[derive(Default)]
pub struct GameOptions {
    /// The user configurable color palette of the game.
    pub palette: ColorPalette,
}

/// Base colors used in the game and the ui.
#[derive(Debug, Reflect, Serialize, Deserialize, Copy!)]
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

/// Used to store information about the player, the level and game progress.
#[persistent]
#[derive(Default)]
pub struct SaveData {
    /// Placeholder.
    pub test: bool,
}

// Helpers
// ---

/// Indicates that a `Resource` can be saved and loaded from disk.
/// This is implemented automatically when using `persistent`.
///
/// # Examples
///
/// ```
/// use game::prelude::*;
/// use serde::{Deserialize, Serialize};
///
/// #[persistent]
/// #[derive(Default)]
/// pub struct SomeData {
///     pub test: bool,
/// }
///
/// // The persistent data can be accessed in any system.
/// fn read(data: Res<SomeData>) {
///     info!("{:?}", data.test);
/// }
///
/// // Writing can be done in a few ways.
/// fn write(mut data: ResMut<SomeData>) {
///     // This will persist the new value.
///     data.update(|data| {
///         data.test = true;
///     });
///     // This will not until you call `persist` manually.
///     data.test = false;
///     data.persist();
/// }
/// ```
pub trait Persistent: Resource + Serialize + DeserializeOwned + Default {
    /// Reads a resource from disk if it exists. If it doesn't it returns the
    /// default value.
    fn load() -> Self;
    /// Reads the saved value of this resource and overwrites its current value.
    fn reload(&mut self);
    /// Serializes the data of this resource and saves it.
    fn persist(&self) -> Result<()>;
    /// Mutates the values of the resource using a closure and writes the result
    /// to disk after it is done.
    fn update(&mut self, f: impl Fn(&mut Self)) -> Result<()>;
    /// Returns the resource to its default value and saves it.
    fn reset(&mut self) -> Result<()>;
}
