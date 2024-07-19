//! Input module

use bevy::prelude::*;
pub use leafwing_input_manager::prelude::ActionState;
use leafwing_input_manager::prelude::*;

use crate::GameState;

// ······
// Plugin
// ······

/// Input
/// Uses the leafwing input manager for handling input
/// This allows mapping multiple sources to the same `Action`
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(
                OnEnter(if cfg!(feature = "menu") { GameState::Menu } else { GameState::Play }),
                init.run_if(run_once()),
            );

        #[cfg(feature = "menu")]
        app.add_systems(
            Update,
            handle_input.run_if(in_state(GameState::Play)),
        );
    }
}

// ··········
// Components
// ··········

/// These are all the possible actions that have an input mapping
/// CHANGE: Add player actions here and configure the default mappings in `init`
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    /// Button press usually assigned to Space or the A button in the gamepad
    Jump,
    /// Two axis input usually assigned to WASD or the left gamepad stick
    Move,
    /// Button press usually assigned to Escape or Start
    Pause,
}

// ·······
// Systems
// ·······

/// Create a new input manager for the general game
fn init(mut cmd: Commands) {
    let mut input_map = InputMap::default();
    input_map
        .insert(Action::Jump, KeyCode::Space)
        .insert(Action::Jump, GamepadButtonType::South)
        .insert(Action::Jump, MouseButton::Left)
        .insert(Action::Move, KeyboardVirtualDPad::WASD)
        .insert(Action::Move, GamepadStick::LEFT)
        .insert(Action::Pause, KeyCode::Escape)
        .insert(Action::Pause, GamepadButtonType::Start);

    cmd.spawn(InputManagerBundle::with_map(input_map));
}

/// Read the input and perform actions
#[cfg(feature = "menu")]
fn handle_input(
    input: Query<&ActionState<Action>>,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    let Ok(input) = input.get_single() else { return };

    if input.just_pressed(&Action::Pause) {
        next_state.set(crate::GameState::Menu)
    }
}
