use bevy::prelude::*;
use hello_bevy::GamePlugin;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
