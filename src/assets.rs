#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use crate::{ui::*, GameOptions, GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_persistent::Persistent;
use iyes_progress::prelude::*;

#[cfg(debug_assertions)]
const SPLASH_TIME: f32 = 0.1;
#[cfg(not(debug_assertions))]
const SPLASH_TIME: f32 = 2.;

// ······
// Plugin
// ······

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(LoadingState::new(GameState::Loading))
            .init_collection::<GameAssets>()
            .add_collection_to_loading_state::<_, ExampleAssets>(GameState::Loading)
            .add_plugins((ProgressPlugin::new(GameState::Loading)
                .continue_to(GameState::Menu)
                .track_assets(),))
            .add_systems(Update, init_splash.run_if(in_state(GameState::Loading)))
            .add_systems(OnExit(GameState::Loading), clear_loading)
            .add_systems(
                Update,
                (check_splash_finished.track_progress(), check_progress)
                    .run_if(in_state(GameState::Loading))
                    .after(LoadingStateSet(GameState::Loading)),
            );
    }
}

// ·········
// Resources
// ·········

// Assets for the splash screen and menus
// They are loaded inmediately after the app is fired, no effect on loading state
#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "icons/bevy.png")]
    pub bevy_icon: Handle<Image>,

    #[asset(path = "fonts/sans.ttf")]
    pub font: Handle<Font>,
}

// Example assets
#[derive(AssetCollection, Resource)]
pub struct ExampleAssets {
    #[asset(path = "sounds/boing.ogg")]
    pub boing: Handle<AudioSource>,

    #[asset(path = "music/soundscape.ogg")]
    pub ambient_music: Handle<AudioSource>,
}

// ··········
// Components
// ··········

#[derive(Component)]
struct SplashTimer(Timer);

#[derive(Component)]
struct ProgressBar;

// ·······
// Systems
// ·······

fn init_splash(
    mut cmd: Commands,
    node: Query<Entity, With<UiNode>>,
    assets: Res<GameAssets>,
    mut has_init: Local<bool>,
) {
    if *has_init {
        return;
    }

    if let Ok(node) = node.get_single() {
        if let Some(mut node) = cmd.get_entity(node) {
            node.with_children(|parent| {
                parent.spawn(ImageBundle {
                    image: UiImage {
                        texture: assets.bevy_icon.clone(),
                        ..default()
                    },
                    style: Style {
                        width: Val::Px(128.),
                        ..default()
                    },
                    ..default()
                });
            });
        }

        *has_init = true;
    }

    cmd.spawn(SplashTimer(Timer::from_seconds(
        SPLASH_TIME,
        TimerMode::Once,
    )));
}

fn check_splash_finished(time: Res<Time>, mut timer: Query<&mut SplashTimer>) -> Progress {
    if let Ok(mut timer) = timer.get_single_mut() {
        return (timer.0.tick(time.delta()).finished()).into();
    }
    false.into()
}

fn check_progress(
    mut cmd: Commands,
    progress: Option<Res<ProgressCounter>>,
    assets: Res<GameAssets>,
    timer: Query<&SplashTimer>,
    node: Query<Entity, With<UiNode>>,
    opts: Res<Persistent<GameOptions>>,
    mut bar: Query<&mut Style, With<ProgressBar>>,
    mut last_progress: Local<(u32, u32)>,
) {
    if let Some(progress) = progress.map(|counter| counter.progress()) {
        if progress.done == progress.total {
            return;
        }

        if progress.done > last_progress.0 {
            info!("Loading progress: {}/{}", progress.done, progress.total);
            *last_progress = (progress.done, progress.total);
            if let Ok(mut bar) = bar.get_single_mut() {
                bar.width = Val::Percent(progress.done as f32 / progress.total as f32 * 100.);
            }
        }
    }

    if let Ok(timer) = timer.get_single() {
        if timer.0.just_finished() {
            if let Ok(node) = node.get_single() {
                if let Some(mut entity) = cmd.get_entity(node) {
                    entity.with_children(|parent| {
                        // Loading text
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                "Loading",
                                TextStyle {
                                    font: assets.font.clone(),
                                    font_size: 48.,
                                    color: opts.color.mid,
                                },
                            ),
                            ..default()
                        });

                        // Progress bar
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(70.),
                                    height: Val::Px(32.),
                                    ..default()
                                },
                                background_color: opts.color.dark.into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Percent(
                                                last_progress.0 as f32 / last_progress.1 as f32
                                                    * 100.,
                                            ),
                                            height: Val::Px(32.),
                                            ..default()
                                        },
                                        background_color: opts.color.light.into(),
                                        ..default()
                                    },
                                    ProgressBar,
                                ));
                            });
                    });
                }
            }
        }
    }
}

fn clear_loading(mut cmd: Commands, node: Query<Entity, With<UiNode>>) {
    if let Ok(node) = node.get_single() {
        if let Some(mut entity) = cmd.get_entity(node) {
            entity.despawn_descendants();
        }
    }
}
