#![allow(clippy::type_complexity)]

use crate::{
    config::{GameOptions, Keybinds, Persistent},
    input::{Bind, InputState},
    ui::*,
    GameState,
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
    OptionsColor(String),
    RemapKeybind(String, Vec<Bind>),
}

// ·······
// Systems
// ·······

fn init_menu(mut cmd: Commands, opts: Res<Persistent<GameOptions>>, style: Res<UIStyle>) {
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
                background_color: opts.color.darker.into(),
                ..default()
            },
            MenuNode,
        ))
        .id();

    layout_main(cmd, node, &style);
}

fn handle_buttons(
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut text: Query<&mut Text>,
    mut buttons: Query<
        (&Interaction, &MenuButton, &Children, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    opts: Res<Persistent<GameOptions>>,
) {
    for (inter, button, child, mut bg) in &mut buttons {
        let child = child.iter().next();
        if let Some(mut text) = child.and_then(|child| text.get_mut(*child).ok()) {
            match inter {
                Interaction::Pressed => {
                    bg.0 = opts.color.dark;
                    text.sections[0].style.color = opts.color.light;

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
                        MenuButton::OptionsColor(field) => {
                            // TODO: Add color picker
                            info!("color {}", field);
                        }
                        MenuButton::RemapKeybind(field, _) => {
                            // TODO: Remap keymaps
                            info!("remap {}", field);
                        }
                    }
                }
                Interaction::Hovered => {
                    bg.0 = opts.color.mid;
                    text.sections[0].style.color = opts.color.darker;
                }
                Interaction::None => {
                    bg.0 = opts.color.light;
                    text.sections[0].style.color = opts.color.dark;
                }
            }
        }
    }
}

fn clean_menu(
    mut cmd: Commands,
    state: Res<State<MenuState>>,
    node: Query<Entity, With<MenuNode>>,
    style: Res<UIStyle>,
    opts: Res<Persistent<GameOptions>>,
    keybinds: Res<Persistent<Keybinds>>,
) {
    let node = node.single();

    cmd.entity(node).despawn_descendants();

    match state.get() {
        MenuState::Main => layout_main(cmd, node, &style),
        MenuState::Options => layout_options(cmd, node, &style, &opts),
        MenuState::Keybinds => layout_keybinds(cmd, node, &style, &keybinds),
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

fn layout_main(mut cmd: Commands, node: Entity, style: &UIStyle) {
    cmd.entity(node).with_children(|parent| {
        create_title(parent, style, "Hello Bevy");

        create_button(parent, style, "Play", MenuButton::Play);
        create_button(parent, style, "Options", MenuButton::GoOptions);
    });
}

fn layout_options(mut cmd: Commands, node: Entity, style: &UIStyle, opts: &GameOptions) {
    cmd.entity(node).with_children(|parent| {
        create_title(parent, style, "Options");

        create_button(parent, style, "Keybinds", MenuButton::GoKeybinds);

        for (i, value) in opts.color.iter_fields().enumerate() {
            let field_name = opts.color.name_at(i).unwrap();
            if let Some(value) = value.downcast_ref::<Color>() {
                let r = value.r();
                let g = value.g();
                let b = value.b();
                create_button(
                    parent,
                    style,
                    &format!(
                        "{}: {:.0},{:.0},{:.0}",
                        field_name,
                        r * 255.,
                        g * 255.,
                        b * 255.
                    ),
                    MenuButton::OptionsColor(field_name.to_string()),
                );
            }
        }

        create_button(parent, style, "Back", MenuButton::GoMain);
    });
}

fn layout_keybinds(mut cmd: Commands, node: Entity, style: &UIStyle, keybinds: &Keybinds) {
    cmd.entity(node).with_children(|parent| {
        create_title(parent, style, "Keybinds");

        // TODO: Scrollable section

        for (i, value) in keybinds.iter_fields().enumerate() {
            let field_name = keybinds.name_at(i).unwrap();
            if let Some(value) = value.downcast_ref::<Vec<Bind>>() {
                create_keybind_remap(
                    parent,
                    style,
                    field_name,
                    MenuButton::RemapKeybind(field_name.to_string(), value.clone()),
                    value,
                );
            }
        }

        create_button(parent, style, "Back", MenuButton::GoOptions);
    });
}
