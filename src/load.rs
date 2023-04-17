use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;

const SPLASH_TIME: f32 = 2.;

// Loading assets plugin
pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(LoadingState::new(GameState::Loading))
            .init_collection::<SplashAssets>()
            .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
            .add_plugin(ProgressPlugin::new(GameState::Loading).continue_to(GameState::Menu))
            .add_systems(OnEnter(GameState::Loading), splash_init)
            .add_systems(
                Update,
                check_splash_finished
                    .track_progress()
                    .run_if(in_state(GameState::Loading)),
            )
            .add_systems(
                Update,
                check_progress
                    .run_if(in_state(GameState::Loading))
                    .after(ProgressSystemSet::CheckProgress),
            )
            .add_systems(
                Update,
                fake_load
                    .track_progress()
                    .run_if(in_state(GameState::Loading)),
            )
            .add_systems(OnExit(GameState::Loading), load_clear);
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
struct SplashScreen;

#[derive(Component)]
struct SplashNode;

#[derive(Component)]
struct SplashScreenTimer(Timer);

#[derive(AssetCollection, Resource)] // this is loaded inmediately after the app is fired, has no effect on state
struct SplashAssets {
    #[asset(path = "icons/pixelbevy.png")]
    pub bevy_icon: Handle<Image>,

    #[asset(path = "fonts/gb.ttf")]
    pub font: Handle<Font>,
}

fn splash_init(mut cmd: Commands, assets: Res<SplashAssets>) {
    cmd.spawn((Camera2dBundle::default(), SplashScreen));

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
        SplashScreen,
        SplashNode,
    ))
    .with_children(|parent| {
        parent.spawn(ImageBundle {
            image: UiImage {
                texture: assets.bevy_icon.clone(),
                ..default()
            },
            style: Style {
                size: Size::width(Val::Px(240.0)),
                ..default()
            },
            ..default()
        });
    });

    cmd.spawn((
        SplashScreenTimer(Timer::from_seconds(SPLASH_TIME, TimerMode::Once)),
        SplashScreen,
    ));
}

fn check_splash_finished(time: Res<Time>, mut timer: Query<&mut SplashScreenTimer>) -> Progress {
    if let Ok(mut timer) = timer.get_single_mut() {
        return (timer.0.tick(time.delta()).just_finished()).into();
    }
    false.into()
}

// Check the loading progress
fn check_progress(
    mut cmd: Commands,
    progress: Option<Res<ProgressCounter>>,
    assets: Res<SplashAssets>,
    timer: Query<&SplashScreenTimer>,
    node: Query<Entity, With<SplashNode>>,
    mut last_progress: Local<u32>,
) {
    if let Some(progress) = progress.map(|counter| counter.progress()) {
        if progress.done > *last_progress {
            debug!("Loading progress: {}/{}", progress.done, progress.total);
            *last_progress = progress.done;
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
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    });

                    // Progress bar
                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(70.), Val::Px(32.)),
                            ..default()
                        },
                        background_color: Color::ALICE_BLUE.into(),
                        ..default()
                    });
                });
            }
        }
    }
}

fn fake_load(time: Res<Time>) -> Progress {
    (time.elapsed_seconds() > 4.).into()
}

// Finish the loading, clear all resources and transition to next state
fn load_clear(
    mut cmd: Commands,
    splash_entities: Query<Entity, With<SplashScreen>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for entity in splash_entities.iter() {
        cmd.entity(entity).despawn_recursive();
    }

    next_state.set(GameState::Menu);
}
