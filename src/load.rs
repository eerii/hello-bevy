// Loading screen and asset plugin configuration
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_tweening::*;
use iyes_progress::prelude::*;
use std::time::Duration;

#[cfg(debug_assertions)]
pub const SPLASH_TIME: f32 = 0.;
#[cfg(not(debug_assertions))]
pub const SPLASH_TIME: f32 = 2.;

const COLOR_LIGHT: Color = Color::rgb(1.0, 0.96, 0.97);
const COLOR_MID: Color = Color::rgb(0.65, 0.74, 0.76);
const COLOR_DARK: Color = Color::rgb(0.27, 0.42, 0.45);

// Loading assets plugin
pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(LoadingState::new(GameState::Loading))
            .init_collection::<SplashAssets>()
            .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
            .add_plugin(ProgressPlugin::new(GameState::Loading).continue_to(GameState::Menu))
            .add_systems(OnEnter(GameState::Loading), init_splash)
            .add_systems(OnExit(GameState::Loading), clear_loading)
            .add_systems(
                Update,
                (
                    check_splash_finished.track_progress(),
                    check_progress.after(ProgressSystemSet::CheckProgress),
                )
                    .run_if(in_state(GameState::Loading)),
            );
    }
}

// Test assets
#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/gb.ttf")]
    pub gameboy: Handle<Font>,
}

// Splash screen setup
#[derive(Component)]
struct SplashCam;

#[derive(Component)]
struct SplashNode;

#[derive(Component)]
struct SplashTimer(Timer);

#[derive(AssetCollection, Resource)] // this is loaded inmediately after the app is fired, has no effect on state
struct SplashAssets {
    #[asset(path = "icons/pixelbevy.png")]
    pub bevy_icon: Handle<Image>,

    #[asset(path = "fonts/gb.ttf")]
    pub font: Handle<Font>,
}

fn init_splash(mut cmd: Commands, assets: Res<SplashAssets>) {
    cmd.spawn((Camera2dBundle::default(), SplashCam));

    // Main ui node for the loading screen
    cmd.spawn((
        NodeBundle {
            style: Style {
                size: Size::all(Val::Percent(100.)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                gap: Size {
                    width: Val::Px(0.),
                    height: Val::Px(12.),
                },
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        },
        SplashNode,
    ))
    .with_children(|parent| {
        // Bevy pixel logo
        parent.spawn((
            ImageBundle {
                image: UiImage {
                    texture: assets.bevy_icon.clone(),
                    ..default()
                },
                style: Style {
                    size: Size::width(Val::Px(240.0)),
                    ..default()
                },
                ..default()
            },
            Animator::new(
                Tween::new(
                    EaseFunction::CubicInOut,
                    Duration::from_secs(2),
                    lens::TransformScaleLens {
                        start: Vec3::splat(1.),
                        end: Vec3::splat(0.95),
                    },
                )
                .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
                .with_repeat_count(RepeatCount::Infinite),
            ),
        ));
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

// Check the loading progress
#[derive(Component)]
struct ProgressBar;

fn check_progress(
    mut cmd: Commands,
    progress: Option<Res<ProgressCounter>>,
    assets: Res<SplashAssets>,
    timer: Query<&SplashTimer>,
    node: Query<Entity, With<SplashNode>>,
    mut bar: Query<&mut Style, With<ProgressBar>>,
    mut last_progress: Local<(u32, u32)>,
) {
    if let Some(progress) = progress.map(|counter| counter.progress()) {
        if progress.done == progress.total {
            return;
        }

        if progress.done > last_progress.0 {
            debug!("Loading progress: {}/{}", progress.done, progress.total);
            *last_progress = (progress.done, progress.total);
            if let Ok(mut bar) = bar.get_single_mut() {
                bar.size.width = Val::Percent(progress.done as f32 / progress.total as f32 * 100.);
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
                                color: COLOR_MID,
                            },
                        ),
                        ..default()
                    });

                    // Progress bar
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(70.), Val::Px(32.)),
                                ..default()
                            },
                            background_color: COLOR_DARK.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Percent(
                                                last_progress.0 as f32 / last_progress.1 as f32
                                                    * 100.,
                                            ),
                                            Val::Px(32.),
                                        ),
                                        ..default()
                                    },
                                    background_color: COLOR_LIGHT.into(),
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
