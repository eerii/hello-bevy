use bevy::{
    color::palettes::css,
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};
use bevy_mod_picking::prelude::*;

use crate::prelude::*;

// TODO: Create macro to derive navigable
//       Automatically register_component_as and create an interaction
//       handler for then the navigable is pressed
//       Also add the option to change state and send events,
//       these need to pass parameters into the function, so the
//       systems they generate need to have them

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(DefaultPickingPlugins)
        .add_systems(Update, (handle_next_prev, handle_press).chain());
}

#[bevy_trait_query::queryable]
pub trait Navigable {
    fn label(&self) -> String; // For tts
    fn action(&self);
}

#[derive(Component, Default)]
pub(super) struct NavContainer;

#[derive(Clone)]
struct NavSelected;

impl Component for NavSelected {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _id| {
            let Some(mut background) = world.get_mut::<BackgroundColor>(entity) else { return };
            *background = css::MEDIUM_SEA_GREEN.into();
        });

        hooks.on_remove(|mut world, entity, _id| {
            let Some(mut background) = world.get_mut::<BackgroundColor>(entity) else { return };
            *background = css::ROYAL_BLUE.into();
        });
    }
}

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

#[derive(Component)]
struct InputRepeatDelay;

fn handle_next_prev(
    mut cmd: Commands,
    input: Query<&ActionState<Action>>,
    navigation: Query<&Children, With<NavContainer>>,
    selected: Query<Entity, With<NavSelected>>,
    repeat_delay: Query<Entity, With<InputRepeatDelay>>,
) {
    if !repeat_delay.is_empty() {
        return;
    };

    let Ok(input) = input.get_single() else { return };

    let val = input.clamped_axis_pair(&Action::Move);
    let move_forward = if val.length() > 0.2 {
        val.x > 0. || val.y < 0.
    } else {
        return;
    };

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
                let value = if move_forward { prev + 1 } else { prev + children.len() - 1 };
                value % children.len()
            },
            None => 0,
        };
        cmd.entity(children[next]).insert(NavSelected);
    }
}

fn handle_press(
    input: Query<&ActionState<Action>>,
    selected: Query<&dyn Navigable, With<NavSelected>>,
) {
    let Ok(input) = input.get_single() else { return };

    if input.just_pressed(&Action::Act) {
        let Ok(selected) = selected.get_single() else { return };
        for selected in &selected {
            selected.action();
        }
    }
}
