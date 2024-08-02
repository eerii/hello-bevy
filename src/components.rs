//! Some common components and their associated systems.

use crate::prelude::*;

pub mod camera;
pub mod music;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((camera::plugin, music::plugin));
}

/// The prelude for this module.
pub mod prelude {
    pub use super::camera::GameCamera;
}
