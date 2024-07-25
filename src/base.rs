mod data;
mod sets;
mod states;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((data::plugin, sets::plugin, states::plugin));
}

pub mod prelude {
    pub use super::{
        data::{Persistent, SaveData},
        sets::PlaySet,
        states::GameState,
    };
}
