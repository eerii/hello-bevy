use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::GameState;

// ······
// Plugin
// ······

// Input
// Uses the leafwing input manager for handling input
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(OnEnter(GameState::Play), init)
            .add_systems(
                Update,
                handle_input.run_if(in_state(GameState::Play)),
            )
            .add_systems(OnExit(GameState::Play), stop);
    }
}

// ··········
// Components
// ··········

#[derive(Component)]
struct GameInput;

// These are all the possible actions that have an input mapping
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Action {
    Jump,
}

// ·······
// Systems
// ·······

// Create a new input manager if there are no others
fn init(mut cmd: Commands, input: Query<(), With<GameInput>>) {
    if input.iter().len() > 0 {
        return;
    }

    let input_map = InputMap::new([
        (Action::Jump, KeyCode::Space),
        (Action::Jump, KeyCode::KeyW),
    ]);

    cmd.spawn(InputManagerBundle::with_map(input_map))
        .insert(GameInput);
}

// Read the input and perform actions
fn handle_input(input: Query<&ActionState<Action>>) {
    let Ok(input) = input.get_single() else {
        return;
    };

    if input.just_pressed(&Action::Jump) {
        info!("Jump!");
    }
}

// Delete the input manager when exiting it's designed state
fn stop(mut cmd: Commands, input: Query<Entity, With<GameInput>>) {
    let Ok(input) = input.get_single() else {
        return;
    };

    cmd.entity(input).despawn();
}
