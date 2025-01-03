use bevy::prelude::*;

#[derive(States, Default, Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum AppState {
    #[default]
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
