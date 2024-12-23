//! This module handles the input using `leafwing_input_manager`.

use leafwing_input_manager::prelude::*;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<Action>::default())
        .add_systems(OnEnter(GameState::Startup), init);
}

/// The prelude of this module.
pub mod prelude {
    pub use leafwing_input_manager::prelude::ActionState;

    pub use super::Action;
}

/// These are all the possible game actions that have an input mapping.
///
/// # Examples
///
/// ```
/// use game::prelude::*;
/// fn handle_input(input: Query<&ActionState<Action>>) {
///     let input = single!(input);
///     if input.just_pressed(&Action::Act) {
///         info!("Hi! c:");
///     }
/// }
/// ```
#[derive(Reflect, Std!)]
pub enum Action {
    /// Button press usually assigned to Space or the A button in the gamepad
    Act,
    /// Button press usually assigned to Escape or Start
    Pause,
    /// Dual axis input usually assigned to WASD or the left gamepad stick
    Move,
}

impl Actionlike for Action {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Action::Move => InputControlKind::DualAxis,
            _ => InputControlKind::Button,
        }
    }
}

impl Action {
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();
        input_map
            .insert(Action::Act, KeyCode::Space)
            .insert(Action::Act, KeyCode::Enter)
            .insert(Action::Act, GamepadButton::South)
            .insert(Action::Act, MouseButton::Left)
            .insert(Action::Pause, KeyCode::Escape)
            .insert(Action::Pause, GamepadButton::Start)
            .insert_dual_axis(Action::Move, VirtualDPad::wasd())
            .insert_dual_axis(Action::Move, VirtualDPad::arrow_keys())
            .insert_dual_axis(Action::Move, GamepadStick::LEFT);
        input_map
    }
}

/// Creates a new input manager
fn init(mut cmd: Commands) {
    cmd.spawn(InputManagerBundle::with_map(Action::default_input_map()));
}
