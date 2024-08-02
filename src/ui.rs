use crate::prelude::*;

mod menu;
mod navigation;
mod widgets;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((menu::plugin, navigation::plugin));
}

pub mod prelude {
    pub use bevy_mod_picking::prelude::Listener;
    pub use bevy_trait_query::RegisterExt;

    pub use super::{
        menu::MenuState,
        navigation::{NavActionEvent, NavBundle, NavContainer, Navigable},
        widgets::{Container, NavigableExt, Stylable, Widget},
    };
}
