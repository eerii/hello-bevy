//! Navigation module

use bevy::{math::bounding::*, prelude::*, window::PrimaryWindow};
use bevy_alt_ui_navigation_lite::{
    events::Direction as NavDirection, prelude::*, NavigationPlugin as AltNavigationPlugin,
};
use leafwing_input_manager::prelude::*;

use crate::{ui::widgets::BUTTON_COLOR, GameState};

// ······
// Plugin
// ······

/// Navigation
/// Uses bevy-alt-ui-navigation-lite to enable keyboard/gamepad navigation of ui
/// elements We don't use the full implementation there, the input part is
/// rewritten to match with our input system using leafwing-input-manager
pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AltNavigationPlugin::new(),
            InputManagerPlugin::<UiAction>::default(),
        ))
        .add_systems(OnExit(GameState::Startup), init)
        .add_systems(
            Update,
            ((
                handle_input.before(NavRequestSystem),
                update_focus.after(NavRequestSystem),
            )
                .run_if(in_state(GameState::Menu)),),
        );

        #[cfg(feature = "menu")]
        app.add_systems(
            PreUpdate,
            on_mouse_move.run_if(state_changed::<super::menu::MenuState>),
        );
    }
}

// ··········
// Components
// ··········

/// These are all the possible actions that have an input mapping
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub(super) enum UiAction {
    /// Press the selected interface element
    Continue,
    /// Go back to the previous menu
    Back,
    /// Axis movement to switch to the next or previous element
    Move,
    /// This just detects when the mouse moves to avoid excesive queries
    /// The actual mouse position is calculated on `on_mouse_move`
    Mouse,
}

/// If this marker is in an interactable button, use fill instead of border to
/// show that it is being hovered
#[derive(Component)]
pub(super) struct FocusableHoverFill;

/// A focusable that should hightlight its children, not itself
#[derive(Component)]
pub(super) struct HightlightChild;

// ·······
// Systems
// ·······

/// Create a new input manager for the UI
/// Here we assign each action to multiple input devices
/// This defaults should be enough to navigate the menu with keyboard, mouse or
/// gamepad
fn init(mut cmd: Commands) {
    let mut input_map = InputMap::default();
    input_map
        .insert(UiAction::Continue, KeyCode::Space)
        .insert(UiAction::Continue, KeyCode::Enter)
        .insert(UiAction::Continue, MouseButton::Left)
        .insert(
            UiAction::Continue,
            GamepadButtonType::South,
        )
        .insert(UiAction::Back, KeyCode::Escape)
        .insert(UiAction::Back, MouseButton::Right)
        .insert(UiAction::Back, GamepadButtonType::East)
        .insert(
            UiAction::Move,
            KeyboardVirtualDPad::WASD,
        )
        .insert(
            UiAction::Move,
            KeyboardVirtualDPad::ARROW_KEYS,
        )
        .insert(UiAction::Move, GamepadStick::LEFT)
        .insert(UiAction::Mouse, MouseMove::default());

    cmd.spawn(InputManagerBundle::with_map(input_map));
}

/// Update the color of buttons when their state changes
fn update_focus(
    mut focusables: Query<
        (
            Entity,
            &Focusable,
            Option<&Children>,
            Option<&HightlightChild>,
        ),
        Changed<Focusable>,
    >,
    mut border: Query<&mut BorderColor>,
    mut background: Query<&mut BackgroundColor>,
    fill: Query<&FocusableHoverFill>,
) {
    for (entity, focus, children, highlight_child) in focusables.iter_mut() {
        let entity = match highlight_child {
            Some(_) => {
                if let Some(children) = children {
                    *children.last().unwrap_or(&entity)
                } else {
                    entity
                }
            },
            None => entity,
        };

        if fill.contains(entity) {
            let Ok(mut color) = background.get_mut(entity) else { continue };
            *color = match focus.state() {
                FocusState::Focused => BUTTON_COLOR,
                _ => Srgba::NONE.into(),
            }
            .into();
        } else {
            let Ok(mut color) = border.get_mut(entity) else { continue };
            *color = match focus.state() {
                FocusState::Focused => BUTTON_COLOR.lighter(0.3),
                FocusState::Blocked => BUTTON_COLOR.darker(0.3),
                _ => BUTTON_COLOR,
            }
            .into();
        }
    }
}

/// This is our custom input handler
/// It uses leafwing to aggregate all the input sources into actions
/// Then it sends the propper NavRequest events
fn handle_input(
    input: Query<&ActionState<UiAction>>,
    window: Query<&Window, With<PrimaryWindow>>,
    focused: Query<Entity, With<Focused>>,
    focusables: Query<(Entity, &Node, &GlobalTransform), With<Focusable>>,
    mut nav_request_writer: EventWriter<NavRequest>,
) {
    let Ok(input) = input.get_single() else { return };

    // Either go back a level or go back to the game
    if input.just_pressed(&UiAction::Back) {
        nav_request_writer.send(NavRequest::Cancel);
    }

    // Do not compute navigation if there is no focus to change
    if focused.is_empty() {
        return;
    }

    // Accept action
    if input.just_pressed(&UiAction::Continue) {
        nav_request_writer.send(NavRequest::Action);
    }

    // Move through in the menu
    if input.just_pressed(&UiAction::Move) {
        let axis = input.clamped_axis_pair(&UiAction::Move).unwrap_or_default();
        if axis.y().abs() > axis.x().abs() {
            nav_request_writer.send(NavRequest::Move(if axis.y() > 0. {
                NavDirection::North
            } else {
                NavDirection::South
            }));
        } else {
            nav_request_writer.send(NavRequest::Move(if axis.x() > 0. {
                NavDirection::East
            } else {
                NavDirection::West
            }));
        };
    }

    // If using mouse, also call the mouse system
    // This only runs when the mouse has just moved
    if input.just_pressed(&UiAction::Mouse) {
        on_mouse_move(
            window,
            focused,
            focusables,
            nav_request_writer,
        );
    }
}

/// The mouse movement is a bit more tricky
/// Leafwing handles mouse deltas, but we want the mouse screen position (not
/// the world pos in this case) We handle it separately here, using the primary
/// window This is a system that runs when the state is change (to avoid
/// clicking on the unupdated menu) but can also be called as a function when
/// the mouse moves
fn on_mouse_move(
    window: Query<&Window, With<PrimaryWindow>>,
    focused: Query<Entity, With<Focused>>,
    focusables: Query<(Entity, &Node, &GlobalTransform), With<Focusable>>,
    mut nav_request_writer: EventWriter<NavRequest>,
) {
    let Ok(window) = window.get_single() else { return };
    if let Some(mouse) = window
        .cursor_position()
        .map(|cursor| Aabb2d::new(cursor, Vec2::ZERO))
    {
        for (entity, node, trans) in focusables.iter() {
            let focused = focused.get_single().ok().unwrap_or(Entity::PLACEHOLDER);
            if entity == focused {
                continue;
            }

            // We use the same Aabb collision logic than with a regular game!
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
