use crate::prelude::*;

mod data;
mod later;
mod sets;
mod states;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((data::plugin, later::plugin, sets::plugin, states::plugin));
}

pub mod prelude {
    pub use super::{
        data::{GameOptions, Persistent, SaveData},
        later::LaterCommandExt,
        sets::PlaySet,
        states::GameState,
    };
}
