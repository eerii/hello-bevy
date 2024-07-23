#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
// #![warn(missing_docs)]

// TODO: Bring back formatter and CI
// TODO: Documentation and code examples
// TODO: Data persistence (custom implementation)
//       If using derive macros, also use them for assets
// TODO: UI Widgets
//       Main menu
//       UI Navigation with input (custom implementation)
// TODO: Keybind remapping
// TODO: Text to speech

use bevy::{prelude::*, window::WindowResolution};

mod assets;
mod base;
mod components;
mod input;

pub mod prelude {
    pub use super::assets::prelude::*;
    pub use super::base::prelude::*;
    pub use super::components::prelude::*;
    pub use super::input::prelude::*;
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Default bevy plugins

        // The window plugin specifies the main window properties like its size,
        // if it is resizable and its title
        let window_plugin = WindowPlugin {
            primary_window: Some(Window {
                title: "Hello Bevy".into(),
                resolution: WindowResolution::new(600., 600.),
                resizable: false,
                canvas: Some("#bevy".into()),
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        };

        app.add_plugins(DefaultPlugins.set(window_plugin));

        // Game plugins
        app.add_plugins((
            assets::plugin,
            base::plugin,
            components::plugin,
            input::plugin,
        ));
    }
}
