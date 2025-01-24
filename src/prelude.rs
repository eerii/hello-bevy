//! Useful declarations grouped together for ease of use.
//! Includes modules from this crate and some redeclarations from dependencies.

pub use anyhow::{Context, Result};
pub use bevy::{
    color::palettes::css,
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

pub use crate::{
    components::camera::{FinalCamera, GameCamera},
    data::{GameOptions, PersistentExt, SaveData},
    helpers::LaterCommandExt,
    single,
    single_mut,
    states::{GameState, PlaySet},
    GamePlugin,
    Persistent,
};

// Shorthands for derive macros
macro_rules_attribute::derive_alias! {
    #[derive(Eq!)] = #[derive(Eq, PartialEq)];
    #[derive(Ord!)] = #[derive(Ord, PartialOrd, Eq!)];
    #[derive(Copy!)] = #[derive(Copy, Clone)];
    #[derive(Std!)] = #[derive(Debug, Copy!, Ord!, Hash)];
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
