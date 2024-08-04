//! Bevy game template.
//! It uses plugins and submodules to structure the code.

#![feature(path_add_extension)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![warn(missing_docs)]

#[macro_use]
extern crate macro_rules_attribute;

use bevy::prelude::*;

pub mod assets;
pub mod base;
pub mod components;
#[macro_use]
pub mod helpers;
pub mod input;
pub mod prelude;
pub mod ui;

/// The base plugin for the game. It recursively adds all of the plugins
/// declared in submodules as well as the default plugin collection.
/// A plugin in bevy allows you to extend the `App` at the start of the game,
/// adding systems, resources and other plugins.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // The embedded plugin, if enabled, must come before bevy's `AssetPlugin`
        #[cfg(feature = "embedded")]
        app.add_plugins(assets::embedded::plugin);

        // Default bevy plugins
        let asset_plugin = AssetPlugin {
            meta_check: bevy::asset::AssetMetaCheck::Never,
            ..default()
        };

        let image_plugin = if cfg!(feature = "pixel_perfect") {
            ImagePlugin::default_nearest()
        } else {
            ImagePlugin::default()
        };

        let window_plugin = WindowPlugin {
            primary_window: Some(Window {
                title: "Hello Bevy".into(),
                canvas: Some("#bevy".into()),
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        };

        app.add_plugins(
            DefaultPlugins
                .set(asset_plugin)
                .set(image_plugin)
                .set(window_plugin),
        );

        // Game plugins
        app.add_plugins((
            assets::plugin,
            base::plugin,
            components::plugin,
            input::plugin,
            ui::plugin,
        ));

        // Debug plugins
        #[cfg(feature = "inspector")]
        app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
    }
}
