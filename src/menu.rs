#![allow(clippy::type_complexity)]

use crate::{
    config::{GameOptions, Keybinds, Persistent},
    input::{Bind, InputState},
    ui::*,
    GameState,
};
use bevy::prelude::*;
use bevy::reflect::Struct;

// TODO: Single UI camera (for debug fps as well)
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
                OnEnter(MenuState::Settings),
                clean_menu.run_if(in_state(GameState::Menu)),
            )
            .add_systems(
                OnEnter(MenuState::Keybinds),
                clean_menu.run_if(in_state(GameState::Menu)),
            )
            .add_systems(
                OnEnter(MenuState::Visual),
                clean_menu.run_if(in_state(GameState::Menu)),
            );
    }
}

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum MenuState {
    #[default]
    Main,
    Settings,
    Keybinds,
    Visual,
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
    GoSettings,
    GoKeybinds,
    GoVisual,
    RemapKeybind(String, Vec<Bind>),
    ChangeFont(String),
    ChangeColor(String, String),
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
                    row_gap: Val::Px(12.),
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
                        MenuButton::GoSettings => {
                            menu_state.set(MenuState::Settings);
                        }
                        MenuButton::GoKeybinds => {
                            menu_state.set(MenuState::Keybinds);
                        }
                        MenuButton::GoVisual => {
                            menu_state.set(MenuState::Visual);
                        }
                        MenuButton::RemapKeybind(_, _) => {
                            // TODO: Remap keymaps
                        }
                        MenuButton::ChangeFont(_) => {
                            // TODO: Change font size
                        }
                        MenuButton::ChangeColor(_, _) => {
                            // TODO: Change color
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
        MenuState::Settings => layout_options(cmd, node, &style),
        MenuState::Keybinds => layout_keybinds(cmd, node, &style, &keybinds),
        MenuState::Visual => layout_visual(cmd, node, &style, &opts),
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
        match *current_menu_state.get() {
            MenuState::Settings => next_menu_state.set(MenuState::Main),
            MenuState::Keybinds => next_menu_state.set(MenuState::Settings),
            MenuState::Visual => next_menu_state.set(MenuState::Settings),
            _ => {}
        }
        game_state.set(GameState::Menu);
    }
}

// ·····
// Extra
// ·····

fn layout_main(mut cmd: Commands, node: Entity, style: &UIStyle) {
    cmd.entity(node).with_children(|parent| {
        UIText::new(style, "Hello Bevy").with_title().add(parent);

        UIButton::new(style, "Play", MenuButton::Play).add(parent);
        UIButton::new(style, "Settings", MenuButton::GoSettings).add(parent);
    });
}

fn layout_options(mut cmd: Commands, node: Entity, style: &UIStyle) {
    cmd.entity(node).with_children(|parent| {
        UIText::new(style, "Settings").with_title().add(parent);

        UIButton::new(style, "Keybinds", MenuButton::GoKeybinds).add(parent);
        UIButton::new(style, "Visual", MenuButton::GoVisual).add(parent);

        UIButton::new(style, "Back", MenuButton::GoMain).add(parent);
    });
}

fn layout_keybinds(mut cmd: Commands, node: Entity, style: &UIStyle, keybinds: &Keybinds) {
    cmd.entity(node).with_children(|parent| {
        UIText::new(style, "Options").with_title().add(parent);

        // TODO: Scrollable section (Requires #8104 to be merged in 0.13)

        for (i, value) in keybinds.iter_fields().enumerate() {
            let field_name = keybinds.name_at(i).unwrap();
            if let Some(value) = value.downcast_ref::<Vec<Bind>>() {
                UIOption::new(style, field_name).add(parent, |row| {
                    let keys = value
                        .iter()
                        .map(|bind| bind.to_string())
                        .collect::<Vec<_>>()
                        .join(" ");

                    UIButton::new(
                        style,
                        &keys,
                        MenuButton::RemapKeybind(field_name.to_string(), value.clone()),
                    )
                    .with_width(Val::Px(64.))
                    .add(row);
                });
            }
        }

        UIButton::new(style, "Back", MenuButton::GoSettings).add(parent);
    });
}

fn layout_visual(mut cmd: Commands, node: Entity, style: &UIStyle, opts: &GameOptions) {
    cmd.entity(node).with_children(|parent| {
        UIText::new(style, "Visual settings")
            .with_title()
            .add(parent);

        for (i, value) in opts.font_size.iter_fields().enumerate() {
            let field_name = opts.font_size.name_at(i).unwrap().to_string();
            if let Some(value) = value.downcast_ref::<f32>() {
                UIOption::new(style, &format!("font_{}", field_name)).add(parent, |row| {
                    UIButton::new(
                        style,
                        &format!("{}", value),
                        MenuButton::ChangeFont(field_name),
                    )
                    .with_width(Val::Px(40.))
                    .add(row);
                });
            }
        }

        for (i, value) in opts.color.iter_fields().enumerate() {
            let field_name = format!("color_{}", opts.color.name_at(i).unwrap());
            if let Some(value) = value.downcast_ref::<Color>() {
                UIOption::new(style, &field_name).add(parent, |row| {
                    UIButton::new(
                        style,
                        &format!("{:.0}", value.r() * 255.),
                        MenuButton::ChangeColor(field_name.clone(), "r".to_string()),
                    )
                    .with_width(Val::Px(40.))
                    .add(row);

                    UIButton::new(
                        style,
                        &format!("{:.0}", value.g() * 255.),
                        MenuButton::ChangeColor(field_name.clone(), "g".to_string()),
                    )
                    .with_width(Val::Px(40.))
                    .add(row);

                    UIButton::new(
                        style,
                        &format!("{:.0}", value.b() * 255.),
                        MenuButton::ChangeColor(field_name.clone(), "b".to_string()),
                    )
                    .with_width(Val::Px(40.))
                    .add(row);
                });
            }
        }

        UIButton::new(style, "Back", MenuButton::GoSettings).add(parent);
    });
}
