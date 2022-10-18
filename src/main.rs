use bevy::{asset, input::mouse, prelude::*, scene, winit::WinitSettings};
mod title;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(title::setup_title)
        .add_system(title::button_system_title)
        .run();
}
