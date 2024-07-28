use std::{
    any::TypeId,
    sync::{LazyLock, Mutex},
};

use bevy::reflect::{GetTypeRegistration, ReflectFromPtr};

use crate::prelude::*;

#[cfg(feature = "embedded")]
pub(crate) mod embedded;
mod fonts;
mod meta;
mod music;
mod sound;

static ASSET_MAP: LazyLock<Mutex<Vec<TypeId>>> = LazyLock::new(|| Mutex::new(vec![]));

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((fonts::plugin, meta::plugin, music::plugin, sound::plugin))
        .add_systems(Update, check_loaded.run_if(in_state(GameState::Startup)));
}

pub mod prelude {
    pub use super::{
        fonts::FontAssetKey,
        meta::MetaAssetKey,
        music::MusicAssetKey,
        sound::SoundAssetKey,
        AssetExt,
        AssetKey,
        AssetMap,
    };
}

/// Represents a handle to any asset type
pub trait AssetKey:
    Sized + Eq + std::hash::Hash + Reflect + FromReflect + TypePath + GetTypeRegistration
{
    type Asset: Asset;
}

/// A resource that holds asset `Handle`s for a particular type of `AssetKey`
/// Easy to access on any system using `Res<AssetMap<...>>`
#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(AssetsLoaded)]
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
    /// Returns a weak clone of the asset handle
    pub fn get(&self, key: &K) -> Handle<K::Asset> {
        self[key].clone_weak()
    }
}

#[reflect_trait]
trait AssetsLoaded {
    /// Check if all of the assets are loaded
    fn all_loaded(&self, asset_server: &AssetServer) -> bool;
}

impl<K: AssetKey> AssetsLoaded for AssetMap<K> {
    fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}

pub trait AssetExt {
    fn load_asset<K: AssetKey>(&mut self) -> &mut Self
    where
        AssetMap<K>: FromWorld;
}

impl AssetExt for App {
    fn load_asset<K: AssetKey>(&mut self) -> &mut Self
    where
        AssetMap<K>: FromWorld,
    {
        ASSET_MAP.lock().unwrap().push(TypeId::of::<AssetMap<K>>());
        self.init_resource::<AssetMap<K>>()
            .register_type::<AssetMap<K>>()
    }
}

fn check_loaded(world: &mut World) {
    let mut map = ASSET_MAP.lock().unwrap();
    let mut loaded = vec![];

    for id in map.iter() {
        match is_resource_loaded(*id, world) {
            Ok(l) if !l => continue,
            Err(e) => warn!("{}", e),
            _ => {},
        }
        loaded.push(*id);
    }

    (*map).retain(|x| !loaded.contains(x));

    if map.len() == 0 {
        let mut next_state = world
            .get_resource_mut::<NextState<GameState>>()
            .expect("NextState should exist");
        next_state.set(GameState::Menu);
    }
}

fn is_resource_loaded(id: TypeId, world: &World) -> Result<bool> {
    // Get world resources
    let asset_server = world
        .get_resource::<AssetServer>()
        .expect("Bevy's asset server should exist");
    let registry = world
        .get_resource::<AppTypeRegistry>()
        .expect("Bevy's type registry should exist");
    let registry = registry.read();

    // Get the AssetMap component id and raw pointer
    let registration = registry.get(id).context("Id is not registered")?;
    let component_id = world
        .components()
        .get_resource_id(registration.type_id())
        .context("Couldn't get the component id of the resource")?;
    let ptr = world
        .get_resource_by_id(component_id)
        .context("The resource is not registred")?;

    // Convert the pointer into a Reflect trait
    // This should be safe
    let reflect_from_ptr = registration
        .data::<ReflectFromPtr>()
        .context("Type registration should exist")?;
    // SAFETY: from the context it is known that `ReflectFromPtr` was made for the
    // type of the `MutUntyped`
    let resource: &dyn Reflect = unsafe { reflect_from_ptr.as_reflect(ptr) };

    // Get the LoadedAsset trait registration
    let loaded_trait = registry
        .get_type_data::<ReflectAssetsLoaded>(id)
        .context("The AssetLoaded trait is not registered")?;

    // Convert the AssetMap dyn Reflect object into a dyn LoadedAsset trait
    let resource = loaded_trait
        .get(resource)
        .context("The resource doesn't implement the LoadedAsset trait")?;

    // Check if all of the assets are loaded
    Ok(resource.all_loaded(asset_server))
}
