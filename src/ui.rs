//! All of the Ui for the game.

use crate::prelude::*;

pub mod menu;
pub mod navigation;
pub mod widgets;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((menu::plugin, navigation::plugin, widgets::plugin));
}

/// The prelude of this module
pub mod prelude {
    pub use bevy_mod_picking::prelude::*;

    pub use super::{
        menu::MenuState,
        navigation::{NavActionEvent, NavBundle, NavContainer, Navigable},
        widgets::{Container, NavigableExt, Stylable, Widget},
    };
}
