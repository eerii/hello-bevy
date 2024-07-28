use crate::prelude::*;

mod navigation;
mod widgets;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((navigation::plugin, widgets::plugin));
    app.add_systems(OnEnter(GameState::Startup), init);
}

pub mod prelude {
    pub use bevy_trait_query::RegisterExt;

    pub use super::{
        navigation::{NavBundle, NavContainer, Navigable},
        widgets::{Container, NavigableExt, Stylable, Widget},
    };
}

fn init(mut cmd: Commands) {
    let mut root = cmd.ui_root();
    root.with_children(|node| {
        node.button("hey");
        node.button("hi").no_nav();
        node.button("hello");
    })
    .nav_container();
}
