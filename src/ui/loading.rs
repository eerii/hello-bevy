use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;

use crate::{
    ui::*,
    CoreAssets,
    GameState,
};

#[cfg(debug_assertions)]
const SPLASH_TIME: f32 = 0.1;
#[cfg(not(debug_assertions))]
const SPLASH_TIME: f32 = 2.;

// ······
// Plugin
// ······

pub struct LoadingUIPlugin;

impl Plugin for LoadingUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            init_splash.run_if(in_state(GameState::Loading)),
        )
        .add_systems(OnExit(GameState::Loading), clean_ui)
        .add_systems(
            Update,
            (
                check_progress,
                check_splash_finished.track_progress(),
            )
                .run_if(
                    in_state(GameState::Loading).and_then(resource_exists_and_changed::<
                        ProgressCounter,
                    >()),
                )
                .after(LoadingStateSet(GameState::Loading)),
        );
    }
}

// ··········
// Components
// ··········

#[derive(Component)]
struct ProgressBar;

#[derive(Component)]
struct SplashTimer(Timer);

impl Default for SplashTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            SPLASH_TIME,
            TimerMode::Once,
        ))
    }
}

// ·······
// Systems
// ·······

fn init_splash(
    mut cmd: Commands,
    assets: Res<CoreAssets>,
    node: Query<Entity, With<UiNode>>,
    mut has_init: Local<bool>,
) {
    if *has_init {
        return;
    }

    let Ok(node) = node.get_single() else { return };
    let Some(mut node) = cmd.get_entity(node) else { return };

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

    *has_init = true;
    cmd.spawn(SplashTimer::default());
}

fn check_progress(
    progress: Res<ProgressCounter>,
    mut progress_bar: Query<&mut Style, With<ProgressBar>>,
    mut last_progress: Local<u32>,
) {
    let progress = progress.progress();
    if progress.done == progress.total {
        return;
    }

    if progress.done > *last_progress {
        info!(
            "Loading progress: {}/{}",
            progress.done, progress.total
        );
        *last_progress = progress.done;

        let Ok(mut progress_bar) = progress_bar.get_single_mut() else { return };
        progress_bar.width = Val::Percent(progress.done as f32 / progress.total as f32 * 100.);
    }
}

fn check_splash_finished(
    mut cmd: Commands,
    time: Res<Time>,
    style: Res<UIStyle>,
    opts: Res<Persistent<GameOptions>>,
    progress: Res<ProgressCounter>,
    node: Query<Entity, With<UiNode>>,
    mut timer: Query<&mut SplashTimer>,
) -> Progress {
    let Ok(mut timer) = timer.get_single_mut() else { return false.into() };
    let timer = timer.0.tick(time.delta());

    let percent = progress.progress().done as f32 / progress.progress().total as f32;

    // Create loading screen
    if timer.just_finished() {
        let Ok(node) = node.get_single() else { return false.into() };
        let Some(mut node) = cmd.get_entity(node) else { return false.into() };
        node.with_children(|parent| {
            UIText::new(&style, "Loading...").with_title().add(parent);
            progress_bar(parent, &opts, percent);
        });
    }

    timer.finished().into()
}

// ·····
// Extra
// ·····

fn progress_bar(parent: &mut ChildBuilder, opts: &GameOptions, progress: f32) {
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
                        width: Val::Percent(progress * 100.),
                        height: Val::Px(32.),
                        ..default()
                    },
                    background_color: opts.color.light.into(),
                    ..default()
                },
                ProgressBar,
            ));
        });
}
