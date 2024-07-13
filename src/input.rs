//! Input module

use bevy::prelude::*;
pub use leafwing_input_manager::prelude::ActionState;
use leafwing_input_manager::prelude::*;

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
            .add_systems(Startup, init);

        #[cfg(feature = "menu")]
        app.add_systems(
            Update,
            handle_input.run_if(in_state(crate::GameState::Play)),
        );

        #[cfg(feature = "touch")]
        app.add_systems(
            PreUpdate,
            touch_system
                .chain()
                .after(bevy::input::InputSystem)
                .before(leafwing_input_manager::plugin::InputManagerSystem::Update),
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

    // If touch input is enabled, make the touch movement map to the move action
    #[cfg(feature = "touch")]
    input_map.insert(Action::Move, MouseMove::default());

    cmd.spawn(InputManagerBundle::with_map(input_map));
}

/// Read the input and perform actions
#[cfg(feature = "menu")]
fn handle_input(
    input: Query<&ActionState<Action>>,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    let Ok(input) = input.get_single() else {
        return;
    };

    if input.just_pressed(&Action::Pause) {
        next_state.set(crate::GameState::Menu)
    }
}

/// Touch inputs are converted to equivalent mouse values to make them
/// compatible with leafwing
#[cfg(feature = "touch")]
fn touch_system(
    window: Query<Entity, With<bevy::window::PrimaryWindow>>,
    touches: Res<Touches>,
    mut mouse_button_writer: EventWriter<bevy::input::mouse::MouseButtonInput>,
    mut cursor_moved_writer: EventWriter<CursorMoved>,
) {
    use bevy::input::{mouse::MouseButtonInput, ButtonState};

    let Ok(window) = window.get_single() else { return };

    for _ in touches.iter_just_pressed() {
        mouse_button_writer.send(MouseButtonInput {
            button: MouseButton::Left,
            state: ButtonState::Pressed,
            window,
        });
    }

    for _ in touches.iter_just_released() {
        mouse_button_writer.send(MouseButtonInput {
            button: MouseButton::Left,
            state: ButtonState::Released,
            window,
        });
    }

    // Doesn't support multitouch
    let Some(touch) = touches.iter().next() else {
        return;
    };

    cursor_moved_writer.send(CursorMoved {
        position: touch.position(),
        delta: Some(touch.delta()),
        window,
    });
}
