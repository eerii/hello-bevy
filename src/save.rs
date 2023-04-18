use bevy::prelude::*;
use bevy_pkv::PkvStore;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PkvStore::new("koala", "hello-bevy"));
    }
}

fn _load(mut _pkv: ResMut<PkvStore>) {
    /*if let Ok(user) = pkv.get::<String>("user") {
        info!("Hey {user}");
    } else {
        pkv.set_string("user", "koala").expect("Failed to save user");
    }*/
}

fn _save() {}
