#![feature(path_add_extension)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
// #![warn(missing_docs)]

#[macro_use]
extern crate macro_rules_attribute;

// TODO: Documentation and code examples
// TODO: Readme
// TODO: Keybind remapping
// TODO: Text to speech
// TODO: Migrate proc macros to macro_rules_attribute?

use bevy::{prelude::*, window::WindowResolution};

mod assets;
mod base;
mod components;
#[macro_use]
mod helpers;
mod input;
mod ui;

pub mod prelude {
    pub use anyhow::{Context, Result};
    pub use bevy::{color::palettes::css, prelude::*, utils::HashMap};
    pub use macros::*;

    pub use super::{
        assets::prelude::*,
        base::prelude::*,
        components::prelude::*,
        input::prelude::*,
        ui::prelude::*,
        GamePlugin,
    };

    // Shorthands for derive macros
    macro_rules_attribute::derive_alias! {
        #[derive(Eq!)] = #[derive(Eq, PartialEq)];
        #[derive(Ord!)] = #[derive(Ord, PartialOrd, Eq!)];
        #[derive(Copy!)] = #[derive(Copy, Clone)];
        #[derive(Std!)] = #[derive(Debug, Copy!, Ord!, Hash)];
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // The embedded plugin, if enabled, must come before bevy's `AssetPlugin`
        #[cfg(feature = "embedded")]
        app.add_plugins(assets::embedded::plugin);

        // Default bevy plugins
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
            ui::plugin,
        ));
    }
}
