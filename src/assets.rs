#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use bevy::prelude::*;

use crate::GameState;

// ······
// Plugin
// ······

// Asset loader
pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadingData::default())
            .add_systems(Startup, load_core)
            .add_systems(
                OnEnter(GameState::Loading),
                load_example,
            )
            .add_systems(
                Update,
                check_load_state.run_if(in_state(GameState::Loading)),
            );
    }
}

// ·········
// Resources
// ·········

#[derive(Resource, Debug, Default)]
struct LoadingData {
    assets: Vec<UntypedHandle>,
    loaded: usize,
    total: usize,
}

impl LoadingData {
    // Loads an asset into the server and adds it to the list to keep track of its
    // state
    fn load<T: Asset>(&mut self, asset_server: &AssetServer, path: &'static str) -> Handle<T> {
        let handle = asset_server.load(path);
        debug!("Loading \"{:?}\"", handle.path());

        self.assets.push(handle.clone().into());
        self.total += 1;

        handle
    }

    // Returns the current loaded assets and the total assets registered
    fn current(&mut self, asset_server: &AssetServer) -> (usize, usize) {
        let mut pop_list: Vec<usize> = Vec::new();
        for (i, asset) in self.assets.iter().enumerate() {
            if let Some(state) = asset_server.get_load_states(asset) {
                if let bevy::asset::RecursiveDependencyLoadState::Loaded = state.2 {
                    pop_list.push(i);
                    self.loaded += 1;
                    debug!(
                        "\"{:?}\" loaded! ({}/{})",
                        asset.path(),
                        self.loaded,
                        self.total
                    );
                }
            }
        }

        for i in pop_list.iter() {
            self.assets.remove(*i);
        }

        (self.loaded, self.total)
    }
}

// Assets for the splash screen and menus
// They are loaded inmediately after the app is fired, so they have no effect on
// loading state
#[derive(Resource)]
pub struct CoreAssets {
    pub bevy_icon: Handle<Image>,
    pub font: Handle<Font>,
}

// Example assets
// They are loaded during the loading state, showing the progress
#[derive(Resource)]
pub struct ExampleAssets {
    pub boing: Handle<AudioSource>,
    pub ambient_music: Handle<AudioSource>,
}

// ·······
// Systems
// ·······

fn load_core(mut cmd: Commands, asset_server: Res<AssetServer>) {
    // They use the asset server directly
    let assets = CoreAssets {
        bevy_icon: asset_server.load("icons/bevy.png"),
        font: asset_server.load("fonts/sans.ttf"),
    };

    cmd.insert_resource(assets);
}

fn load_example(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut loading_data: ResMut<LoadingData>,
) {
    // They use the loading data manager, which tracks if they are loaded
    let assets = ExampleAssets {
        boing: loading_data.load(&asset_server, "sounds/boing.ogg"),
        ambient_music: loading_data.load(&asset_server, "music/soundscape.ogg"),
    };

    cmd.insert_resource(assets);
}

fn check_load_state(
    mut next_state: ResMut<NextState<GameState>>,
    mut loading_data: ResMut<LoadingData>,
    asset_server: Res<AssetServer>,
) {
    let (loaded, total) = loading_data.current(&asset_server);
    if loaded == total {
        next_state.set(if cfg!(feature = "menu") { GameState::Menu } else { GameState::Play });
    }
}
