use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MenuState::Main), init);
}

fn init(mut cmd: Commands) {
    cmd.ui_root()
        .with_children(|root| {
            root.button("Play").nav_state(GameState::Play);
            root.button("Options").nav_state(MenuState::Options);
            #[cfg(not(target_arch = "wasm32"))]
            root.button("Exit")
                .nav(|mut app_exit_writer: EventWriter<AppExit>| {
                    app_exit_writer.send(AppExit::Success);
                });
        })
        .nav_container()
        .insert(StateScoped(MenuState::Main));
}
