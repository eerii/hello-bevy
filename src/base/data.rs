//! Defines persistent data structures.
//! For a more complete solution, look at <https://github.com/umut-sahin/bevy-persistent>

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::prelude::*;

/// The directory where the persistent data should be saved in.
const DATA_PATH: &str = ".data";

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
#[derive(Reflect, Resource, Serialize, Deserialize)]
pub struct GameOptions {
    /// The user configurable color palette of the game.
    pub palette: ColorPalette,
    /// If the window is allowed to resize
    pub resizable: bool,
    /// The last saved resolution of the window
    pub resolution: UVec2,
}

persistent!(GameOptions);

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
#[derive(Reflect, Resource, Serialize, Deserialize, Default)]
pub struct SaveData {
    /// Placeholder.
    pub test: bool,
}

persistent!(SaveData);

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

// TODO: Look into macro_rules_attribute to derive this instead

/// Indicates that a `Resource` can be saved and loaded from disk.
/// This is implemented automatically when using `persistent`.
///
/// # Examples
///
/// ```
/// use game::prelude::*;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Reflect, Resource, Serialize, Deserialize, Default)]
/// pub struct SomeData {
///     pub test: bool,
/// }
/// persistent!(SomeData);
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
pub trait Persistent: Resource + Serialize + DeserializeOwned + Default + TypePath {
    /// Returns the path that this resource needs to write to.
    fn path() -> &'static str;
    /// Reads a resource from disk if it exists. If it doesn't it returns the
    /// default value.
    fn load() -> Self {
        let mut data = Self::default();
        data.reload();
        data
    }
    /// Reads the saved value of this resource and overwrites its current value.
    fn reload(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        let data = {
            let path = format!("{}/{}.toml", DATA_PATH, Self::path());
            std::fs::read_to_string(path).ok()
        };

        #[cfg(target_arch = "wasm32")]
        let data = (|| {
            let local_storage = web_sys::window()?.local_storage().ok()??;
            local_storage.get(Self::path()).ok()?
        })();

        *self = match data {
            Some(data) => toml::from_str(&data).unwrap_or_default(),
            None => Self::default(),
        };
    }
    /// Serializes the data of this resource and saves it.
    fn persist(&self) -> Result<()> {
        let name = Self::type_path();
        let data = toml::to_string(self)
            .with_context(|| format!("Failed to serialize data for {}", name))?;

        #[cfg(not(target_arch = "wasm32"))]
        {
            let path = format!("{}/{}.toml", DATA_PATH, Self::path());
            std::fs::write(path.clone(), data)
                .with_context(|| format!("Failed to save serialized data for {}", name))?;
        }

        #[cfg(target_arch = "wasm32")]
        {
            let local_storage = web_sys::window()
                .context("Error getting the JavaScript window")?
                .local_storage()
                .ok()
                .context("No access to localStorage")?
                .context("No access to localStorage")?;
            local_storage
                .set(Self::path(), &data)
                .ok()
                .with_context(|| format!("Failed to save serialized data for {}", name))?;
        }

        debug!("{} updated", name);
        Ok(())
    }

    /// Mutates the values of the resource using a closure and writes the result
    /// to disk after it is done.
    fn update(&mut self, f: impl Fn(&mut Self)) -> Result<()> {
        f(self);
        self.persist()
    }
    /// Returns the resource to its default value and saves it.
    fn reset(&mut self) -> Result<()> {
        *self = Self::default();
        self.persist()
    }
}
