use bevy::{prelude::*, utils::HashMap};

mod fonts;
mod meta;
mod music;
mod sound;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((fonts::plugin, meta::plugin, music::plugin, sound::plugin));
}

pub mod prelude {
    pub use super::{
        fonts::FontAssetKey,
        meta::MetaAssetKey,
        music::MusicAssetKey,
        sound::SoundAssetKey,
        AssetKey,
        AssetMap,
    };
}

/// Represent a handle to any asset type
pub trait AssetKey: Sized {
    type Asset: Asset;
}

/// A resource that holds asset `Handle`s for a particular type of `AssetKey`
#[derive(Resource, Deref, DerefMut)]
pub struct AssetMap<K: AssetKey>(HashMap<K, Handle<K::Asset>>);

impl<K: AssetKey, T> From<T> for AssetMap<K>
where
    T: Into<HashMap<K, Handle<K::Asset>>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<K: AssetKey> AssetMap<K> {
    /// Check if all of the assets are loaded
    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}
