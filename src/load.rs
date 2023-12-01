#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use crate::{config::GameOptions, GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_persistent::Persistent;
use iyes_progress::prelude::*;

#[cfg(debug_assertions)]
pub const SPLASH_TIME: f32 = 0.1;
#[cfg(not(debug_assertions))]
pub const SPLASH_TIME: f32 = 2.;

// ······
// Plugin
// ······

pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(LoadingState::new(GameState::Loading))
            .init_collection::<GameAssets>()
            .add_collection_to_loading_state::<_, SampleAssets>(GameState::Loading)
            .add_plugins(
                ProgressPlugin::new(GameState::Loading)
                    .continue_to(GameState::Menu)
                    .track_assets(),
            )
            .add_systems(OnEnter(GameState::Loading), init_splash)
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

// Sample assets
#[derive(AssetCollection, Resource)]
pub struct SampleAssets {
    #[asset(path = "sounds/boing.ogg")]
    pub boing: Handle<AudioSource>,

    #[asset(path = "music/soundscape.ogg")]
    pub ambient_music: Handle<AudioSource>,
}

// ··········
// Components
// ··········

#[derive(Component)]
struct SplashCam;

#[derive(Component)]
struct SplashNode;

#[derive(Component)]
struct SplashTimer(Timer);

#[derive(Component)]
struct ProgressBar;

// ·······
// Systems
// ·······

fn init_splash(mut cmd: Commands, assets: Res<GameAssets>) {
    cmd.spawn((Camera2dBundle::default(), SplashCam));

    // Main ui node for the loading screen
    cmd.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(12.),
                ..default()
            },
            ..default()
        },
        SplashNode,
    ))
    .with_children(|parent| {
        // Bevy pixel logo
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
    node: Query<Entity, With<SplashNode>>,
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
                cmd.entity(node).with_children(|parent| {
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
                                            last_progress.0 as f32 / last_progress.1 as f32 * 100.,
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

// Finish the loading and clear all resources
fn clear_loading(
    mut cmd: Commands,
    splash_entities: Query<Entity, Or<(With<SplashCam>, With<SplashNode>, With<SplashTimer>)>>,
) {
    for entity in splash_entities.iter() {
        cmd.entity(entity).despawn_recursive();
    }
}
