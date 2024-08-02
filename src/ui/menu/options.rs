use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MenuState::Options), init);
}

fn init(mut cmd: Commands) {
    cmd.ui_root()
        .with_children(|root| {
            root.button("Mappings").nav_state(MenuState::Mappings);
            root.button("Back").nav_state(MenuState::Main);
        })
        .nav_container()
        .insert(StateScoped(MenuState::Options));
}
