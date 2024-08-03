//! Global helper functions and macros.

use bevy::ecs::world::DeferredWorld;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// The prelude of this module.
pub mod prelude {
    pub use super::{color_from_palette, ColorPalette};
    pub use crate::{component_palette, persistent, single, single_mut};
}

/// Gets a single component from a `Query` or returns gracefully (no panic).
#[macro_export]
macro_rules! single {
    ($q:expr, $r:expr) => {
        match $q.get_single() {
            Ok(m) => m,
            _ => {
                debug!("get single failed for ${}", stringify!($e));
                $r
            },
        }
    };
    ($q:expr) => {
        single!($q, return)
    };
}

/// Gets a single mutable component from a `Query` or returns gracefully (no
/// panic).
#[macro_export]
macro_rules! single_mut {
    ($q:expr, $r:expr) => {
        match $q.get_single_mut() {
            Ok(m) => m,
            _ => {
                debug!("get single mut failed for ${}", stringify!($e));
                $r
            },
        }
    };
    ($q:expr) => {
        single_mut!($q, return)
    };
}

/// Declares a bevy resource that can serialize data locally and persist it
/// between game restarts.
#[macro_export]
macro_rules! persistent {
    ($i:ident) => {
        impl Persistent for $i {
            #[inline]
            fn path() -> &'static str {
                stringify!($i)
            }
        }
    };
}

/// Base colors used in the game and the ui.
#[derive(Debug, Reflect, Serialize, Deserialize, Copy!)]
pub struct ColorPalette {
    /// Lighter color.
    pub light: Color,
    /// Base color.
    pub primary: Color,
    /// Darker color.
    pub dark: Color,
    /// Much darker color.
    pub darker: Color,
}

impl ColorPalette {
    /// Builds a color palette from shades of a single color.
    pub fn monocrome(base: Color) -> Self {
        Self {
            light: base.with_luminance(0.7).lighter(0.6),
            primary: base.with_luminance(0.5),
            dark: base.with_luminance(0.3),
            darker: base.with_luminance(0.3).darker(0.07),
        }
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::monocrome(css::ROYAL_BLUE.into())
    }
}

/// Converts from a named palette field to the corresponding color.
pub fn color_from_palette(world: &DeferredWorld, field: &'static str) -> Color {
    let palette = world
        .get_resource::<GameOptions>()
        .map(|data| data.palette)
        .unwrap_or_default();
    let Some(reflect) = palette.field(field) else { return css::RED.into() };
    reflect
        .downcast_ref::<Color>()
        .cloned()
        .unwrap_or(css::RED.into())
}

/// Creates a custom component that updates another component of the Ui once it
/// is added using the global palette. This is done automatically by using the
/// `register_component_hooks` with a custom `Component` implementation.
///
/// The parameters are the following:
/// - $i: The name of the custom component. This needs to be a tuple struct with
///   one string literal field.
/// - $c: The name of the component to modify.
/// - $d: The default field of `ColorPalette` to use.
/// - $f: Optionally, a closure taking a `DeferredWorld`, `Entity` and
///   calculated palette `Color` to manually assign the color to the entity.
///   This is useful if the component to modify doesn't directly dereference to
///   something that implements `From<Color>` or for more complex logic.
#[macro_export]
macro_rules! component_palette {
    ($i:ident, $c:ident, $d:literal, $f:expr) => {
        impl Default for $i {
            fn default() -> Self {
                Self($d)
            }
        }

        impl Component for $i {
            const STORAGE_TYPE: bevy::ecs::component::StorageType =
                bevy::ecs::component::StorageType::Table;

            fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
                hooks.on_add(|world, entity, _id| {
                    let field = world.get::<$i>(entity).cloned().unwrap_or_default().0;
                    let color = color_from_palette(&world, field);
                    $f(world, entity, color);
                });
            }
        }
    };

    ($i:ident, $c:ident, $d:literal) => {
        component_palette!($i, $c, $d, |mut world: bevy::ecs::world::DeferredWorld,
                                        entity: Entity,
                                        color: Color| {
            let Some(mut component) = world.get_mut::<$c>(entity) else {
                return;
            };
            *component = color.into()
        });
    };
}
