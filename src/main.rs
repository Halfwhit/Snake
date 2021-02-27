use bevy::prelude::*;

fn main() {
    App::build()
        .add_startup_system(setup.system())
    .add_plugins(DefaultPlugins).run();
}

fn setup(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default());
}