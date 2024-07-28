use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MenuState::Main), init);
}

fn init(mut cmd: Commands) {
    cmd.ui_root()
        .with_children(|root| {
            root.button("hey");
            root.button("hi");
            root.button("hello");
        })
        .nav_container()
        .insert(StateScoped(MenuState::Main));
}
