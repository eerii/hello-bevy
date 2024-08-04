//! Ui navigation system that allows for mouse, keyboard and gamepad input.

use bevy::ecs::component::{ComponentHooks, StorageType};

use crate::prelude::*;

// TODO: Text to speech

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        DefaultPickingPlugins,
        EventListenerPlugin::<NavActionEvent>::default(),
    ))
    .add_systems(Update, (handle_next_prev, handle_press).chain());
}

/// `Navigable` children of entities with this components can be selected and
/// have its focus moved with input actions.
/// Nesting is not properly supported yet.
#[derive(Component)]
pub struct NavContainer;

/// An UI element that can be navigated to.
#[derive(Component)]
pub struct Navigable {
    /// Text associated with the navigation item that can be read aloud with
    /// screen readers.
    pub label: String,
}

/// A marker for the selected `Navigable` entity of a `NavContainer`.
/// It has custom component hooks to change properties of its entity when it is
/// added and removed.
#[derive(Clone)]
struct NavSelected;

impl Component for NavSelected {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _id| {
            let palette = world
                .get_resource::<GameOptions>()
                .map(|data| data.palette)
                .unwrap_or_default();
            if let Some(mut background) = world.get_mut::<BackgroundColor>(entity) {
                *background = palette.dark.into();
            };
            if let Some(mut border) = world.get_mut::<BorderColor>(entity) {
                *border = palette.primary.into();
            };
        });

        hooks.on_remove(|mut world, entity, _id| {
            let palette = world
                .get_resource::<GameOptions>()
                .map(|data| data.palette)
                .unwrap_or_default();
            if let Some(mut background) = world.get_mut::<BackgroundColor>(entity) {
                *background = palette.primary.into();
            };
            if let Some(mut border) = world.get_mut::<BorderColor>(entity) {
                *border = palette.light.into();
            };
        });
    }
}

/// Prevents navigation movement actions from happenning too quickly.
#[derive(Component)]
struct InputRepeatDelay;

/// Component bundle for added cursor support for selection changes.
#[derive(Bundle)]
pub struct NavBundle {
    pointer_move: On<Pointer<Move>>,
    pointer_out: On<Pointer<Out>>,
}

impl Default for NavBundle {
    fn default() -> Self {
        Self {
            pointer_move: On::<Pointer<Move>>::run(
                |event: Listener<Pointer<Move>>,
                 mut cmd: Commands,
                 selected: Query<Entity, With<NavSelected>>| {
                    for prev in &selected {
                        cmd.entity(prev).remove::<NavSelected>();
                    }
                    cmd.entity(event.target).insert(NavSelected);
                },
            ),
            pointer_out: On::<Pointer<Out>>::target_remove::<NavSelected>(),
        }
    }
}

/// Event that is triggered when an action is requested for the selected
/// navigation item.
#[derive(Clone, Event, EntityEvent)]
pub struct NavActionEvent {
    /// The entity that triggered the event.
    #[target]
    pub target: Entity,
}

/// Uses `Action::Move` to switch the focus of the `Selected` entity inside a
/// `NavContainer` to the next or previous entity. It has wrapping and it focus
/// on the first entity if no one is selected.
fn handle_next_prev(
    mut cmd: Commands,
    input: Query<&ActionState<Action>>,
    navigation: Query<&Children, With<NavContainer>>,
    navigables: Query<(), With<Navigable>>,
    selected: Query<Entity, With<NavSelected>>,
    repeat_delay: Query<Entity, With<InputRepeatDelay>>,
) {
    if !repeat_delay.is_empty() {
        return;
    };

    let input = single!(input);

    // If using WASD, S and D will call next and W and A will call prev
    let val = input.clamped_axis_pair(&Action::Move);
    let move_forward = if val.length() > 0.2 {
        val.x > 0. || val.y < 0.
    } else {
        return;
    };

    // Schedule a delay to avoid having one focus change every frame
    let entity = cmd.spawn(InputRepeatDelay).id();
    cmd.later(0.2, move |cmd| {
        cmd.entity(entity).despawn();
    });

    for children in &navigation {
        if children.len() == 0 {
            continue;
        };

        let curr = selected
            .iter()
            .find_map(|e| children.iter().position(|&p| p == e));

        let next = match curr {
            Some(prev) => {
                cmd.entity(children[prev]).remove::<NavSelected>();
                let mut next = prev;
                for i in 1..children.len() {
                    let value = if move_forward { prev + i } else { prev + children.len() - i }
                        % children.len();
                    if navigables.contains(children[value]) {
                        next = value;
                        break;
                    }
                }
                next
            },
            None => 0,
        };
        cmd.entity(children[next]).insert(NavSelected);
    }
}

/// When `Action::Act` is pressed, trigger the `NavActionEvent` for
/// `NavSelected`
fn handle_press(
    input: Query<&ActionState<Action>>,
    selected: Query<Entity, With<NavSelected>>,
    mut nav_action_writer: EventWriter<NavActionEvent>,
) {
    let input = single!(input);
    if input.just_pressed(&Action::Act) {
        let selected = single!(selected);
        nav_action_writer.send(NavActionEvent { target: selected });
    }
}
