use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::prelude::*;

const DATA_PATH: &str = ".data"; // If changed, update in `macros/lib.rs`

pub(super) fn plugin(app: &mut App) {
    #[cfg(not(target_arch = "wasm32"))]
    if let Err(e) = std::fs::create_dir_all(DATA_PATH) {
        warn!("Couldn't create the save directory {}: {}", DATA_PATH, e);
    };
    app.insert_resource(SaveData::load());
}

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
