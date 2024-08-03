//! Defines persistent data structures.
//! For a more complete solution, look at <https://github.com/umut-sahin/bevy-persistent>

use bevy::window::{PrimaryWindow, WindowResized};
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
        .insert_resource(GameOptions::load())
        .add_systems(OnEnter(GameState::Startup), init)
        .add_systems(Update, on_resize.run_if(on_event::<WindowResized>()));
}

// Resources
// ---

/// Stores options that can be configured on the menu, related to accesibility
/// and customization.
#[persistent]
pub struct GameOptions {
    /// The user configurable color palette of the game.
    pub palette: ColorPalette,
    /// If the window is allowed to resize
    pub resizable: bool,
    /// The last saved resolution of the window
    pub resolution: UVec2,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            palette: ColorPalette::default(),
            resizable: false,
            resolution: UVec2::new(600, 600),
        }
    }
}

/// Used to store information about the player, the level and game progress.
#[persistent]
#[derive(Default)]
pub struct SaveData {
    /// Placeholder.
    pub test: bool,
}

// Systems
// ---

/// When the game starts, set the window resolution and resizability to the
/// value read in the options
fn init(mut window: Query<&mut Window, With<PrimaryWindow>>, options: Res<GameOptions>) {
    let mut window = single_mut!(window);
    let res = options.resolution.as_vec2();
    window.resolution.set(res.x, res.y);
    window.resizable = options.resizable;
}

/// When the window is resized, updates the saved resolution
fn on_resize(mut resize_events: EventReader<WindowResized>, mut options: ResMut<GameOptions>) {
    for event in resize_events.read() {
        let _ = options.update(|options| {
            options.resolution = UVec2::new(event.width as u32, event.height as u32);
        });
    }
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
