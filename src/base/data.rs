use bevy::prelude::*;
use macros::persistent;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

const DATA_PATH: &str = ".data"; // If changed, update in `macros/lib.rs`

pub(super) fn plugin(app: &mut App) {
    let _ = std::fs::create_dir_all(DATA_PATH);
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
    fn persist(&self);
    fn update(&mut self, f: impl Fn(&mut Self));
    fn reset(&mut self);
}
