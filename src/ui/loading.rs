//! Loading screen module
//! Handles the loading screen when starting up the game

use bevy::prelude::*;
use sickle_ui::prelude::*;

use super::widgets::UiImageWidget;
use crate::{
    assets::{CoreAssets, LoadingData},
    ui::{widgets::UiTextWidget, UiRootContainer},
    GameState,
};

const UI_GAP: Val = Val::Px(16.);

/// Loading screen
/// Adds a loading screen at the start of the game while it is loading assets
pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<LoadingScreenState>()
            .enable_state_scoped_entities::<LoadingScreenState>()
            .add_systems(
                OnEnter(LoadingScreenState::Bevy),
                init_bevy,
            )
            .add_systems(
                Update,
                update_bevy.run_if(in_state(LoadingScreenState::Bevy)),
            )
            .add_systems(
                OnEnter(LoadingScreenState::Loading),
                init_loading,
            )
            .add_systems(
                Update,
                update_loading.run_if(in_state(LoadingScreenState::Loading)),
            );
    }
}

/// Loading screen state
/// Used to go through the different loading logos and finally the loading bar
#[derive(SubStates, Debug, Default, Clone, Eq, PartialEq, Hash)]
#[source(GameState = GameState::Loading)]
pub enum LoadingScreenState {
    /// Shows the Made with Bevy logo
    #[default]
    Bevy,
    /// The game still needs to load some assets after finishing the splash
    /// screens
    Loading,
}

// ··········
// Components
// ··········

#[derive(Component)]
struct BevySplashScreen;

#[derive(Component)]
struct LoadingText;

#[derive(Component)]
struct SplashScreenTimer(Timer);

// ·······
// Systems
// ·······

fn init_bevy(
    mut cmd: Commands,
    root: Query<Entity, With<UiRootContainer>>,
    assets: Res<CoreAssets>,
) {
    let Ok(root) = root.get_single() else { return };

    cmd.ui_builder(root)
        .column(|column| {
            column
                .style()
                .width(Val::Percent(100.))
                .align_items(AlignItems::Center)
                .justify_content(JustifyContent::Center)
                .row_gap(UI_GAP);

            column
                .image(assets.bevy_icon.clone())
                .insert(BevySplashScreen)
                .style()
                .width(Val::Percent(35.));

            column
                .title(
                    "Made with Bevy".into(),
                    assets.font.clone(),
                )
                .insert(BevySplashScreen);
        })
        .insert(StateScoped(LoadingScreenState::Bevy));

    cmd.spawn((
        SplashScreenTimer(Timer::from_seconds(2., TimerMode::Once)),
        StateScoped(LoadingScreenState::Bevy),
    ));
}

fn update_bevy(
    mut text: Query<&mut Text, With<BevySplashScreen>>,
    mut image: Query<&mut UiImage, With<BevySplashScreen>>,
    mut timer: Query<&mut SplashScreenTimer>,
    mut next_loading_state: ResMut<NextState<LoadingScreenState>>,
    time: Res<Time>,
) {
    let Ok(mut timer) = timer.get_single_mut() else { return };

    let timer = timer.0.tick(time.delta());
    if timer.just_finished() {
        next_loading_state.set(LoadingScreenState::Loading);
        return;
    }

    let alpha = timer.fraction_remaining();
    for mut text in text.iter_mut() {
        for sec in text.sections.iter_mut() {
            sec.style.color.set_alpha(alpha);
        }
    }
    for mut image in image.iter_mut() {
        image.color.set_alpha(alpha);
    }
}

fn init_loading(
    mut cmd: Commands,
    root: Query<Entity, With<UiRootContainer>>,
    assets: Res<CoreAssets>,
    mut loading_data: ResMut<LoadingData>,
    asset_server: Res<AssetServer>,
) {
    let (loaded, total) = loading_data.current(&asset_server);
    if loaded == total {
        return;
    };

    let Ok(root) = root.get_single() else { return };

    cmd.ui_builder(root)
        .column(|column| {
            column
                .style()
                .width(Val::Percent(100.))
                .align_items(AlignItems::Center)
                .justify_content(JustifyContent::Center)
                .row_gap(UI_GAP);

            column
                .title("Loading...".into(), assets.font.clone())
                .insert(LoadingText);
        })
        .insert(StateScoped(LoadingScreenState::Loading));
}

fn update_loading(
    mut text: Query<&mut Text, With<LoadingText>>,
    mut loading_data: ResMut<LoadingData>,
    asset_server: Res<AssetServer>,
) {
    let Ok(mut text) = text.get_single_mut() else { return };
    let Some(sec) = text.sections.first_mut() else { return };
    let (loaded, total) = loading_data.current(&asset_server);
    sec.value = format!("Loading {}/{}", loaded, total);
}
