use bevy::prelude::*;
mod settings;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const FONT_SIZE: f32 = 40.0;
const MENU_BUTTON_MARGIN: f32 = 20.0;
const BASE_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const HIGH_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);

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
        .add_systems(OnEnter(AppState::Menu), spawn_main_menu)
        .add_systems(OnExit(AppState::Menu), despawn_entities_with_component::<OnMainMenuScreen>)
        .add_plugins(settings::settings_plugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component)]
struct OnMainMenuScreen;

fn despawn_entities_with_component<T: Component>(query: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_main_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(BASE_COLOR),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Main Menu"),
                        TextFont {
                            font_size: FONT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::all(Val::Px(MENU_BUTTON_MARGIN)),
                            ..default()
                        },
                    ));
                });
        });
}
