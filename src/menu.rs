#![allow(clippy::type_complexity)]

use crate::{
    config::{GameOptions, Persistent},
    load::SplashAssets,
    GameState, COLOR_DARK, COLOR_DARKER, COLOR_LIGHT, COLOR_MID,
};
use bevy::prelude::*;

// ······
// Plugin
// ······

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), init_menu)
            .add_systems(Update, handle_buttons.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), clean_menu);
    }
}

// ··········
// Components
// ··········

#[derive(Component)]
struct MenuCam;

#[derive(Component)]
struct MenuNode;

#[derive(Component)]
enum Button {
    Play,
    Options,
}

// ·······
// Systems
// ·······

// Create the menu
fn init_menu(mut cmd: Commands, assets: Res<SplashAssets>) {
    cmd.spawn((Camera2dBundle::default(), MenuCam));

    // Main UI node for the menu
    cmd.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(24.),
                ..default()
            },
            background_color: COLOR_DARKER.into(),
            ..default()
        },
        MenuNode,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Hello Bevy",
            TextStyle {
                font: assets.font.clone(),
                font_size: 64.,
                color: COLOR_MID,
            },
        ));

        create_button(parent, assets.font.clone(), "Play", Button::Play);
        create_button(parent, assets.font.clone(), "Options", Button::Options);
    });
}

fn create_button(parent: &mut ChildBuilder, font: Handle<Font>, text: &str, button: Button) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(256.),
                    height: Val::Px(64.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: COLOR_LIGHT.into(),
                ..default()
            },
            button,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size: 32.,
                    color: COLOR_DARK,
                },
            ));
        });
}

// Check for button presses
fn handle_buttons(
    mut state: ResMut<NextState<GameState>>,
    mut text: Query<&mut Text>,
    mut buttons: Query<
        (&Interaction, &Button, &Children, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut opts: ResMut<Persistent<GameOptions>>,
) {
    for (inter, button, child, mut bg) in &mut buttons {
        let child = child.iter().next();
        let text = child.and_then(|child| text.get_mut(*child).ok());

        match inter {
            Interaction::Pressed => {
                bg.0 = COLOR_DARK;
                if let Some(mut text) = text {
                    text.sections[0].style.color = COLOR_LIGHT;
                }

                match button {
                    Button::Play => {
                        state.set(GameState::Play);
                    }
                    Button::Options => {
                        opts.update(|opts| {
                            opts.test = !opts.test;
                        })
                        .expect("failed to update game options");
                    }
                }
            }
            Interaction::Hovered => {
                bg.0 = COLOR_MID;
                if let Some(mut text) = text {
                    text.sections[0].style.color = COLOR_DARKER;
                }
            }
            Interaction::None => {
                bg.0 = COLOR_LIGHT;
                if let Some(mut text) = text {
                    text.sections[0].style.color = COLOR_DARK;
                }
            }
        }
    }
}

// Clean the menu
fn clean_menu(mut cmd: Commands, query: Query<Entity, Or<(With<MenuNode>, With<MenuCam>)>>) {
    for entity in query.iter() {
        cmd.entity(entity).despawn_recursive();
    }
}
