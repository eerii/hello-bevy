//! Bevy game template.
//! It uses plugins and submodules to structure the code.

#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![warn(missing_docs)]

#[macro_use]
extern crate macro_rules_attribute;

use bevy::prelude::*;

pub mod components;
pub mod data;
pub mod helpers;
pub mod prelude;
pub mod states;

/// The base plugin for the game. It recursively adds all of the plugins
/// declared in submodules as well as the default plugin collection.
/// A plugin in bevy allows you to extend the `App` at the start of the game,
/// adding systems, resources and other plugins.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let window_plugin = WindowPlugin {
            primary_window: Some(Window {
                title: "Hello Bevy".into(),
                canvas: Some("#bevy".into()),
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        };

        let log_plugin = bevy::log::LogPlugin {
            filter: "warn,wgpu=error,game=debug".into(),
            ..default()
        };

        app.add_plugins((
            DefaultPlugins.set(window_plugin).set(log_plugin),
            components::plugin,
            data::plugin,
            helpers::plugin,
            states::plugin,
        ));

        #[cfg(feature = "embedded")]
        app.add_plugins(bevy_embedded_assets::EmbeddedAssetPlugin::default());
    }
}
