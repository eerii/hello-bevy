use bevy::{math::bounding::*, prelude::*, window::PrimaryWindow};
use bevy_alt_ui_navigation_lite::{
    events::Direction as NavDirection, prelude::*, NavigationPlugin as AltNavigationPlugin,
};
use leafwing_input_manager::prelude::*;

use crate::{ui::menu::MenuState, GameState};

pub const BUTTON_COLOR: Color = Color::srgb(0.3, 0.5, 0.9);

// ······
// Plugin
// ······

// Navigation
// Uses bevy-alt-ui-navigation-lite to enable keyboard/gamepad navigation of ui elements
pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AltNavigationPlugin::new(),
            InputManagerPlugin::<MenuAction>::default(),
        ))
        .add_systems(Startup, init)
        .add_systems(
            PreUpdate,
            on_mouse_move.run_if(state_changed::<MenuState>),
        )
        .add_systems(
            Update,
            ((handle_input, update_focus).run_if(in_state(GameState::Menu)),),
        );
    }
}

// ··········
// Components
// ··········

// These are all the possible actions that have an input mapping
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum MenuAction {
    Continue,
    Back,
    Move,
    Mouse,
}

// ·······
// Systems
// ·······

// Create a new input manager for the UI
fn init(mut cmd: Commands) {
    let mut input_map = InputMap::default();
    input_map
        .insert(MenuAction::Continue, KeyCode::Space)
        .insert(MenuAction::Continue, KeyCode::Enter)
        .insert(MenuAction::Continue, MouseButton::Left)
        .insert(
            MenuAction::Continue,
            GamepadButtonType::South,
        )
        .insert(MenuAction::Back, KeyCode::Escape)
        .insert(MenuAction::Back, MouseButton::Right)
        .insert(
            MenuAction::Back,
            GamepadButtonType::East,
        )
        .insert(
            MenuAction::Move,
            KeyboardVirtualDPad::WASD,
        )
        .insert(
            MenuAction::Move,
            KeyboardVirtualDPad::ARROW_KEYS,
        )
        .insert(MenuAction::Move, GamepadStick::LEFT)
        .insert(MenuAction::Mouse, MouseMove::default());

    cmd.spawn(InputManagerBundle::with_map(input_map));
}

// Update the color of buttons when their state changes
fn update_focus(mut focusables: Query<(&Focusable, &mut BackgroundColor), Changed<Focusable>>) {
    for (focus, mut color) in focusables.iter_mut() {
        *color = match focus.state() {
            FocusState::Focused => BUTTON_COLOR.lighter(0.1),
            FocusState::Blocked => BUTTON_COLOR.darker(0.1),
            _ => BUTTON_COLOR,
        }
        .into();
    }
}

fn handle_input(
    input: Query<&ActionState<MenuAction>>,
    window: Query<&Window, With<PrimaryWindow>>,
    focused: Query<Entity, With<Focused>>,
    focusables: Query<(Entity, &Node, &GlobalTransform), With<Focusable>>,
    mut nav_request_writer: EventWriter<NavRequest>,
) {
    let Ok(input) = input.get_single() else {
        return;
    };

    // Either go back a level or go back to the game
    if input.just_pressed(&MenuAction::Back) {
        nav_request_writer.send(NavRequest::Cancel);
    }

    // Do not compute navigation if there is no focus to change
    if focused.is_empty() {
        return;
    }

    // Accept action
    if input.just_pressed(&MenuAction::Continue) {
        nav_request_writer.send(NavRequest::Action);
    }

    // Move up and down in the menu
    if input.just_pressed(&MenuAction::Move) {
        let axis = input.axis_pair(&MenuAction::Move);
        let dir = axis.unwrap_or_default().y();

        if dir.abs() > 0.5 {
            nav_request_writer.send(NavRequest::Move(if dir > 0. {
                NavDirection::North
            } else {
                NavDirection::South
            }));
        }
    };

    // If using mouse
    if input.just_pressed(&MenuAction::Mouse) {
        on_mouse_move(
            window,
            focused,
            focusables,
            nav_request_writer,
        );
    }
}

fn on_mouse_move(
    window: Query<&Window, With<PrimaryWindow>>,
    focused: Query<Entity, With<Focused>>,
    focusables: Query<(Entity, &Node, &GlobalTransform), With<Focusable>>,
    mut nav_request_writer: EventWriter<NavRequest>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };
    if let Some(mouse) = window
        .cursor_position()
        .map(|cursor| Aabb2d::new(cursor, Vec2::ZERO))
    {
        for (entity, node, trans) in focusables.iter() {
            let focused = focused.get_single().ok().unwrap_or(Entity::PLACEHOLDER);
            if entity == focused {
                continue;
            }

            let bounds = Aabb2d::new(
                trans.translation().truncate(),
                node.size() * 0.5,
            );
            if bounds.contains(&mouse) {
                nav_request_writer.send(NavRequest::FocusOn(entity));
            }
        }
    }
}
