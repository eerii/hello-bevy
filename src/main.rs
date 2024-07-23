use bevy::prelude::*;
use game::GamePlugin;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
