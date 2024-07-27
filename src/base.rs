mod data;
mod later;
mod sets;
mod states;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((data::plugin, later::plugin, sets::plugin, states::plugin));
}

pub mod prelude {
    pub use super::{
        data::{Persistent, SaveData},
        later::LaterCommandExt,
        sets::PlaySet,
        states::GameState,
    };
}
