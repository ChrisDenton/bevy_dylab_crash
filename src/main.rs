use bevy::prelude::*;

fn main() -> AppExit {
    App::new().add_plugins(DefaultPlugins)
    .add_systems(Startup, spawn_camera)
    .run()
}

fn spawn_camera(_commands: Commands) {}
