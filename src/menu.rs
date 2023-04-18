// Main game menu with play and options
#![allow(clippy::type_complexity)]

use crate::{load::FontAssets, GameState};
use bevy::prelude::*;

const COLOR_LIGHT: Color = Color::rgb(1.0, 0.96, 0.97);
const COLOR_MID: Color = Color::rgb(0.65, 0.74, 0.76);
const COLOR_DARK: Color = Color::rgb(0.27, 0.42, 0.45);
const COLOR_DARKER: Color = Color::rgb(0.05, 0.1, 0.12);

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), init_menu)
            .add_systems(Update, handle_buttons.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), clean_menu);
    }
}

// Components
#[derive(Component)]
struct MenuCam;

#[derive(Component)]
struct MenuNode;

#[derive(Component)]
enum Button {
    Play,
    Options,
}

// Create the menu
fn init_menu(mut cmd: Commands, fonts: Res<FontAssets>) {
    cmd.spawn((Camera2dBundle::default(), MenuCam));

    // Main UI node for the menu
    cmd.spawn((
        NodeBundle {
            style: Style {
                size: Size::all(Val::Percent(100.)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                gap: Size {
                    width: Val::Px(0.),
                    height: Val::Px(24.),
                },
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
                font: fonts.gameboy.clone(),
                font_size: 64.,
                color: COLOR_MID,
            },
        ));

        create_button(parent, &fonts, "Play", Button::Play);
        create_button(parent, &fonts, "Options", Button::Options);
    });
}

fn create_button(parent: &mut ChildBuilder, fonts: &Res<FontAssets>, text: &str, button: Button) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(256.), Val::Px(64.)),
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
                    font: fonts.gameboy.clone(),
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
) {
    for (inter, button, child, mut bg) in &mut buttons {
        let child = child.iter().next();
        let text = child.and_then(|child| text.get_mut(*child).ok());

        match inter {
            Interaction::Clicked => {
                bg.0 = COLOR_DARK;
                if let Some(mut text) = text {
                    text.sections[0].style.color = COLOR_LIGHT;
                }

                match button {
                    Button::Play => {
                        state.set(GameState::Play);
                    }
                    Button::Options => {}
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
