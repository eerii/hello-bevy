//! Mappings menu screen.

// TODO: Show keymaps
// TODO: Remapping

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MenuState::Mappings), init);
}

fn init(mut cmd: Commands) {
    cmd.ui_root()
        .with_children(|root| {
            root.button("Back").nav_state(MenuState::Options);
        })
        .nav_container()
        .insert(StateScoped(MenuState::Mappings));
}
