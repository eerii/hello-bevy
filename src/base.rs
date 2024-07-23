mod sets;
mod states;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((sets::plugin, states::plugin));
}

pub mod prelude {
    pub use super::sets::PlaySet;
    pub use super::states::GameState;
}
