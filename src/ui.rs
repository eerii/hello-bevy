use bevy::prelude::*;

// use crate::{prelude::*, ui::navigation::*};

mod navigation;
mod widgets;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(navigation::plugin);
    // app.register_component_as::<dyn Navigable, MenuItem>()
    //     .add_systems(OnEnter(GameState::Startup), init);
}

pub mod prelude {
    pub use bevy_trait_query::RegisterExt;

    pub use super::{
        navigation::NavBundle,
        widgets::{Container, Stylable, Widget},
    };
}

// #[derive(Component)]
// struct MenuItem {
//     label: String,
// }
//
// impl Navigable for MenuItem {
//     fn label(&self) -> String {
//         self.label.clone()
//     }
//
//     fn action(&self) {
//         info!("action {}", self.label());
//     }
// }
//
// fn init(mut cmd: Commands) {
//     let mut root = cmd.ui_root();
//     root.with_children(|node| {
//         node.button("hey").insert(MenuItem {
//             label: "hey".into(),
//         });
//         node.button("hi").insert(MenuItem { label: "hi".into() });
//         node.button("hello").insert(MenuItem {
//             label: "hello".into(),
//         });
//     })
//     .insert(NavContainer::default());
// }
