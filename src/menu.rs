#![allow(clippy::type_complexity)]

use crate::{
    config::{GameOptions, Keybinds, Persistent},
    input::{Bind, InputState},
    load::GameAssets,
    GameState, COLOR_DARK, COLOR_DARKER, COLOR_LIGHT, COLOR_MID,
};
use bevy::prelude::*;
use bevy::reflect::Struct;

// TODO: Extract styles into external functions (maybe create ui package)
// TODO: Change the create functions to be more modular
// TODO: Single UI camera (for debug fps as well
// TODO: Tweening and animation

// ······
// Plugin
// ······

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), init_menu)
            .add_systems(
                Update,
                (
                    handle_buttons.run_if(in_state(GameState::Menu)),
                    return_to_menu,
                ),
            )
            .add_systems(OnExit(GameState::Menu), exit_menu)
            .add_systems(
                OnEnter(MenuState::Main),
                clean_menu.run_if(in_state(GameState::Menu)),
            )
            .add_systems(
                OnEnter(MenuState::Options),
                clean_menu.run_if(in_state(GameState::Menu)),
            )
            .add_systems(
                OnEnter(MenuState::Keybinds),
                clean_menu.run_if(in_state(GameState::Menu)),
            );
    }
}

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum MenuState {
    #[default]
    Main,
    Options,
    Keybinds,
}

// ··········
// Components
// ··········

#[derive(Component)]
struct MenuCam;

#[derive(Component)]
struct MenuNode;

#[derive(Component)]
struct MenuText;

#[derive(Component)]
enum MenuButton {
    Play,
    GoMain,
    GoOptions,
    GoKeybinds,
    OptionsTest,
    RemapKeybind(String, Vec<Bind>),
}

// ·······
// Systems
// ·······

fn init_menu(mut cmd: Commands, assets: Res<GameAssets>) {
    cmd.spawn((Camera2dBundle::default(), MenuCam));

    let node = cmd
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(18.),
                    ..default()
                },
                background_color: COLOR_DARKER.into(),
                ..default()
            },
            MenuNode,
        ))
        .id();

    layout_main(cmd, node, assets);
}

fn handle_buttons(
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut text: Query<&mut Text>,
    mut buttons: Query<
        (&Interaction, &MenuButton, &Children, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut opts: ResMut<Persistent<GameOptions>>,
) {
    for (inter, button, child, mut bg) in &mut buttons {
        let child = child.iter().next();
        if let Some(mut text) = child.and_then(|child| text.get_mut(*child).ok()) {
            match inter {
                Interaction::Pressed => {
                    bg.0 = COLOR_DARK;
                    text.sections[0].style.color = COLOR_LIGHT;

                    match button {
                        MenuButton::Play => {
                            game_state.set(GameState::Play);
                        }
                        MenuButton::GoMain => {
                            menu_state.set(MenuState::Main);
                        }
                        MenuButton::GoOptions => {
                            menu_state.set(MenuState::Options);
                        }
                        MenuButton::GoKeybinds => {
                            menu_state.set(MenuState::Keybinds);
                        }
                        MenuButton::OptionsTest => {
                            opts.update(|opts| {
                                opts.test = !opts.test;
                            })
                            .expect("Failed to update game options");

                            text.sections[0].value = if opts.test {
                                "Test: On".to_string()
                            } else {
                                "Test: Off".to_string()
                            };
                        }
                        MenuButton::RemapKeybind(field, _) => {
                            // TODO: Remap keymaps
                            info!("remap {}", field);
                        }
                    }
                }
                Interaction::Hovered => {
                    bg.0 = COLOR_MID;
                    text.sections[0].style.color = COLOR_DARKER;
                }
                Interaction::None => {
                    bg.0 = COLOR_LIGHT;
                    text.sections[0].style.color = COLOR_DARK;
                }
            }
        }
    }
}

fn clean_menu(
    mut cmd: Commands,
    state: Res<State<MenuState>>,
    node: Query<Entity, With<MenuNode>>,
    assets: Res<GameAssets>,
    opts: Res<Persistent<GameOptions>>,
    keybinds: Res<Persistent<Keybinds>>,
) {
    let node = node.single();

    cmd.entity(node).despawn_descendants();

    match state.get() {
        MenuState::Main => layout_main(cmd, node, assets),
        MenuState::Options => layout_options(cmd, node, assets, opts),
        MenuState::Keybinds => layout_keybinds(cmd, node, assets, keybinds),
    }
}

fn exit_menu(
    mut cmd: Commands,
    query: Query<Entity, Or<(With<MenuNode>, With<MenuCam>)>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for entity in query.iter() {
        cmd.entity(entity).despawn_recursive();
    }
    menu_state.set(MenuState::Main);
}

fn return_to_menu(
    mut game_state: ResMut<NextState<GameState>>,
    current_menu_state: Res<State<MenuState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    gamepad: Res<Input<GamepadButton>>,
    keybinds: Res<Persistent<Keybinds>>,
) {
    let input = InputState::new(&keyboard, &mouse, &gamepad);

    if input.just_pressed(&keybinds.pause).unwrap_or(false) {
        if *current_menu_state.get() != MenuState::Main {
            next_menu_state.set(MenuState::Main);
        }
        game_state.set(GameState::Menu);
    }
}

// ·····
// Extra
// ·····

fn layout_main(mut cmd: Commands, node: Entity, assets: Res<GameAssets>) {
    cmd.entity(node).with_children(|parent| {
        create_title(parent, assets.font.clone(), "Hello Bevy");

        create_button(parent, assets.font.clone(), "Play", MenuButton::Play);
        create_button(
            parent,
            assets.font.clone(),
            "Options",
            MenuButton::GoOptions,
        );
    });
}

fn layout_options(
    mut cmd: Commands,
    node: Entity,
    assets: Res<GameAssets>,
    opts: Res<Persistent<GameOptions>>,
) {
    cmd.entity(node).with_children(|parent| {
        create_title(parent, assets.font.clone(), "Options");

        create_button(
            parent,
            assets.font.clone(),
            if opts.test { "Test: On" } else { "Test: Off" },
            MenuButton::OptionsTest,
        );

        create_button(
            parent,
            assets.font.clone(),
            "Keybinds",
            MenuButton::GoKeybinds,
        );

        create_button(parent, assets.font.clone(), "Back", MenuButton::GoMain);
    });
}

fn layout_keybinds(
    mut cmd: Commands,
    node: Entity,
    assets: Res<GameAssets>,
    keybinds: Res<Persistent<Keybinds>>,
) {
    cmd.entity(node).with_children(|parent| {
        create_title(parent, assets.font.clone(), "Keybinds");

        // TODO: Scrollable section

        for (i, value) in keybinds.iter_fields().enumerate() {
            let field_name = keybinds.name_at(i).unwrap();
            if let Some(value) = value.downcast_ref::<Vec<Bind>>() {
                create_keybind_remap(parent, assets.font.clone(), field_name, value);
            }
        }

        create_button(parent, assets.font.clone(), "Back", MenuButton::GoOptions);
    });
}

fn create_title(parent: &mut ChildBuilder, font: Handle<Font>, text: &str) {
    parent.spawn(TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: 48.,
            color: COLOR_MID,
        },
    ));
}

fn create_button(parent: &mut ChildBuilder, font: Handle<Font>, text: &str, button: MenuButton) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(196.),
                    height: Val::Px(48.),
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
                    font_size: 24.,
                    color: COLOR_DARK,
                },
            ));
        });
}

fn create_keybind_remap(parent: &mut ChildBuilder, font: Handle<Font>, text: &str, bind: &[Bind]) {
    parent
        .spawn(NodeBundle {
            style: Style {
                min_width: Val::Px(196.),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(12.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            let name = text
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if i == 0 {
                        c.to_uppercase().next().unwrap()
                    } else if c == '_' {
                        ' '
                    } else {
                        c
                    }
                })
                .collect::<String>();

            parent.spawn(
                TextBundle::from_section(
                    name,
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.,
                        color: COLOR_LIGHT,
                    },
                )
                .with_style(Style {
                    flex_grow: 1.,
                    ..default()
                }),
            );

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(56.),
                            height: Val::Px(40.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: COLOR_LIGHT.into(),
                        ..default()
                    },
                    MenuButton::RemapKeybind(text.to_string(), bind.to_vec()),
                ))
                .with_children(|parent| {
                    let name = bind
                        .iter()
                        .map(|bind| bind.name())
                        .collect::<Vec<String>>()
                        .join(", ");
                    let font_size = if name.len() > 1 { 16. } else { 24. };
                    parent.spawn(TextBundle::from_section(
                        name,
                        TextStyle {
                            font,
                            font_size,
                            color: COLOR_DARK,
                        },
                    ));
                });
        });
}
